fn main(){
    let mut x = 10;
    {
        let xr = &mut x;
        *xr += 1;
        println!("shadow value is - {}",xr);
    }
    println!("value of x is - {}",x);
}