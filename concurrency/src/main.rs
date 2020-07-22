use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;

const NUMBER_OF_STUDENTS: u8 = 3;
const NUMBER_OF_CONCEPTS: u8 = 6;

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

struct ChatScreen {
    rx: Receiver<ChatMessage>,
}

impl ChatScreen {
    fn handle_messages(&self) {
        let tid = thread::current().id();
        println!("{:?}: Chat is ready", tid);
        // receiver will block until messages arrive
        for msg in &self.rx {
            println!("{:?} - ChatScreen received: {:?}", tid, msg);
        }
    }
}

lazy_static! {
    static ref SPEAK_MUTEX: Mutex<i32> = Mutex::new(0i32);
}

fn say_stuff(stuff: &str, name: &str) {
    let _lock = SPEAK_MUTEX.lock().unwrap();
    println!("{:?} - {} says: {}", thread::current().id(), name, stuff);
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
        for i in 1..NUMBER_OF_CONCEPTS + 1 {
            say_stuff(format!("Let me tell you about thing #{} that is amazing about Rust...", i).as_str(),
                      self.name.as_str());
            thread::sleep(Duration::from_millis(1500));
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
            if i % 3 == 0 {
                say_stuff(format!("Amazing thing #{} sounds tricky :-(", i).as_str(), self.name.as_str());
            } else {
                let message = ChatMessage::new(&self.name,
                                               format!("Amazing thing #{} is cool!", i).as_str());
                self.tx.send(message).unwrap();
            }
            thread::sleep(Duration::from_millis(1700));
        }
    }
}

fn main() {
    println!();
    println!("{:?}: Start of training session", thread::current().id());

    let (tx, rx) = mpsc::channel();
    let mut thread_handles = vec![];

    thread::spawn(|| {
        let chat = ChatScreen {
            rx,
        };
        chat.handle_messages();
    });

    let handle = thread::spawn(|| {
        // Disclaimer: Any resemblance to real persons is purely coincidental!
        let presenter = Presenter::new("Mat");
        presenter.present();
    });
    thread_handles.push(handle);

    for idx in 0..NUMBER_OF_STUDENTS {
        let tx_clone = mpsc::Sender::clone(&tx);
        // A move closure transfers ownership of values from one thread to another.
        // Needed to be able to use variable idx (declared in main thread) in each spawned thread.
        let handle = thread::spawn(move || {
            let student = Student::new(format!("{}{}", "Student_", idx + 1).as_str(), tx_clone);
            student.active_listen();
        });
        thread_handles.push(handle);
    }

    // block the main thread until all threads have finished (except the chat, which never finishes)
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!();
    println!("{:?}: End of training session", thread::current().id());
}
