// <task> ::= [<completed>] ["(" <priority> ") "] [<completion_date>] [<creation_date>] <description> {<tag>}
// <completed> ::= "x "
// <priority> ::= "A" | "B" | "C"
// <completion_date> ::= <date> " "
// <creation_date> ::= <date> " "
// <tag> ::= " +" <project_name> | " @" <context_name>

use std::slice;
use std::str;

extern crate time;

fn task(s: &str) -> Option<Task> {
    let mut pos = 0;
    let (comp, pos) = completed(s, pos);
    let (pri, pos) = priority(s, pos);
    let (d1, pos) = completion_date(s, pos);
    let (d2, pos) = creation_date(s, pos);

    let desc = &s[pos..];
    let t = Task {
        completed: comp,
        priority: pri,
        completion_date: d1,
        creation_date: d2,
        description: String::from(desc),
    };
    Some(t)

    // while s[pos] == " " {
    //     let t, pos = tag(s, pos);
    // }
}
fn completed(s: &str, pos: usize) -> (bool, usize) {
    if s.chars().nth(pos) == Some('x') {
        (true, pos + 1)
    } else {
        (false, pos)
    }
}
fn priority(s: &str, pos: usize) -> (Option<Priority>, usize) {
    let mut chars = s.chars();
    if chars.nth(pos) == Some('(') && chars.nth(pos + 2) == Some(')') {
        let ch = chars.nth(pos + 1).unwrap();
        let p = match ch {
            'A' => Priority::High,
            'B' => Priority::Mid,
            _ => Priority::Low,
        };
        (Some(p), pos + 3)
    } else {
        (None, pos)
    }
}
fn date(s: &str, pos: usize) -> (Option<String>, usize) {
    let mut chars = s.chars();
    if chars.nth(pos + 11) == Some(' ') {
        if let Some(datestr) = s.get(pos..pos + 11) {
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
// fn tag(s: &str, pos: usize) -> (String, usize) {
// }

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

#[derive(Debug)]
struct Task {
    completed: bool,
    priority: Option<Priority>,
    completion_date: Option<String>,
    creation_date: Option<String>,
    description: String,
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
        assert_eq!(t.description,
                   "Review Tim's pull request +TodoTxtTouch @github");
        // assert_eq!(task.description.txt, "Review Tim's pull request ");
        // assert_eq!(task.description.contexts.length, 1);
        // assert_eq!(task.description.contexts[0], "TodoTxtTouch");
        // assert_eq!(task.description.projects.length, 1);
        // assert_eq!(task.description.projects[0], "github");
    }
}
