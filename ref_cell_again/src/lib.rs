#![allow(dead_code)]
use std::cell::RefCell;

struct Logger {
    log_entries: RefCell<Vec<String>>,
}
impl Logger {
    fn new() -> Logger {
        Logger { log_entries: RefCell::new(Vec::new()),
        }
    }

    fn add_entry(&self, message: &str) {
        self.log_entries.borrow_mut().push(message.to_string());
    }

    fn view_log(&self) {
        for entry in self.log_entries.borrow().iter() {
            println!("Log: {}", entry);
        }
    }
}
