use std::fs::File;
use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};

use crate::entities::point::Point;
use crate::entities::rack::Rack;
use crate::entities::sensor::Sensor;
use crate::entities::user::User;
use crate::operators::gradient::{calcGradient, HEIGHT_STEP, POINTS_PER_SQRT_METER};
use crate::utils::stl::{gradientLayerSTL, wallsSTL};

/// Контейнер для хранения данных о помещении
pub struct Room {
    pub(crate) name: String,
    pub(crate) owner: User,
    pub length: &'static f32,
    pub width: &'static f32,
    pub height: &'static f32,
    /// Набор стоек в помещении
    pub(crate) map: Vec<Rack>,
    /// Набор датчиков помещения
    pub(crate) sensors: Vec<Sensor>,
}

impl Room {

    /// Запросить данные со всех датчиков в помещении
    pub fn updateSensors(&mut self) {
        for mut sens in &mut self.sensors {
            if(!sens.isVirtual()) {
                sens.request();
            }
        }
    }

    /// Случайно сгенерировать температуры датчиков в помещении
    pub fn emulateSensors(&mut self) {
        for mut sens in &mut self.sensors {
            if(!sens.isVirtual()) {
                sens.generateTemp();
            }
        }
    }

    /// Подсчет количества треугольников в конечном STL-файла
    /// пиксель = грань = 2 треугольника
    fn calcSTLtringlesNum(&self) -> u32{
        return (2 * (4 + 4 * self.map.len() + (self.length * self.height * (POINTS_PER_SQRT_METER as f32)) as usize)) as u32;
    }

    /// Сохранить помещение как 3d карту в формате stl
    /// Создается папка "ИМЯ_ПОМЕЩЕНИЯ" и в ней STL файлы для каждого уровня градиента
    pub fn saveAsSTL(&self) {
        let gradientPack = calcGradient(&self);
        for i in 0..(self.height / HEIGHT_STEP) as usize {
            let filename = format!("{}/{}.stl", &self.name, i as f32 * HEIGHT_STEP);
            let mut file = File::create(filename).unwrap();

            let header: [u8; 10] = [0; 10]; // записываем пустой заголовок -> файл будет интерпретирован как бинарный
            file.write(&header);
            file.write(self.calcSTLtringlesNum().to_ne_bytes().as_ref());   // количество треугольнико в конечноом файле

            // границы помещения
            wallsSTL(&Point { x: 0.0, y: 0.0, z: 0.0 }, *self.length, *self.width, *self.height, &mut file);

            // отрисовка шкафов
            for rack in &self.map {
                rack.toSTL(&mut file);
            }
            gradientLayerSTL(self,&gradientPack[i], HEIGHT_STEP*i as f32, &mut file);
        }
    }

    pub fn isInsideRack(&self, x: f32, y: f32, z: f32) -> bool {
        for rack in &self.map {
            if (rack.isInside(Point { x, y, z })) { return true; }
        }
        return false;
    }
}
