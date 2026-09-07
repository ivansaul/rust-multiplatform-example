use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{
    database::Database,
    ffi::{app::AppContext, error::CoreError, tasks::TaskService},
    tasks::{
        sqlx_task_repository::SqlxTaskRepository, task_service::TaskService as TasksServiceCore,
    },
};

pub struct RustyCore {
    tasks_service: TaskService,
}

#[boltffi::export]
impl RustyCore {
    pub fn new(context: AppContext) -> Result<Self, CoreError> {
        let mut db_path = PathBuf::from(&context.storage.documents);
        db_path.push("app.db");

        let runtime = crate::runtime::runtime();
        let pool = runtime.block_on(build_pool(&db_path))?;
        runtime.block_on(sqlx_migrate(&pool))?;
        Self::from_pool(pool)
    }

    pub fn preview() -> Result<Self, CoreError> {
        let runtime = crate::runtime::runtime();
        let pool = runtime.block_on(build_memory_pool())?;
        runtime.block_on(sqlx_migrate(&pool))?;
        let core = Self::from_pool(pool)?;
        runtime.block_on(seed_preview_data(&core))?;
        Ok(core)
    }

    fn from_pool(pool: SqlitePool) -> Result<Self, CoreError> {
        let database = Database::new(pool);
        let repository = SqlxTaskRepository::new(Arc::new(database));
        let service_core = TasksServiceCore::new(repository);
        let tasks_service = TaskService::new(Arc::new(service_core));

        Ok(Self { tasks_service })
    }

    pub fn tasks(&self) -> TaskService {
        self.tasks_service.clone()
    }
}

async fn build_pool(path: impl AsRef<Path>) -> Result<SqlitePool, CoreError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| {
            eprintln!("SQLx ERROR: {e:?}");
            CoreError::Database
        })
}

async fn sqlx_migrate(pool: &SqlitePool) -> Result<(), CoreError> {
    sqlx::migrate!("src/database/migrations")
        .run(pool)
        .await
        .map_err(|e| {
            eprintln!("SQLx MIGRATION ERROR: {e:?}");
            CoreError::Database
        })
}

// Preview

async fn build_memory_pool() -> Result<SqlitePool, CoreError> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|e| {
            eprintln!("SQLx ERROR: {e:?}");
            CoreError::Database
        })
}

async fn seed_preview_data(core: &RustyCore) -> Result<(), CoreError> {
    use crate::ffi::tasks::models::CreateTaskItem;
    for i in 0..5 {
        let task = CreateTaskItem {
            title: format!("Task {}", i),
        };
        core.tasks().create_task(task).await.map_err(|e| {
            eprintln!("SEED ERROR: {e:?}");
            CoreError::Internal
        })?;
    }
    Ok(())
}
