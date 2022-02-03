enum switch {
    on,
    off
}
fn main(){
    let swtich_state:switch = switch::on;
    match swtich_state {
        switch::on => println!("switch is on"),
        switch::off  => println!("switch is off")
    }
}