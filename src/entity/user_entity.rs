struct UserEntity {
    id: u64,
    name: String,
    password_string: String,
}

impl UserEntity {
    fn set_id(&mut self, id: u64) {
        self.id = id;
    }
    
    fn get_id(&self) -> u64 {
        self.id
    }
    
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn set_password(&mut self, password_string: String) {
        self.password_string = password_string;
    }
    
    fn get_password(&self) -> &str {
        &self.password_string
    }
}

fn login(user: &UserEntity) -> bool{
    if user.get_name() == "Alice" && user.get_password() == "password" {
        true
    } else {
        false
    }
}

fn main() {
    let mut user = UserEntity {
        id: 1,
        name: "Alice".to_string(),
        password_string: "password".to_string(),
    };

    if login(&user) {
        println!("Login Successful!");
    } else {
        println!("Login Failed!");
    }

    println!("User ID before: {}", user.get_id());
    user.set_id(2);
    println!("User ID After: {}", user.get_id());

}
