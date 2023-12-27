use std::fs::File;
use std::io::Write;

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
        for sens in self.sensors {
            if(!sens.isVirtual()) {
                sens.update();
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
        let mut file = File::create(self.name + ".stl")?;
        file.write(b"solid " + self.name.as_bytes());

        wallsSTL(Point {0,0,0}, self.length, self.width, self.height, file);
        for rack in map {
            rack.toSTL(file);
        }

        file.write(b"endsolid " + self.name.as_bytes());
    }
}
