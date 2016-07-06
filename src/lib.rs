pub enum TaskStatus {
    TODO,
    DONE,
}

pub struct Task {
    pub name: String,
    pub status: TaskStatus,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            name: name,
            status: TaskStatus::TODO,
        }
    }

    fn check(&mut self) {
        self.status = TaskStatus::DONE;
    }

    fn uncheck(&mut self) {
        self.status = TaskStatus::TODO;
    }
}

pub struct TaskList {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(name: String) -> TaskList {
        TaskList {
            name: name,
            tasks: vec![],
        }
    }

    pub fn add(&mut self, name: String) {
        self.tasks.push(Task::new(name));
    }

    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn toggle(&mut self, index: usize) {
        let mut task = self.tasks.get_mut(index).unwrap();
        match task.status {
            TaskStatus::TODO => {
                task.check();
            },
            TaskStatus::DONE => {
                task.uncheck();
            }
        }
    }
}