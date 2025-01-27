use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    
    println!("In filepath, {file_path}");
    let contents = fs::read_to_string(file_path).expect("Probably should be read now?");
    println!("With text :\n {contents}")
}
