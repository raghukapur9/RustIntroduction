fn main(){
    let mut n = 0;
    loop {
        n+=1;
        if n == 10{
            continue
        }
        else if n==12{
            break
        }
        println!("The number is {}",n);
    }
}