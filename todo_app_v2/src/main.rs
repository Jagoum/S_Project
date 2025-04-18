

// in main.rs

use todo_app_v2::Task; 
extern crate todo_app_v2;
use std::io::Write;
use std::io::stdin; 
use std::io::stdout; 

fn runprompt(todo: &mut Vec<Task>){
    loop{
        let mut stdout = stdout(); 
        print!("(todo list) > "); 
        stdout.flush().expect("can't flush the stdout"); 

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("cannot readline"); 

        // take the args into the run function of lib and get the result of the computation out. 
        let args: Vec<&str> = buffer.split_whitespace().collect(); 

        todo_app_v2::run(args, todo); 
        
    }
}

fn main() { 

    let mut todo: Vec<Task> = Vec::new(); 

    runprompt(&mut todo); 
    
}