use std::sync::Arc;

use crate::{
    database::Database,
    tasks::{
        errors::TaskRepositoryError,
        models::{CreateTaskItem, TaskId, TaskItem, UpdateTaskItem},
        task_repository::TaskRepository,
    },
};

pub struct SqlxTaskRepository {
    database: Arc<Database>,
}

impl SqlxTaskRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

impl TaskRepository for SqlxTaskRepository {
    async fn create(&self, task: &CreateTaskItem) -> Result<TaskItem, TaskRepositoryError> {
        let task_id = uuid::Uuid::now_v7().to_string();
        let task = sqlx::query_as!(
            TaskItem,
            r#"
            INSERT INTO tasks (
                id,
                title
            ) VALUES (?, ?)
            RETURNING
                id,
                title,
                completed
            "#,
            task_id,
            task.title,
        )
        .fetch_one(&self.database.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(task)
    }

    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskItem>, TaskRepositoryError> {
        let task = sqlx::query_as!(
            TaskItem,
            r#"
            SELECT
                id,
                title,
                completed
            FROM tasks
            WHERE id = ?
            "#,
            id,
        )
        .fetch_optional(&self.database.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(task)
    }

    async fn list(&self) -> Result<Vec<TaskItem>, TaskRepositoryError> {
        sqlx::query_as!(
            TaskItem,
            r#"SELECT
                id,
                title,
                completed
            FROM tasks
            "#
        )
        .fetch_all(&self.database.pool)
        .await
        .map_err(map_sqlx_error)
    }

    async fn update(
        &self,
        id: &TaskId,
        task: &UpdateTaskItem,
    ) -> Result<TaskItem, TaskRepositoryError> {
        let task = sqlx::query_as!(
            TaskItem,
            r#"
            UPDATE tasks
            SET
                title = ?
            WHERE id = ?
            RETURNING
                id,
                title,
                completed
            "#,
            task.title,
            id
        )
        .fetch_one(&self.database.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(task)
    }

    async fn delete(&self, id: &TaskId) -> Result<bool, TaskRepositoryError> {
        let res = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.database.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(res.rows_affected() > 0)
    }
    async fn set_completed(
        &self,
        id: &TaskId,
        completed: bool,
    ) -> Result<TaskItem, TaskRepositoryError> {
        let task = sqlx::query_as!(
            TaskItem,
            r#"
            UPDATE tasks
            SET
                completed = ?
            WHERE id = ?
            RETURNING
                id,
                title,
                completed
            "#,
            completed,
            id
        )
        .fetch_one(&self.database.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(task)
    }
}

fn map_sqlx_error(err: sqlx::Error) -> TaskRepositoryError {
    match err {
        sqlx::Error::PoolTimedOut => TaskRepositoryError::Unavailable,
        _ => TaskRepositoryError::Internal {
            source: Box::new(err),
        },
    }
}
