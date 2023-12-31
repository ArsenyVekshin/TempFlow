use std::fmt::format;
use std::fs::File;
use std::io::Write;
use crate::entities::point::Point;
use crate::entities::rack::Rack;
use crate::entities::sensor::Sensor;
use crate::entities::user::User;
use crate::utils::stl::wallsSTL;


/// Контейнер для хранения данных о помещении
pub struct Room {
    name: String,
    owner: User,
    length: f32,
    width: f32,
    height: f32,
    /// Набор стоек в помещении
    map: Vec<Rack>,
    /// Набор датчиков помещения
    sensors: Vec<Sensor>,
}

impl Room {

    /// Запросить данные со всех датчиков в помещении
    pub fn updateSensors(&self) {
        for mut sens in self.sensors {
            if(!sens.isVirtual()) {
                sens.request();
            }
        }
    }

    /// Случайно сгенерировать температуры датчиков в помещении
    pub fn emulateSensors(&self) {
        for sens in self.sensors {
            if(!sens.isVirtual()) {
                sens.emulate();
            }
        }
    }


    /// Сохранить помещение как 3d карту в формате stl (без градиента)
    pub fn saveAsSTL(&self, file: File){
        let mut file = File::create(format!("{}.stl", &self.name))?;
        file.write(b"solid ");
        file.write(self.name.as_bytes());


        wallsSTL(&Point { x: 0.0, y: 0.0, z: 0.0 }, self.length, self.width, self.height, &mut file);
        for rack in self.map {
            rack.toSTL(&mut file);
        }

        file.write(b"endsolid ");
        file.write(self.name.as_bytes());
    }

    pub fn isObstacleBetween()
}
