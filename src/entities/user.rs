use std::collections::HashMap;
use sha2::{Sha256, Digest};

pub struct User {
    username: String,
    password: String,
    contact: String
}

fn get_hash(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    return  format!("{:x}", hasher.finalize());
}

impl User{
    pub fn new(username: &str, password: &str, contact: &str) -> User {
        return User {
            username: username.to_string(),
            password: get_hash(password),
            contact: contact.to_string()
        }
    }

    pub fn checkPassword(&self, password: &str) -> bool {
        return self.password == get_hash(password);
    }

    pub fn alert(&self, rack: Rack, sensor: Sensor) {
        println!("ALERT: Уведомить пользователя {}, контакт: {}", self.username, self.contact);
        if(rack != null) {
            print!("\t Ошибка тригера точки {}", sensor.name);
        }
        if(rack != null) {
            print!(", шкаф: {}", rack.name);
        }
        println!();
    }
}
