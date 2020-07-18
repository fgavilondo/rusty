use chrono::{DateTime, Utc};

#[derive(Debug)]
struct Participant {
    name: String,
    joined: DateTime<Utc>,
}

impl Participant {
    fn new(name: &str) -> Participant {
        Participant {
            name: String::from(name),
            joined: Utc::now(),
        }
    }

    fn say(&self, what: &str) -> ChatMessage {
        return ChatMessage::new(&self.name, what);
    }
}

#[derive(Debug)]
struct ChatMessage {
    name: String,
    text: String,
    timestamp: DateTime<Utc>,
}

impl ChatMessage {
    fn new(name: &str, text: &str) -> ChatMessage {
        ChatMessage {
            name: String::from(name),
            text: String::from(text),
            timestamp: Utc::now(),
        }
    }
}

fn main() {
    let steve = Participant::new("Steve");
    let message = steve.say("Hello!");
    println!("message: {:#?}", message);
}
