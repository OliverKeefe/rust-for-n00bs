enum Status {
    Dead,
    Alive,
    WoefullyInebriated 
}

enum Location {
    Pub,
    Home,
    Gym
}

fn main() {
    let (first_name, last_name, status, location) = ("John", "Smith", Status::WoefullyInebriated, Location::Pub);
    match status {
        Status::Dead => println!("{first_name} {last_name} is dead."),
        Status::Alive => println!("{first_name} {last_name} is alive"),
        Status::WoefullyInebriated => println!("{first_name} {last_name} needs to go home.")
    }
}
