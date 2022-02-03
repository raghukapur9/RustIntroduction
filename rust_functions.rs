fn main(){
    write_to_console("Hello there");
    println!("is the number even? {}",is_even(29));
}

fn write_to_console(msg: &str){
    println!("The string is {}", msg);
}

fn is_even(num: u8) -> bool{
    if num%2 == 0{
        return true;
    }
    else{
        return false;
    }
}