use cupcli::request::get_time;
fn main() {
    match get_time() {
        Ok(time) => println!("{}", time),
        Err(e) => println!("{}", e),
    }
}
