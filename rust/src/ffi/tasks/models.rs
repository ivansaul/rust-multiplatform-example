use crate::tasks::models::{
    CreateTaskItem as CreateTaskItemCore, TaskItem as TaskItemCore,
    UpdateTaskItem as UpdateTaskItemCore,
};

#[boltffi::data]
#[derive(Clone)]
pub struct TaskItem {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl From<TaskItem> for TaskItemCore {
    fn from(value: TaskItem) -> Self {
        Self {
            id: value.id,
            title: value.title,
            completed: value.completed,
        }
    }
}

impl From<TaskItemCore> for TaskItem {
    fn from(value: TaskItemCore) -> Self {
        Self {
            id: value.id,
            title: value.title,
            completed: value.completed,
        }
    }
}

// CreateTaskItem

#[boltffi::data]
#[derive(Clone)]
pub struct CreateTaskItem {
    pub title: String,
}

impl From<CreateTaskItem> for CreateTaskItemCore {
    fn from(value: CreateTaskItem) -> Self {
        Self { title: value.title }
    }
}

impl From<CreateTaskItemCore> for CreateTaskItem {
    fn from(value: CreateTaskItemCore) -> Self {
        Self { title: value.title }
    }
}

// UpdateTaskItem

#[boltffi::data]
#[derive(Clone)]
pub struct UpdateTaskItem {
    pub title: String,
}

impl From<UpdateTaskItem> for UpdateTaskItemCore {
    fn from(value: UpdateTaskItem) -> Self {
        Self { title: value.title }
    }
}

impl From<UpdateTaskItemCore> for UpdateTaskItem {
    fn from(value: UpdateTaskItemCore) -> Self {
        Self { title: value.title }
    }
}
