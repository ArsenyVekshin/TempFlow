use std::fmt::format;
use rand::Rng;
use crate::entities::point::Point;

#[derive(PartialEq, Debug)]
pub enum SensorType {
    GET,
    UDP,
    TCP,
    SNMP,
    NFS,
    TFTP,
    RPC
}

pub struct Trigger {
    pub min: f32,
    pub max: f32
}

pub struct Sensor {
    pub id: u32,
    pub name: String,
    pub position: Point,
    pub temp: f32,
    pub trig: Trigger,
    pub address: String,
    pub protocol: SensorType,
    pub key: String
}

impl Sensor{

    pub fn print(&self) {
        print!("Sensor id: {}, name: {}, temp: {}, address: {}, protocol: {:?}, key: {}) \n",
               self.id, self.name, self.temp, self.address, self.protocol, self.key);
    }

    pub async fn request(&mut self) {
        let response = reqwest::get(format!("{}/{}", self.address, self.key)).await.unwrap();
        let body = response.text().await.unwrap();
        self.temp = body.parse().unwrap();
        println!("Got data about sensor \'{}\': {}", self.name, body);
    }

    pub fn generateTemp(&mut self) {
        let gen: f32 = rand::thread_rng().gen_range(40.0..80.0);
        self.temp = self.temp + (gen);
    }

    pub fn checkWarn(&self) -> bool {
        return (self.temp <= self.trig.min) || (self.temp >= self.trig.max);
    }

    pub fn isVirtual(&self) -> bool {
        return self.address != "";
    }

}