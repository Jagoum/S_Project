// in main.rs

use todo_app_v2::Task;
extern crate todo_app_v2;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

fn runprompt(todo: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout();
        print!("(todo list) > ");
        stdout.flush().expect("can't flush the stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("cannot readline");

        // Split only on the first whitespace to separate command from arguments
        let parts: Vec<&str> = buffer.splitn(2, char::is_whitespace).collect();
        let mut args = Vec::new();

        if !parts.is_empty() {
            args.push(parts[0]);
            if parts.len() > 1 {
                // Add the rest as a single argument, trimming any extra whitespace
                args.push(parts[1].trim());
            }
        }

        todo_app_v2::run(args, todo);
    }
}

fn main() {
    let mut todo: Vec<Task> = Vec::new();

    runprompt(&mut todo);
}
