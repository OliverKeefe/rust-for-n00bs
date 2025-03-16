mod person;

use person::{Person, Status};

fn main() {
    let mut person = Person::new("Alice, "1990-01-03);
    println!("Name: {}, Status {:?}", person.name, person.status);

    person.kill();

    println!("Name: {}, is now: {:?}, person.name, person.status");
}