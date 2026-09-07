pub type TaskId = String;

#[derive(Debug)]
pub struct TaskItem {
    pub id: TaskId,
    pub title: String,
    pub completed: bool,
}

impl TaskItem {
    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn reopen(&mut self) {
        self.completed = false;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

// DTOs

#[derive(Debug)]
pub struct CreateTaskItem {
    pub title: String,
}

#[derive(Debug)]
pub struct UpdateTaskItem {
    pub title: String,
}
