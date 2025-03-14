fn main() {
    let mut message1 = String::from("FirstPart, SecondPart");
    println!("{}", message1);
    
    let message2 = &mut message1;
    println!("{}", message2);
    
    *message2 = String::from("FirstPart");
    println!("{}", message2);
}
