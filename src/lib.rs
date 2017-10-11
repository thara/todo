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
    let (desc, pos) = description(s, pos);

    if let desc = s[pos..] {
        let t = Task {
            completed: comp,
            priority: pri,
            completion_date: d1,
            creation_date: d2,
            description: desc,
        };
        Some(t)
    } else {
        None
    }

    // while s[pos] == " " {
    //     let t, pos = tag(s, pos);
    // }
}
fn completed(s: &str, pos: usize) -> (bool, usize) {
    if s[pos] == "x" {
        (true, pos + 1)
    } else {
        (false, pos)
    }
}
fn priority(s: &str, pos: usize) -> (Option<Priority>, usize) {
    if s[pos] == "(" && s[pos + 2] == ")" {
        let p = match s[pos + 1] {
            "A" => High,
            "B" => Mid,
            _ => Low,
        };
        (Some(p), pos + 3)
    } else {
        (None, pos)
    }
}
fn date(s: &str, pos: usize) -> (Option<String>, usize) {
    if s[pos + 11] == " " {
        if let Some(datestr) = s.get(pos..pos+11) {
            match time::strptime(datestr, "%Y-%m-%d") {
                Ok(_) => (Some(String::from(datestr)), pos+11),
                _ => (None, pos)
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

enum Priority {High, Mid, Low}

struct Task {
    completed: bool,
    priority: Option<Priority>,
    completion_date: Option<String>,
    creation_date: Option<String>,
    description: String
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
        Task::parse_line(line);
        // let task = Task::parse_line(line);
        // assert_eq!(task.completed, true);
        // assert_eq!(task.priority, Priority::High);
        // assert_eq!(task.completion_date, "2011-03-02");
        // assert_eq!(task.creation_date, "2011-03-01");
        // assert_eq!(task.description, "Review Tim's pull request +TodoTxtTouch @github");
        // assert_eq!(task.description.txt, "Review Tim's pull request ");
        // assert_eq!(task.description.contexts.length, 1);
        // assert_eq!(task.description.contexts[0], "TodoTxtTouch");
        // assert_eq!(task.description.projects.length, 1);
        // assert_eq!(task.description.projects[0], "github");
    }
}
