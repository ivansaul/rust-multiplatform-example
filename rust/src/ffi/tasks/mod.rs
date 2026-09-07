use std::sync::Arc;

use crate::{
    ffi::tasks::{
        error::TaskError,
        models::{CreateTaskItem, TaskItem, UpdateTaskItem},
    },
    tasks::{
        models::TaskId, sqlx_task_repository::SqlxTaskRepository,
        task_service::TaskService as CoreTaskService,
    },
};

pub mod error;
pub mod models;

#[derive(Clone)]
pub struct TaskService {
    inner: Arc<CoreTaskService<SqlxTaskRepository>>,
}

#[boltffi::export]
impl TaskService {
    pub(crate) fn new(inner: Arc<CoreTaskService<SqlxTaskRepository>>) -> Self {
        Self { inner }
    }

    #[rusty_macros::tokio_runtime]
    pub async fn create_task(&self, input: CreateTaskItem) -> Result<TaskItem, TaskError> {
        self.inner
            .create_task(input.into())
            .await
            .map(TaskItem::from)
            .map_err(TaskError::from)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn get_task(&self, id: String) -> Result<TaskItem, TaskError> {
        self.inner
            .get_task(id)
            .await
            .map(TaskItem::from)
            .map_err(TaskError::from)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn list_tasks(&self) -> Result<Vec<TaskItem>, TaskError> {
        let tasks: Vec<TaskItem> = self
            .inner
            .list_tasks()
            .await
            .map_err(TaskError::from)?
            .into_iter()
            .map(TaskItem::from)
            .collect::<Vec<_>>();
        Ok(tasks)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn update_task(
        &self,
        id: String,
        input: UpdateTaskItem,
    ) -> Result<TaskItem, TaskError> {
        self.inner
            .update_task(id, input.into())
            .await
            .map(TaskItem::from)
            .map_err(TaskError::from)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn delete_task(&self, id: String) -> Result<(), TaskError> {
        self.inner.delete_task(id).await.map_err(TaskError::from)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn complete_task(&self, id: String) -> Result<TaskItem, TaskError> {
        self.inner
            .complete_task(id)
            .await
            .map(TaskItem::from)
            .map_err(TaskError::from)
    }

    #[rusty_macros::tokio_runtime]
    pub async fn reopen_task(&self, id: String) -> Result<TaskItem, TaskError> {
        self.inner
            .reopen_task(id)
            .await
            .map(TaskItem::from)
            .map_err(TaskError::from)
    }
}
