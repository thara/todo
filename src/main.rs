use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader, LineWriter};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::fs::OpenOptions;
use std::process;
use std::os::unix::process::CommandExt;

#[macro_use]
extern crate log;
extern crate clap;
extern crate ansi_term;
extern crate tempfile;
use ansi_term::ANSIString;
use ansi_term::Colour::{Red, Yellow, Blue};
use tempfile::NamedTempFile;

extern crate todo;
mod cli;

fn main() {
    let matches = cli::build_app().get_matches();

    let bf: PathBuf = matches.value_of("target")
        .map_or_else(|| {
                         let home_dir = env::home_dir().expect("Impossible to get your home dir.");
                         home_dir.join("todo.txt")
                     },
                     PathBuf::from);
    let txt_path = bf.as_path();

    if !txt_path.exists() {
        println!("Todo.txt does not found at {}. Create new todo.txt.",
                 txt_path.to_string_lossy());
        let f = fs::File::create(txt_path).expect("todo.txt creation failed");
        let m = f.metadata().expect("Can not access todo.txt metadata");
        let mut perm = m.permissions();
        perm.set_mode(0o600);
    } else {
        info!("Detect todo.txt in {}", txt_path.to_string_lossy());
    }

    let metadata = fs::metadata(txt_path).expect("Can not access todo.txt metadata");
    if metadata.is_dir() {
        writeln!(io::stderr(),
                 "todo.txt({}) must not a directory but a file.",
                 txt_path.to_string_lossy())
            .unwrap();
        process::exit(1);
    }

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let f = OpenOptions::new()
                .write(true)
                .append(true)
                .open(txt_path)
                .expect("todo.txt open failed");
            let description = sub_m.value_of("TASK").unwrap();

            let p = sub_m.value_of("priority");
            let task = todo::TaskBuilder::new(description).priority(p).build();

            let s = format!("{}", task);
            let bytes = s.as_bytes();
            let mut w = LineWriter::new(f);
            w.write(bytes).expect("Writing bytes is failed");
            w.write(b"\n").expect("Writing bytes is failed");
        }
        ("del", Some(sub_m)) => {
            let task_no = sub_m.value_of("TASK_NO").unwrap();
            let task_no = task_no.parse::<usize>().unwrap();

            let f = BufReader::new(OpenOptions::new()
                .read(true)
                .open(txt_path)
                .expect("todo.txt open failed"));

            let tmpfile = NamedTempFile::new().ok().unwrap();
            let tmpfile_path = tmpfile.path().to_owned();
            let mut tmp = BufWriter::new(tmpfile);

            for (i, line) in f.lines().filter_map(|e| e.ok()).enumerate() {
                if i != task_no {
                    let n = if task_no < i {
                        i - 1
                    } else {
                        i
                    };
                    format_line(n, &line);
                    tmp.write(line.as_bytes()).expect("Writing bytes is failed");
                    tmp.write(b"\n").expect("Writing bytes is failed");
                } else {
                    print!("DELETE => ");
                    format_line(i, &line);
                }
            }
            tmp.flush().expect("Flush for tmpfile is failed");

            fs::rename(tmpfile_path, txt_path).expect("Renaming file is failed");
        }
        ("edit", Some(_)) => {
            let path: &OsStr = txt_path.as_ref();
            process::Command::new("vi").arg(path).exec();
        }
        _ => {
            if metadata.len() == 0 {
                println!("todo.txt is empty.");
            } else {
                let f = BufReader::new(OpenOptions::new()
                    .read(true)
                    .open(txt_path)
                    .expect("todo.txt open failed"));
                for (i, line) in f.lines().filter_map(|e| e.ok()).enumerate() {
                    format_line(i, &line);
                }
            }
        }
    }
}

fn format_line(i: usize, s: &str) {
    if let Some(task) = todo::parser::task(&s) {
        let t = format!("{} {}", i, task);

        match task.priority {
            Some(todo::Priority::High) => println!("{}", Red.paint(t)),
            Some(todo::Priority::Mid) => println!("{}", Yellow.paint(t)),
            Some(todo::Priority::Low) => println!("{}", Blue.paint(t)),
            _ => println!("{}", ANSIString::from(t)),
        }
    }
}
