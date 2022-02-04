fn main(){
    let mut my_vec = vec![1,2,3,4];
    my_vec.push(10);
    my_vec.remove(3);
    for number in my_vec.iter(){
        println!("{}",number);
    }
}