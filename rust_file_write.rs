use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::create("write.txt").expect("Unable to create file");
    file.write_all(b"writing file in rust").expect("unable to write in rust");
}