use std::io;
fn main(){
    let mut user_input = String::new();
    println!("Enter the input");
    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            println!("Input entered correctly {}",user_input);
        },
        Err(e) => println!("Error reading the input - {}",e)
    }
}