use crate::tasks::errors::TaskRepositoryError;
use crate::tasks::models::{CreateTaskItem, TaskId, TaskItem, UpdateTaskItem};

pub trait TaskRepository: Send + Sync {
    async fn create(&self, task: &CreateTaskItem) -> Result<TaskItem, TaskRepositoryError>;
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskItem>, TaskRepositoryError>;
    async fn list(&self) -> Result<Vec<TaskItem>, TaskRepositoryError>;
    async fn update(
        &self,
        id: &TaskId,
        task: &UpdateTaskItem,
    ) -> Result<TaskItem, TaskRepositoryError>;
    async fn delete(&self, id: &TaskId) -> Result<bool, TaskRepositoryError>;
    async fn set_completed(
        &self,
        id: &TaskId,
        completed: bool,
    ) -> Result<TaskItem, TaskRepositoryError>;
}
