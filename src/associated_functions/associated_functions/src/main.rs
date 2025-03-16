mod person;

use person::person::{Person};

fn main() {
    let mut person = Person::new("Alice", "1990-01-01");

    println!("Person: {}, Date of Bird: {}, Status: {:?}", person.name, person.dob, person.status);

    person.kill();

    println!("Person after kill: {}, Status: {:?}", person.name, person.status);
}
