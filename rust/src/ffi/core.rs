use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rusty_kv::KeyValue;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{
    database::Database,
    ffi::{
        app::AppContext, error::CoreError, settings::SettingsService as FfiSettingsService,
        tasks::TaskService as FfiTaskService,
    },
    settings::SettingsService as CoreSettingsService,
    tasks::{
        sqlx_task_repository::SqlxTaskRepository, task_service::TaskService as CoreTaskService,
    },
};

pub struct RustyCore {
    task_service: FfiTaskService,
    settings_service: FfiSettingsService,
}

struct Infrastructure {
    database: Database,
    key_value: KeyValue,
}

#[boltffi::export]
impl RustyCore {
    pub fn new(context: AppContext) -> Result<Self, CoreError> {
        let runtime = crate::runtime::runtime();

        let infrastructure = runtime.block_on(build_production_infrastructure(&context))?;

        Self::from_infrastructure(infrastructure)
    }

    pub fn preview() -> Result<Self, CoreError> {
        let runtime = crate::runtime::runtime();

        let infrastructure = runtime.block_on(build_preview_infrastructure())?;

        let core = Self::from_infrastructure(infrastructure)?;

        runtime.block_on(seed_preview_data(&core))?;

        Ok(core)
    }

    fn from_infrastructure(infrastructure: Infrastructure) -> Result<Self, CoreError> {
        let database = Arc::new(infrastructure.database);

        let task_repository = SqlxTaskRepository::new(database.clone());
        let task_service = CoreTaskService::new(task_repository);
        let task_service = FfiTaskService::new(Arc::new(task_service));

        let settings_service = CoreSettingsService::new(infrastructure.key_value);
        let settings_service = FfiSettingsService::new(Arc::new(settings_service));

        Ok(Self {
            task_service,
            settings_service,
        })
    }

    pub fn tasks(&self) -> FfiTaskService {
        self.task_service.clone()
    }

    pub fn settings(&self) -> FfiSettingsService {
        self.settings_service.clone()
    }
}

async fn build_production_infrastructure(
    context: &AppContext,
) -> Result<Infrastructure, CoreError> {
    let database_path = PathBuf::from(&context.storage.documents).join("app.db");
    let key_value_path = PathBuf::from(&context.storage.documents).join("kv.db");

    let database_pool = build_database_pool(&database_path).await?;

    run_database_migrations(&database_pool).await?;

    let database = Database::new(database_pool);

    let key_value = KeyValue::new(&key_value_path).map_err(|error| {
        eprintln!("KeyValue error: {error:?}");
        CoreError::Database
    })?;

    Ok(Infrastructure {
        database,
        key_value,
    })
}

async fn build_preview_infrastructure() -> Result<Infrastructure, CoreError> {
    let database_pool = build_in_memory_database_pool().await?;

    run_database_migrations(&database_pool).await?;

    let database = Database::new(database_pool);

    let key_value = KeyValue::in_memory().map_err(|error| {
        eprintln!("KeyValue error: {error:?}");
        CoreError::Database
    })?;

    Ok(Infrastructure {
        database,
        key_value,
    })
}

// Database

async fn build_database_pool(path: impl AsRef<Path>) -> Result<SqlitePool, CoreError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|error| {
            eprintln!("Database connection error: {error:?}");
            CoreError::Database
        })
}

async fn build_in_memory_database_pool() -> Result<SqlitePool, CoreError> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|error| {
            eprintln!("In-memory database connection error: {error:?}");
            CoreError::Database
        })
}

async fn run_database_migrations(pool: &SqlitePool) -> Result<(), CoreError> {
    sqlx::migrate!("src/database/migrations")
        .run(pool)
        .await
        .map_err(|error| {
            eprintln!("Database migration error: {error:?}");
            CoreError::Database
        })
}

// Preview

async fn seed_preview_data(core: &RustyCore) -> Result<(), CoreError> {
    use crate::ffi::tasks::models::CreateTaskItem;

    for index in 0..5 {
        let task = CreateTaskItem {
            title: format!("Task {index}"),
        };

        core.tasks().create_task(task).await.map_err(|error| {
            eprintln!("Preview seed error: {error:?}");
            CoreError::Internal
        })?;
    }

    Ok(())
}
