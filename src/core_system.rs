use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error;

// Custom error type for CoreSystem
#[derive(Debug)]
pub enum CoreSystemError {
    LockError,
    ProcessCreationError,
    SchedulingError,
}

impl fmt::Display for CoreSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreSystemError::LockError => write!(f, "Failed to acquire lock"),
            CoreSystemError::ProcessCreationError => write!(f, "Failed to create process"),
            CoreSystemError::SchedulingError => write!(f, "Error during scheduling"),
        }
    }
}

impl Error for CoreSystemError {}

pub struct Process {
    id: u32,
    state: ProcessState,
}

#[derive(Debug, PartialEq)]
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
        log("Creating new CoreSystem");
        CoreSystem {
            processes: Arc::new(Mutex::new(VecDeque::new())),
            current_process: None,
            next_pid: 1,
        }
    }

    pub fn create_process(&mut self) -> Result<u32, CoreSystemError> {
        let pid = self.next_pid;
        self.next_pid += 1;
        let process = Process {
            id: pid,
            state: ProcessState::Ready,
        };
        
        match self.processes.lock() {
            Ok(mut processes) => {
                processes.push_back(process);
                log(&format!("Created new process with PID: {}", pid));
                Ok(pid)
            },
            Err(_) => {
                log_error("Failed to acquire lock while creating process");
                Err(CoreSystemError::LockError)
            }
        }
    }

    pub fn schedule(&mut self) -> Result<(), CoreSystemError> {
        if let Some(mut current) = self.current_process.take() {
            current.state = ProcessState::Ready;
            match self.processes.lock() {
                Ok(mut processes) => processes.push_back(current),
                Err(_) => {
                    log_error("Failed to acquire lock while scheduling (push_back)");
                    return Err(CoreSystemError::LockError);
                }
            }
        }
        
        match self.processes.lock() {
            Ok(mut processes) => {
                if let Some(next) = processes.pop_front() {
                    next.state = ProcessState::Running;
                    self.current_process = Some(next);
                    log(&format!("Scheduled process with PID: {}", self.current_process.as_ref().unwrap().id));
                    Ok(())
                } else {
                    log("No processes to schedule");
                    Ok(())
                }
            },
            Err(_) => {
                log_error("Failed to acquire lock while scheduling (pop_front)");
                Err(CoreSystemError::LockError)
            }
        }
    }

    pub async fn run(&mut self) {
        log("Core System running...");
        loop {
            match self.schedule() {
                Ok(_) => {
                    // Simulate process execution
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                },
                Err(e) => {
                    log_error(&format!("Error during scheduling: {}", e));
                    // In a real system, you might want to implement some error recovery here
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}

// Simple logging functions
fn log(message: &str) {
    println!("[INFO] {}", message);
}

fn log_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_process() {
        let mut core_system = CoreSystem::new();
        let result = core_system.create_process();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        let result = core_system.create_process();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_schedule() {
        let mut core_system = CoreSystem::new();
        
        // Create two processes
        let _ = core_system.create_process();
        let _ = core_system.create_process();

        // First schedule should set the current process
        let result = core_system.schedule();
        assert!(result.is_ok());
        assert!(core_system.current_process.is_some());
        assert_eq!(core_system.current_process.as_ref().unwrap().id, 1);
        assert_eq!(core_system.current_process.as_ref().unwrap().state, ProcessState::Running);

        // Second schedule should switch to the next process
        let result = core_system.schedule();
        assert!(result.is_ok());
        assert!(core_system.current_process.is_some());
        assert_eq!(core_system.current_process.as_ref().unwrap().id, 2);
        assert_eq!(core_system.current_process.as_ref().unwrap().state, ProcessState::Running);
    }

    #[test]
    fn test_schedule_empty() {
        let mut core_system = CoreSystem::new();
        
        // Scheduling with no processes should not error
        let result = core_system.schedule();
        assert!(result.is_ok());
        assert!(core_system.current_process.is_none());
    }
}