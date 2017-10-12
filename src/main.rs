extern crate todo;

use todo::task;

fn main() {
    let line = "x (A) 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github";
    let t = task(line).unwrap();

    println!("{:?}", t);
}
