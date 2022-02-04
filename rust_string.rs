fn main(){
    let my_string = "This is a string tutorial";
    println!("Length of the string is {}",my_string.len());
    println!("Is the string empty {}", my_string.is_empty());
    for sub_string in my_string.split_whitespace(){
        println!("{}",sub_string);
    }
}