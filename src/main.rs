mod request;
mod config;
use request::time_get;

fn main() {
    match time_get() {
        Ok(time) => println!("{}", time),
        Err(e) => println!("{}", e),
    }
}
