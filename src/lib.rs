// <task> ::= [<completed>] ["(" <priority> ") "] [<completion_date>] [<creation_date>] <description> {<tag>}
// <completed> ::= "x "
// <priority> ::= "A" | "B" | "C"
// <completion_date> ::= <date> " "
// <creation_date> ::= <date> " "
// <tag> ::= " +" <project_name> | " @" <context_name>

use std::str;

extern crate time;

fn task(s: &str) -> Option<Task> {
    let mut pos = 0;
    let comp = completed(s, &mut pos);
    let pri = priority(s, &mut pos);
    let d1 = completion_date(s, &mut pos);
    let d2 = creation_date(s, &mut pos);
    let desc = description(s, &mut pos);

    let mut tags = Vec::new();
    while pos < s.len() {
        if let Some(t) = tag(s, &mut pos) {
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

fn completed(s: &str, pos: &mut usize) -> bool {
    match s.get(*pos..*pos+1) {
        Some("x") => {
            *pos = *pos + 2;
            true
        },
        _ => false
    }
}

fn priority(s: &str, pos: &mut usize) -> Option<Priority> {
    let v = s.get(*pos..*pos+3);
    let mut f = |p| {
        *pos += 4;
        Some(p)
    };
    match v {
        Some("(A)") => f(Priority::High),
        Some("(B)") => f(Priority::Mid),
        Some("(C)") => f(Priority::Low),
        _ => None
    }
}
fn date(s: &str, pos: &mut usize) -> Option<String> {
    match s.get(*pos..*pos + 11) {
        Some(v) if time::strptime(v, "%Y-%m-%d ").is_ok() => {
            *pos = *pos + 11;
            let datestr = &v[..10];
            Some(String::from(datestr))
        },
        _ => None
    }
}

fn completion_date(s: &str, pos: &mut usize) -> Option<String> {
    date(s, pos)
}

fn creation_date(s: &str, pos: &mut usize) -> Option<String> {
    date(s, pos)
}

fn description(s: &str, pos: &mut usize) -> String {
    let ref ls = s[*pos..];
    if let Some(i) = ls.find(" +").or(ls.find(" @")) {
        let ref xs = s[*pos..*pos+i];
        *pos = *pos + i;
        String::from(xs.trim())
    } else {
        *pos = s.len();
        String::from(ls.trim())
    }
}
fn tag(s: &str, pos: &mut usize) -> Option<Tag> {
    let ref ls = s[*pos..];

    if let Some(i) = ls.find(" +").or(ls.find(" @")) {
        let ref ys = ls[i+1..];
        let end = ys.find(" ").unwrap_or(ys.len());
        let ref tag = s[*pos..*pos+end+1];

        match tag.split_at(2) {
            (" +", name) => {
                *pos = *pos + end + 1;
                Some(Tag::Project(String::from(name)))
            },
            (" @", name) => {
                *pos = *pos + end + 1;
                Some(Tag::Context(String::from(name)))
            },
            _ => {
                *pos = s.len();
                None
            }
        }
    } else {
        *pos = s.len();
        None
    }
}

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

#[cfg(test)]
mod tests {
    use {task, completed, priority, date, description, tag};
    use Tag;
    use Priority;

    #[test]
    fn it_parse_full() {
        let line = "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github";
        let t = task(line).unwrap();
        println!("{:?}", t);

        assert_eq!(t.completed, true);
        assert_eq!(t.priority.unwrap(), Priority::High);
        assert_eq!(t.completion_date.unwrap(), "2011-03-02".to_string());
        assert_eq!(t.creation_date.unwrap(), "2011-03-01".to_string());
        assert_eq!(t.description, "Review Tim's pull request");
        assert_eq!(t.tags.len(), 2);
        assert_eq!(t.tags[0], Tag::project("TodoTxtTouch"));
        assert_eq!(t.tags[1], Tag::context("github"));
    }

    #[test]
    fn it_parse_completed() {
        assert_eq!(completed("x ", &mut 0), true);
        assert_eq!(completed("x", &mut 0), true);

        assert_eq!(completed("", &mut 0), false);
        assert_eq!(completed("a", &mut 0), false);
        assert_eq!(completed(" x", &mut 0), false);
    }

    #[test]
    fn it_parse_priority() {
        assert_eq!(priority("(A)", &mut 0), Some(Priority::High));
        assert_eq!(priority("(B)", &mut 0), Some(Priority::Mid));
        assert_eq!(priority("(C)", &mut 0), Some(Priority::Low));
        assert_eq!(priority("(D)", &mut 0), None);

        assert_eq!(priority("A", &mut 0), None);
        assert_eq!(priority("(A ", &mut 0), None);
        assert_eq!(priority(" A)", &mut 0), None);

        assert_eq!(priority("(a)", &mut 0), None);
    }

    #[test]
    fn it_parse_date() {
        assert_eq!(date("2017-04-16 ", &mut 0), Some(String::from("2017-04-16")));
        assert_eq!(date("2017-04-16", &mut 0), None);

        assert_eq!(date("2017-4-16", &mut 0), None);
        assert_eq!(date("2017-04-6", &mut 0), None);
        assert_eq!(date("17-04-16", &mut 0), None);
    }

    #[test]
    fn it_parse_description() {
        assert_eq!(description("aaaa", &mut 0), "aaaa");
        assert_eq!(description("aaaa ", &mut 0), "aaaa");
        assert_eq!(description("aaaa +", &mut 0), "aaaa");
        assert_eq!(description("aaaa @", &mut 0), "aaaa");
    }

    #[test]
    fn it_parse_tag() {
        assert_eq!(tag(" +abc", &mut 0), Some(Tag::project("abc")));
        assert_eq!(tag(" @abc", &mut 0), Some(Tag::context("abc")));
        assert_eq!(tag("+abc", &mut 0), None);
        assert_eq!(tag("@abc", &mut 0), None);
    }
}
