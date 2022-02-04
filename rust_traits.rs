struct Circle{
    radius: u32
}

impl ToString for Circle{
    fn to_string(&self) -> String{
        return format!("Radius of the circle is {}",self.radius);
    }
}
fn main(){
    let my_circle = Circle{radius:5};
    println!("{}", my_circle.to_string());
}