use std::fmt::format;
use std::net::UdpSocket;
use std::panic::catch_unwind;
use std::ptr::null;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::entities::point::Point;
use crate::entities::sensor::SensorType::UDP;

#[derive(PartialEq)]
enum SensorType {
    UDP,
    TCP,
    SNMP,
    NFS,
    TFTP,
    RPC
}

#[derive(Serialize, Deserialize)]
struct Trigger {
    min: f32,
    max: f32
}

pub struct Sensor {
    pub(crate) name: String,
    pub(crate) position: Point,
    pub(crate) temp: f32,
    pub(crate) trig: Trigger,
    address: String,
    protocol: SensorType,
    key: String
}

impl Sensor{
    pub fn request(&mut self) {
        return;
        /*let mut buf = [0; 4];
        if(self.protocol == UDP){
            let socket = UdpSocket::bind(format!("{} {}", &self.address, &self.key ));
            let (amt, src) = socket.expect("REASON").recv_from(&mut buf);
        }
        self.temp = f32::from_be_bytes(buf);*/
    }

    pub fn generateTemp(&mut self) {
        self.temp = self.temp + (rand::thread_rng().gen_range(0..1) as f32 / 100f32);
    }

    pub fn checkWarn(&self) -> bool {
        return (self.temp <= self.trig.min) || (self.temp >= self.trig.max);
    }

    pub fn isVirtual(&self) -> bool {
        return self.address != "";
    }

}