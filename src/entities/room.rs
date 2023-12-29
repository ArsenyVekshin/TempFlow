use std::fmt::format;
use std::fs::File;
use std::io::Write;
use crate::entities::point::Point;
use crate::entities::rack::Rack;
use crate::entities::sensor::Sensor;
use crate::entities::user::User;
use crate::utils::stl::wallsSTL;


pub struct Room {
    name: String,
    owner: User,
    length: f32,
    width: f32,
    height: f32,
    map: Vec<Rack>,
    sensors: Vec<Sensor>,
}

impl Room {
    pub fn updateSensors(&self) {
        for mut sens in self.sensors {
            if(!sens.isVirtual()) {
                sens.request();
            }
        }
    }

    pub fn emulateSensors(&self) {
        for sens in self.sensors {
            if(!sens.isVirtual()) {
                sens.emulate();
            }
        }
    }

    pub fn saveAsSTL(&self, file: File){
        let mut file = File::create(format!("{}.stl", &self.name))?;
        file.write(b"solid ");
        file.write(self.name.as_bytes());


        wallsSTL(&Point { x: 0.0, y: 0.0, z: 0.0 }, self.length, self.width, self.height, &mut file);
        for rack in self.map {
            rack.toSTL(&file);
        }

        file.write(b"endsolid ");
        file.write(self.name.as_bytes());
    }

    pub fn isObstacleBetween()
}
