mod api;
mod config;
mod utils;

use std::env;

use crate::api::{time_get, task_get};
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Cupcli expects at least two arguments!")
    }
    let res = match args[0].as_str() {
        "timeget" => {
            let valid_args = ["today", "week", "yesterday"];
            if valid_args.contains(&args[1].as_str()) {
                time_get(&args[1])
            } else {
                panic!("Invalid value for argument 'timeget'. Only 'today', 'week', and 'yesterday' are valid!")
            }
        }
        "taskget" => {
            let valid_args = ["last"];
            if valid_args.contains(&args[1].as_str()) {
                task_get(&args[1])
            } else {
                panic!("Invalid value for argument 'taskget'. Only 'last' is valid!")
            }

        }
        "timetrack" => todo!(),
        _ => panic!("Timetrack has not been implemented yet!")
    };
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("An error occurred: {}", e)
    }
}
