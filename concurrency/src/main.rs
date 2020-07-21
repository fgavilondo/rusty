use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

const NUMBER_OF_STUDENTS: u8 = 3;
const NUMBER_OF_CONCEPTS: u8 = 10;

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[derive(Debug)]
struct ChatMessage {
    name: String,
    text: String,
    timestamp: DateTime<Utc>,
}

impl ChatMessage {
    fn new(name: &str, text: &str) -> Self {
        Self {
            name: String::from(name),
            text: String::from(text),
            timestamp: Utc::now(),
        }
    }
}

trait ChatParticipant {
    fn name(&self) -> &str;
    fn chat(&self, what: &str) -> ChatMessage {
        return ChatMessage::new(&self.name(), what);
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    joined: DateTime<Utc>,
}

impl Student {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            joined: Utc::now(),
        }
    }

    fn chat_away(&self) {
        for i in 1..NUMBER_OF_CONCEPTS + 1 {
            let message = self.chat(format!("Thing #{} understood", i).as_str());
            println!("{:?}", message);
            thread::sleep(Duration::from_millis(2200));
        }
    }
}

impl ChatParticipant for Student {
    fn name(&self) -> &str {
        return self.name.as_str();
    }
}

#[derive(Debug)]
struct Presenter {
    name: String,
}

impl Presenter {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    fn present(&self) {
        for i in 1..NUMBER_OF_CONCEPTS + 1 {
            println!("Thing #{} that is great about Rust...", i);
            thread::sleep(Duration::from_millis(2000));
        }
    }
}

impl ChatParticipant for Presenter {
    fn name(&self) -> &str {
        return self.name.as_str();
    }
}

fn main() {
    let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let handle = thread::spawn(|| {
        let mat = Presenter::new("Mat");
        mat.present();
    });
    thread_handles.push(handle);

    for i in 1..NUMBER_OF_STUDENTS + 1 {
        // use 'move' to take ownership of 'i' inside closure
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let student = Student::new(format!("{}{}", "Student_", i).as_str());
            student.chat_away();
        });
        thread_handles.push(handle);
    }

    // wait for all threads to finish
    for h in thread_handles {
        h.join().unwrap();
    }
}
