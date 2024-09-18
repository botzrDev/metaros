use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub u8);

pub struct Task {
    pub id: u32,
    pub priority: Priority,
    pub deadline: Instant,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
            .then_with(|| self.deadline.cmp(&other.deadline))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

pub struct Scheduler {
    tasks: BinaryHeap<Reverse<Task>>,
    next_task_id: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
            next_task_id: 1,
        }
    }

    pub fn add_task(&mut self, priority: u8, deadline: Duration) -> u32 {
        let task_id = self.next_task_id;
        self.next_task_id += 1;

        let task = Task {
            id: task_id,
            priority: Priority(priority),
            deadline: Instant::now() + deadline,
        };

        self.tasks.push(Reverse(task));
        task_id
    }

    pub fn get_next_task(&mut self) -> Option<Task> {
        self.tasks.pop().map(|Reverse(task)| task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler() {
        let mut scheduler = Scheduler::new();

        let task1 = scheduler.add_task(1, Duration::from_secs(10));
        let task2 = scheduler.add_task(2, Duration::from_secs(5));
        let task3 = scheduler.add_task(1, Duration::from_secs(1));

        assert_eq!(scheduler.get_next_task().unwrap().id, task2);
        assert_eq!(scheduler.get_next_task().unwrap().id, task3);
        assert_eq!(scheduler.get_next_task().unwrap().id, task1);
        assert!(scheduler.get_next_task().is_none());
    }
}