use std::collections::HashMap;

fn main(){
    let mut city_state_map = HashMap::new();
    city_state_map.insert("Jammu","J&K");
    city_state_map.insert("Bangalore","Karnataka");
    // Get a value from hashmap
    match city_state_map.get("Jammu"){
        Some(city) => println!("Jammu is in {}", city),
        None => println!("Jammu does not exist")
    }
    // Remove a value from hashmap
    city_state_map.remove("Bangalore");

    //Loop over a hashmap
    for (city, state) in &city_state_map{
        println!("City - {}, State - {}", city, state);
    }
}