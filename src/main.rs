use std::env;

extern crate todo;
use todo::parser::task;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage : todo {{description}}");
        return;
    }

    let line = "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github";
    let t = task(line).unwrap();

    println!("{:?}", t);
}
