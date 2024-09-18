use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

pub struct Message {
    pub sender: u32,
    pub content: String,
}

pub struct IPC {
    mailboxes: Arc<Mutex<HashMap<u32, VecDeque<Message>>>>,
}

impl IPC {
    pub fn new() -> Self {
        IPC {
            mailboxes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn send_message(&self, sender: u32, recipient: u32, content: String) {
        let message = Message { sender, content };
        self.mailboxes
            .lock()
            .unwrap()
            .entry(recipient)
            .or_insert_with(VecDeque::new)
            .push_back(message);
    }

    pub fn receive_message(&self, recipient: u32) -> Option<Message> {
        self.mailboxes
            .lock()
            .unwrap()
            .get_mut(&recipient)
            .and_then(|mailbox| mailbox.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc() {
        let ipc = IPC::new();

        ipc.send_message(1, 2, "Hello".to_string());
        ipc.send_message(3, 2, "World".to_string());

        let msg1 = ipc.receive_message(2).unwrap();
        assert_eq!(msg1.sender, 1);
        assert_eq!(msg1.content, "Hello");

        let msg2 = ipc.receive_message(2).unwrap();
        assert_eq!(msg2.sender, 3);
        assert_eq!(msg2.content, "World");

        assert!(ipc.receive_message(2).is_none());
    }
}