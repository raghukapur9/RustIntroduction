fn main(){
    let x =4;
    {
        // code block, variables outside the block can be accessed inside
        // variables defined in the code block cannot be accessed from outside the block
        let y = 10;
        println!("x {}, y {}",x,y);
    }
}