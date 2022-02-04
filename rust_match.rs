fn main(){
    let number = 4;
    match number{
        1 => println!("Number is 1"),
        2...10 => println!("Number is grater than 2"),
        _ => println!("Incorrect number")
    }
}