// <task> ::= [<completed>] ["(" <priority> ") "] [<completion_date>] [<creation_date>] <description> {<tag>}
// <completed> ::= "x "
// <priority> ::= "A" | "B" | "C"
// <completion_date> ::= <date> " "
// <creation_date> ::= <date> " "
// <tag> ::= " +" <project_name> | " @" <context_name>

use std::str;
use std::slice;

extern crate time;

fn task(s: &str) -> Option<Task> {
    let pos = 0;
    let (comp, pos) = completed(s, pos);
    let (pri, pos) = priority(s, pos);
    let (d1, pos) = completion_date(s, pos);
    let (d2, pos) = creation_date(s, pos);
    let (desc, pos) = description(s, pos);

    let mut tags = Vec::new();
    let mut pos = pos;
    while pos < s.len() {
        let t = tag(s, pos);
        pos = t.1;
        if let Some(t) = t.0 {
            tags.push(t);
        }
    }

    let t = Task {
        completed: comp,
        priority: pri,
        completion_date: d1,
        creation_date: d2,
        description: desc,
        tags: tags,
    };
    Some(t)
}
fn completed(s: &str, pos: usize) -> (bool, usize) {
    if s.get(pos..pos+1) == Some("x") {
        (true, pos + 2)
    } else {
        (false, pos)
    }
}
fn priority(s: &str, pos: usize) -> (Option<Priority>, usize) {
    if s.get(pos..pos+1) == Some(&"(") && s.get(pos+2..pos+3) == Some(&")") {
        let p = match s.get(pos+1..pos+2).unwrap() {
            "A" => Priority::High,
            "B" => Priority::Mid,
            _ => Priority::Low,
        };
        (Some(p), pos + 4)
    } else {
        (None, pos)
    }
}
fn date(s: &str, pos: usize) -> (Option<String>, usize) {
    if s.get(pos+10..pos+11) == Some(" ") {
        if let Some(datestr) = s.get(pos..pos + 10) {
            match time::strptime(datestr, "%Y-%m-%d") {
                Ok(_) => (Some(String::from(datestr)), pos + 11),
                _ => (None, pos),
            }
        } else {
            (None, pos)
        }
    } else {
        (None, pos)
    }
}

fn completion_date(s: &str, pos: usize) -> (Option<String>, usize) {
    date(s, pos)
}

fn creation_date(s: &str, pos: usize) -> (Option<String>, usize) {
    date(s, pos)
}

fn description(s: &str, pos: usize) -> (String, usize) {
    let ref ls = s[pos..];
    if let Some(i) = ls.find(" +").or(ls.find(" @")) {
        let ref xs = s[pos..pos+i];
        (String::from(xs), pos+i)
    } else {
        (String::from(ls), s.len())
    }
}
fn tag(s: &str, pos: usize) -> (Option<Tag>, usize) {
    let ref ls = s[pos..];
    if let Some(i) = ls.find(" +").or(ls.find(" @")) {
        let ref ys = ls[i+1..];
        let end = ys.find(" ").unwrap_or(ys.len());
        let ref name = s[pos+2..pos+end+1];
        let tag = match &s[pos..pos+2] {
            " +" => Tag::Project(String::from(name)),
            " @" => Tag::Context(String::from(name)),
            _ => Tag::Project(String::from(name)),
        };
        (Some(tag), pos+end+1)
    } else {
        (None, s.len())
    }
}

struct Parser<T>(pub fn(String) -> Vec<(T, String)>);

// impl Parser<T> {
//     fn parse(&self, s: String) -> Option<T> {
//         let Parser(f) = self;
//         f(s);
//     }
// }

#[derive(Debug,PartialEq)]
enum Priority {
    High,
    Mid,
    Low,
}

#[derive(Debug, PartialEq)]
enum Tag {
    Project(String),
    Context(String),
}

impl Tag {
    fn project(s: &str) -> Tag {
        Tag::Project(String::from(s))
    }
    fn context(s: &str) -> Tag {
        Tag::Context(String::from(s))
    }
}

#[derive(Debug)]
struct Task {
    completed: bool,
    priority: Option<Priority>,
    completion_date: Option<String>,
    creation_date: Option<String>,
    description: String,
    tags: Vec<Tag>,
}

impl Task {
    fn parse_line(line: &str) {
        let mut chars = line.chars();

        for ch in chars {
            println!("XXXXX {}", ch);
        }
    }
}

#[cfg(test)]
mod tests {
    use Task;
    use Parser;
    use Priority;
    use task;
    use Tag;
    #[test]
    fn it_parse_todo_txt_format() {

        let item = Parser::<String>(|cs| match cs.as_ref() {
            "" => Vec::new(),
            _ => {
                let (head, tail) = cs.split_at(1);
                vec![(String::from(head), String::from(tail))]
            }
        });

        let line = "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github";
        let t = task(line).unwrap();
        println!("{:?}", t);

        assert_eq!(t.completed, true);
        assert_eq!(t.priority.unwrap(), Priority::High);
        assert_eq!(t.completion_date.unwrap(), "2011-03-02".to_string());
        assert_eq!(t.creation_date.unwrap(), "2011-03-01".to_string());
        assert_eq!(t.description, "Review Tim's pull request");
        // assert_eq!(task.description.txt, "Review Tim's pull request ");
        assert_eq!(t.tags.len(), 2);
        assert_eq!(t.tags[0], Tag::project("TodoTxtTouch"));
        assert_eq!(t.tags[1], Tag::context("github"));
        // assert_eq!(task.description.contexts[0], "TodoTxtTouch");
        // assert_eq!(task.description.projects.length, 1);
        // assert_eq!(task.description.projects[0], "github");
    }
}
