struct Color{
    red: u8,
    green: u8,
    blue: u8
}
fn main(){
    let bg = Color{
        red:255,
        green:0,
        blue:123
    };
    print_color(&bg);
}

fn print_color(bg : &Color){
    println!("colors are : {}, {}, {}", bg.red, bg.green, bg.blue);
}