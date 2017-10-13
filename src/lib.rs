use std::fmt;

pub mod parser;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug, PartialEq)]
pub enum Priority {
    High,
    Mid,
    Low,
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
            //write!(f, "{}", Red.paint("x "))?;
            write!(f, "x ")?;
        }

        if self.priority.is_some() {
            let mark = match self.priority {
                Some(Priority::High) => "A",
                Some(Priority::Mid) => "B",
                Some(Priority::Low) => "C",
                _ => panic!("Unexpected invalid priority"),
            };
            write!(f, "({}) ", mark)?;
        }

        write!(f, "{}", self.description)
    }
}
