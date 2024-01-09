use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

use crate::entities::point::Point;
use crate::entities::rack::Rack;
use crate::entities::sensor::Sensor;
use crate::entities::user::User;
use crate::operators::gradient::{calcGradient, HEIGHT_STEP, POINTS_PER_SQRT_METER};
use crate::utils::stl::{gradientLayerSTL, wallsSTL};

/// Контейнер для хранения данных о помещении
pub struct Room {
    pub id: u32,
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

    pub fn print(&self) {
        println!("Room( id: {}, name: {}, owner: {}, length: {}, width: {}, height: {} )\n",
                 self.id, self.owner.username, self.name, self.length, self.width, self.height);
        for rack in &self.map{
            rack.print();
            for sens in &rack.serverSens {
                sens.print();
            }
            println!();
        }

    }

    pub fn isInside(&self, point: &Point) -> bool {
        return point.x > 0.0 && point.x < *self.length
            && point.y > 0.0 && point.y < *self.width
            && point.z > 0.0 && point.z < *self.height
        ;
    }

    /// Запросить данные со всех датчиков в помещении
    pub fn updateSensors(&mut self) {
        for mut sens in &mut self.sensors {
            if(!sens.isVirtual()) {
                sens.request();
            }
        }
        for mut rack in &mut self.map {
            rack.updateSensors();
        }
    }

    /// Случайно сгенерировать температуры датчиков в помещении
    pub fn emulateSensors(&mut self) {
        for mut sens in &mut self.sensors {
            sens.generateTemp();
        }
        for mut rack in &mut self.map {
            rack.emulateSensors();
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
        println!(); //DEBUG
        for i in 0..(self.height / HEIGHT_STEP) as usize {
            let filename = format!("C:/TempFlowOut/{}/{}.stl", &self.name, i as f32 * HEIGHT_STEP);
            println!("Сохраним модель для слоя {} в файл {}", i, filename); //DEBUG
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

    pub fn saveAsJPG(&self) {
        let gradientPack = calcGradient(&self);
        println!(); //DEBUG
        for i in 0..(self.height / HEIGHT_STEP) as usize {
            let filename = format!("C:/TempFlowOut/{}/{}.jpg", &self.name, i as f32 * HEIGHT_STEP);
            let mut img = RgbImage::new((self.length * POINTS_PER_METER) as u32, (self.width*POINTS_PER_METER) as u32);
            println!("Сохраним модель для слоя {} в файл {}", i, filename); //DEBUG

            let X_SIZE = (self.length*POINTS_PER_METER) as usize;
            for y in 0..(self.width*POINTS_PER_METER) as usize{
                for x in 0..X_SIZE as usize{
                    let pixel = Colour::newFromTempJPG(gradientPack[i][x+y*X_SIZE].value);
                    img.put_pixel(x as u32, y as u32, Rgb([pixel.r, pixel.g, pixel.b]))
                }
            }

            for rack in &self.map{
                for x in (rack.leftAngle.x*POINTS_PER_METER) as usize..=((rack.leftAngle.x + rack.length) * POINTS_PER_METER) as usize {
                    for y in (rack.leftAngle.y*POINTS_PER_METER) as usize..=((rack.leftAngle.y + rack.width) * POINTS_PER_METER) as usize {
                        img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]))
                    }
                }

            }

            img.save(filename).unwrap();
        }

    }
    pub fn isInsideRack(&self, point: &Point) -> bool {
        for rack in &self.map {
            if (rack.isInside(point)) { return true; }
        }
        return false;
    }
}
