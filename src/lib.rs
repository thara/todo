use std::fmt;
extern crate time;

pub mod parser;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug, PartialEq)]
pub enum Priority {
    High,
    Mid,
    Low,
}

impl Priority {
    pub fn from_string(s: &str) -> Option<Priority> {
        match s {
            "A" => Some(Priority::High),
            "B" => Some(Priority::Mid),
            "C" => Some(Priority::Low),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Priority::High => String::from("A"),
            Priority::Mid => String::from("B"),
            Priority::Low => String::from("C"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Project(String),
    Context(String),
}

impl Tag {
    pub fn project(s: &str) -> Tag {
        Tag::Project(String::from(s))
    }
    pub fn context(s: &str) -> Tag {
        Tag::Context(String::from(s))
    }
}

#[derive(Debug)]
pub struct Task {
    pub completed: bool,
    pub priority: Option<Priority>,
    pub completion_date: Option<String>,
    pub creation_date: Option<String>,
    pub description: String,
    pub tags: Vec<Tag>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.completed {
            write!(f, "x ")?;
        }

        if let Some(ref p) = self.priority {
            write!(f, "({}) ", p.to_string())?;
        }

        write!(f, "{}", self.description)
    }
}

pub struct TaskBuilder {
    completed: bool,
    priority: Option<Priority>,
    completion_date: Option<String>,
    creation_date: Option<String>,
    description: String,
    tags: Vec<Tag>,
}

impl TaskBuilder {
    pub fn new<S: Into<String>>(description: S) -> TaskBuilder {
        TaskBuilder {
            completed: false,
            priority: None,
            completion_date: None,
            creation_date: None,
            description: description.into(),
            tags: vec![],
        }
    }

    pub fn completed(mut self, completed: bool) -> TaskBuilder {
        self.completed = completed;
        self
    }

    pub fn priority(mut self, priority: Option<&str>) -> TaskBuilder {
        if let Some(s) = priority {
            let p = Priority::from_string(s);
            self.priority = p;
        }
        self
    }

    pub fn completion_date(mut self, date: &str) -> TaskBuilder {
        if time::strptime(date, "%Y-%m-%d").is_ok() {
            self.completion_date = Some(String::from(date));
        }
        self
    }

    pub fn creation_date(mut self, date: &str) -> TaskBuilder {
        if time::strptime(date, "%Y-%m-%d").is_ok() {
            self.creation_date = Some(String::from(date));
        }
        self
    }

    pub fn tag(mut self, tag: Tag) -> TaskBuilder {
        self.tags.push(tag);
        self
    }

    pub fn build(self) -> Task {
        Task {
            completed: self.completed,
            priority: self.priority,
            completion_date: self.completion_date,
            creation_date: self.creation_date,
            description: self.description,
            tags: self.tags,
        }
    }
}
