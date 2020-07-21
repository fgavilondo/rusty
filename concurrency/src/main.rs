use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

const NUMBER_OF_STUDENTS: u8 = 3;
const NUMBER_OF_CONCEPTS: u8 = 5;

// pub fn type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }

#[derive(Debug)]
struct ChatMessage {
    username: String,
    text: String,
    timestamp: DateTime<Utc>,
}

impl ChatMessage {
    fn new(username: &str, text: &str) -> Self {
        Self {
            username: String::from(username),
            text: String::from(text),
            timestamp: Utc::now(),
        }
    }
}

struct Chat {
    rx: Receiver<ChatMessage>,
}

impl Chat {
    fn handle_messages(&self) {
        let tid = thread::current().id();
        println!("{:?}: Chat is ready", tid);
        // receiver will block until messages arrive
        for msg in &self.rx {
            println!("{:?}: Received {:?}", tid, msg);
        }
    }
}

struct Student {
    name: String,
    tx: Sender<ChatMessage>,
}

impl Student {
    fn new(name: &str, tx: Sender<ChatMessage>) -> Self {
        Self {
            name: String::from(name),
            tx,
        }
    }

    fn active_listen(&self) {
        for i in 1..NUMBER_OF_CONCEPTS + 1 {
            let message = ChatMessage::new(&self.name, format!("Thing #{} understood", i).as_str());
            self.tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(1200));
        }
    }
}

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
        let tid = thread::current().id();
        for i in 1..NUMBER_OF_CONCEPTS + 1 {
            println!();
            println!("{:?}: {} says: Thing #{} that is great about Rust...", tid, self.name, i);
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn main() {
    println!();
    println!("{:?}: Start of training session", thread::current().id());

    let (tx, rx) = mpsc::channel();

    // Must use 'move' closure to use variable rx (declared in main thread) in the spawned thread.
    // Move closure transfers ownership of values from one thread to another.
    thread::spawn(move || {
        let chat = Chat {
            rx,
        };
        chat.handle_messages();
    });

    let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let handle = thread::spawn(|| {
        // Disclaimer: Any resemblance to real persons is purely coincidental!
        let presenter = Presenter::new("Mat");
        presenter.present();
    });
    thread_handles.push(handle);

    for idx in 0..NUMBER_OF_STUDENTS {
        let tx_clone = mpsc::Sender::clone(&tx);
        let handle = thread::spawn(move || {
            let student = Student::new(format!("{}{}", "Student_", idx).as_str(), tx_clone);
            student.active_listen();
        });
        thread_handles.push(handle);
    }

    // block the main thread until all threads have finished (except the chat, which never finishes)
    for h in thread_handles {
        h.join().unwrap();
    }

    println!();
    println!("{:?}: End of training session", thread::current().id());
}
