fn main() {
    let mut x = 10;
    {
        let x = 4;
        println!("x - {}",x);
    }
    println!("x - {}",x);
}