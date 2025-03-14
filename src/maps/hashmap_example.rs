use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u8, &str> = HashMap::new();
    
    map.insert(1, "Entry 1");
    map.insert(2, "Entry 2");
    
    for kvp in map.iter() {
        println!("Key: {}, Value: {}", kvp.0, kvp.1);
    }
    
    let value : Option<&&str> = map.get(&2);
    match value {
        Some(v) => println!("{}", v),
        None => println!("Not found")
    }
}
