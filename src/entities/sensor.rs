use std::net::UdpSocket;
use std::panic::catch_unwind;
use std::ptr::null;

enum SensorType {
    UDP,
    TCP,
    SNMP,
    NFS,
    TFTP,
    RPC
}

struct Trigger {
    min: f32,
    max: f32
}

pub struct Sensor {
    name: String,
    position: Point,
    temp: f32,
    trig: Trigger,
    address: String,
    protocol: SensorType,
    key: String
}

impl Sensor{
    fn request(&self) {
        let mut buf = [0; 4];
        if(self.protocol == UDP){
            let socket = UdpSocket::bind(self.address + " " + key)?;
            let (amt, src) = socket.recv_from(&mut buf)?;
        }
        try{
            self.temp = f32::from_be_bytes(buf);
        }
    }

    fn generateTemp(&self) {
        self.temp = self.temp + (rand::thread_rng().gen_range(0, 100)/100 as f32);
    }

    fn checkWarn(&self) -> bool {
        return (self.temp <= self.trig.min) || (self.temp >= self.trig.max);
    }

    fn isVirtual(&self) -> bool {
        return self.address != null;
    }

}