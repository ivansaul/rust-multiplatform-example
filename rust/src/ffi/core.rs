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
