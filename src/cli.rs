use std;

use clap::{App, Arg, SubCommand};

extern crate todo;

pub fn build_app() -> App<'static, 'static> {
    let program = std::env::args()
        .nth(0)
        .and_then(|s| {
            std::path::PathBuf::from(s).file_stem().map(|s| {
                s.to_string_lossy().into_owned()
            })
        })
        .unwrap();

    App::new(program)
        .about("Manage your todo.txt file.")
        .version(todo::VERSION)
        .author(todo::AUTHORS)
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("FILE")
                .help("Your todo.txt (default:$HOME/todo.txt)")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add task to your todo.txt file")
                .arg(
                    Arg::with_name("TASK")
                        .help("Task content")
                        .required(true)
                        .index(1),
                )
                .arg(Arg::with_name("priority")
                    .short("p")
                    .long("pri")
                    .value_name("PRIORITY")
                    .help("Set a priority of task")
                    .takes_value(true))
                ,
        )
        // .subcommand(SubCommand::with_name("archive").about(
        //     "Moves all done tasks from todo.txt to done.txt and removes blank lines",
        // ))
        .subcommand(SubCommand::with_name("list").about("Displays all tasks"))
        .subcommand(SubCommand::with_name("edit").about("Edit todo.txt on your editor"))
}
