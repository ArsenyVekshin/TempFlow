use std::collections::HashMap;
use std::ptr::null;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::entities::rack::Rack;
use crate::entities::sensor::Sensor;

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

    pub fn alert(&self, rack: &Rack, sensor: Sensor) {
        println!("ALERT: Уведомить пользователя {}, контакт: {}", self.username, self.contact);
        print!("\t Ошибка тригера точки {}", sensor.name);
        print!(", шкаф: {}", rack.name);
        println!();
    }
}
