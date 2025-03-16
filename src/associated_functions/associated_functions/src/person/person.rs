#[derive(Debug, PartialEq)]
pub enum Status {
    ALIVE,
    DEAD,
}

pub struct Person {
    pub name: String,
    pub dob: String,
    pub status: Status,
}

impl Person {
    pub fn new(name: &str, dob: &str) -> Self {
        Self {
            name: name.to_string(),
            dob: dob.to_string(),
            status: Status::ALIVE,
        }
    }

    pub fn kill(&mut self) {
        self.status = Status::DEAD;
    }
}
