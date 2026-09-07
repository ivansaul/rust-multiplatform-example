use crate::tasks::{
    errors::{TaskRepositoryError, TaskServiceError},
    models::{CreateTaskItem, TaskId, TaskItem, UpdateTaskItem},
    task_repository::TaskRepository,
};

pub struct TaskService<R>
where
    R: TaskRepository,
{
    repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: TaskRepository> TaskService<R> {
    pub async fn create_task(&self, task: CreateTaskItem) -> Result<TaskItem, TaskServiceError> {
        self.repository.create(&task).await.map_err(map_repo_error)
    }

    pub async fn get_task(&self, id: TaskId) -> Result<TaskItem, TaskServiceError> {
        self.repository
            .find_by_id(&id)
            .await
            .map_err(map_repo_error)?
            .ok_or(TaskServiceError::NotFound)
    }

    pub async fn list_tasks(&self) -> Result<Vec<TaskItem>, TaskServiceError> {
        self.repository.list().await.map_err(map_repo_error)
    }

    pub async fn update_task(
        &self,
        id: TaskId,
        task: UpdateTaskItem,
    ) -> Result<TaskItem, TaskServiceError> {
        self.repository
            .update(&id, &task)
            .await
            .map_err(map_repo_error)
    }

    pub async fn delete_task(&self, id: TaskId) -> Result<(), TaskServiceError> {
        let deleted = self.repository.delete(&id).await.map_err(map_repo_error)?;
        if !deleted {
            return Err(TaskServiceError::NotFound);
        }
        Ok(())
    }

    pub async fn complete_task(&self, id: TaskId) -> Result<TaskItem, TaskServiceError> {
        self.repository
            .set_completed(&id, true)
            .await
            .map_err(map_repo_error)
    }

    pub async fn reopen_task(&self, id: TaskId) -> Result<TaskItem, TaskServiceError> {
        self.repository
            .set_completed(&id, false)
            .await
            .map_err(map_repo_error)
    }
}

fn map_repo_error(err: TaskRepositoryError) -> TaskServiceError {
    match err {
        TaskRepositoryError::Unavailable => TaskServiceError::Unavailable,
        TaskRepositoryError::Internal { source } => TaskServiceError::Unexpected { source },
    }
}
