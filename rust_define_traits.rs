struct Circle{
    radius: u32
}

trait CircleTraits{
    fn has_radius(&self) -> bool;
    fn print_area(&self);
}

impl CircleTraits for Circle{
    fn has_radius(&self) -> bool{
        
        if self.radius == 0{
            return false;
        }
        else{
            return true;
        }
    }
    fn print_area(&self){
        let area = 3.14*f64::from(self.radius)*f64::from(self.radius);
        println!("Radius of the circle is {}",area);
    }
}
fn main(){
    let my_circle = Circle{radius:5};
    println!("Does circle has radius? {}", my_circle.has_radius());
    my_circle.print_area();
}