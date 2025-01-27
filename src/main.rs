use std::env;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};
use std::iter;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In filepath, {file_path}");
    let contents = read_to_string(file_path).expect("Probably should be read now?");
    println!("With text :\n {contents}");
    let count = contents.split_whitespace().count();
    println!("val {count}");

    // Attempted some type of read each line and then count. But to be honest, instead you can
    // Take the full read_to_string() and split(" ") immediately. Simpler might be better
    // Any time the other would be good? Hmm.
    // Either way two ways. The other was way more learning.
    // But learned, line endings, file reading, the damn safety of rust preventing you from 
    // just doing something quick and dirty (looking at you if let and unpackage line in lines.)

    if let Ok(file) = File::open(file_path){
        let lines = BufReader::new(file).lines();
        let mut count = 0;
        for line in lines {
            match line {
                Ok(value) => {
                    println!("Count before is: {value}");
                    // Verify that the line is not empty.
                    if !value.is_empty() {
                        // A word is defined as any characters with a space between
                        count += value.trim().split_whitespace().count(); 
                    }
                    println!("Count after is: {count}");
                }
                Err(e) => {
                    println!("Something went wrong processing line. Error: {e}");
                    }
            }
        }
        println!("Final Count is: {count}");
    };
    
}
