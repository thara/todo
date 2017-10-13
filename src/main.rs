use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::fs::OpenOptions;
use std::process;

#[macro_use]
extern crate log;
extern crate clap;
extern crate ansi_term;
use ansi_term::ANSIString;
use ansi_term::Colour::{Red, Yellow, Blue};

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
            let mut w = LineWriter::new(f);
            let task = sub_m.value_of("TASK").unwrap();
            let bytes = task.as_bytes();
            w.write(bytes).expect("Writing bytes is failed");
            w.write(b"\n").expect("Writing bytes is failed");
        }
        _ => {
            if metadata.len() == 0 {
                println!("todo.txt is empty.");
            } else {
                let f = OpenOptions::new().read(true).open(txt_path).expect("todo.txt open failed");
                let reader = BufReader::new(f);
                for (i, line) in reader.lines().enumerate() {
                    match line {
                        Ok(s) => {
                            if let Some(task) = todo::parser::task(&s) {
                                let t = format!("#{} {}", i, task);

                                match task.priority {
                                    Some(todo::Priority::High) => println!("{}", Red.paint(t)),
                                    Some(todo::Priority::Mid) => println!("{}", Yellow.paint(t)),
                                    Some(todo::Priority::Low) => println!("{}", Blue.paint(t)),
                                    _ => println!("{}", ANSIString::from(t)),
                                }

                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
