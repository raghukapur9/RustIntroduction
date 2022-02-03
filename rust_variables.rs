fn main(){
    let mut x = 45; // i32
    let sign_num:i64 = -100000;
    let unsign_num:u64 = 100000;
    let float:f32 = -1.2;
    let boolean:bool = true;
    println!("The value of mut x is {}",x);
    println!("The value of sign num is {}", sign_num);
    println!("The value of unsign_num is {}", unsign_num);
    println!("The value of float is {}", float);
    println!("The value of bool is {}", boolean);
    x = 60;
    println!("The value of x is {}",x);
}