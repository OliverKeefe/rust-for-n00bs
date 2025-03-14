use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let animals : Vec<String> = vec![
        "Dog".to_string(),
        "Cat".to_string(), 
        "Crocodile".to_string(), 
        "Albanian Yellow Tail Newt".to_string(),
    ];
    
    let fallback = String::from("Unknown");
    let random_int: usize = rng.random_range(0..animals.len());
    let animal = animals.get(random_int).unwrap_or(&fallback);
    
    match animal.as_str() {
        "Dog" => println!("Woof Woof."),
        "Cat" => println!("Meow."),
        "Crocodile" => println!("Run!"),
        _ => println!("No idea what animal that is.")
    }
}
