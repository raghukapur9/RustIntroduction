struct Circle{
    radius: u32
}

impl Circle{
    fn circle_area(&self){
        let area = 3.14*f64::from(self.radius)*f64::from(self.radius);
        println!("Area of the circle is {}",area);
    }
}
fn main(){
    let cir = Circle{radius:5};
    cir.circle_area();
}