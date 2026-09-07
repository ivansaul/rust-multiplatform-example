use crate::tasks::errors::TaskServiceError as TasksServiceErrorCore;

#[boltffi::error]
#[derive(Debug, Clone)]
pub enum TaskError {
    NotFound,
    Unavailable,
    Unexpected,
}

impl From<TasksServiceErrorCore> for TaskError {
    fn from(value: TasksServiceErrorCore) -> Self {
        match value {
            TasksServiceErrorCore::Unavailable => TaskError::Unavailable,
            TasksServiceErrorCore::Unexpected { source: _ } => TaskError::Unexpected,
            TasksServiceErrorCore::NotFound => TaskError::NotFound,
        }
    }
}
