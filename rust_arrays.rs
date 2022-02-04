fn main(){
    let array: [u8;5]= [1,2,3,4,5];
    for n in array.iter(){
        println!("Number is {}",n);
    }
    for i in 0..array.len(){
        println!("Number is {}",i);
    }
}