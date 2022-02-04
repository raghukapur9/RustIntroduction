use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::open("info.txt").expect("Unable to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Unable to read file content");
    println!("{}", file_content);
}