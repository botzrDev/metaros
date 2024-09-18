use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Process {
    id: u32,
    state: ProcessState,
}

pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct CoreSystem {
    processes: Arc<Mutex<VecDeque<Process>>>,
    current_process: Option<Process>,
    next_pid: u32,
}

impl CoreSystem {
    pub fn new() -> Self {
        CoreSystem {
            processes: Arc::new(Mutex::new(VecDeque::new())),
            current_process: None,
            next_pid: 1,
        }
    }

    pub fn create_process(&mut self) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        let process = Process {
            id: pid,
            state: ProcessState::Ready,
        };
        self.processes.lock().unwrap().push_back(process);
        pid
    }

    pub fn schedule(&mut self) {
        if let Some(mut current) = self.current_process.take() {
            current.state = ProcessState::Ready;
            self.processes.lock().unwrap().push_back(current);
        }
        
        if let Some(next) = self.processes.lock().unwrap().pop_front() {
            next.state = ProcessState::Running;
            self.current_process = Some(next);
        }
    }

    pub async fn run(&mut self) {
        println!("Core System running...");
        loop {
            self.schedule();
            // Simulate process execution
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}