use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("{} arguments passed", arguments.len());
    println!("{}", arguments[1]);
}
