use std::fs::File;

use crate::entities::point::Point;
use crate::entities::sensor::Sensor;
use crate::entities::vector::Vector;
use crate::utils::stl::{wallsSTL};

/// Контейнер для хранения отдельной серверной стойки
pub struct Rack {
    /// имя стойки отображаемое для пользователя
    pub(crate) name: String,
    /// левый верхний угол стойки в координатной сетке
    pub leftAngle: Point,
    pub length: f32,
    pub width: f32,
    pub height: f32,
    /// Направление воздуха ИЗ стойки (в "теплый коридор")
    pub hotend: Vector,
    /// Количество слотов в стойке
    pub size: i8,
    pub serverSens: Vec<Sensor>
}

impl Rack {
    /// Абсолютный центр стойки
    pub fn getCenter(&self) -> Point {
        return Point {
            x: self.leftAngle.x + self.length / 2.0,
            y: self.leftAngle.y + self.width / 2.0,
            z: self.leftAngle.z + self.height / 2.0,
        }
    }

    /// записать модель шкафа в stl-файл
    pub fn toSTL(&self, file: &mut File) {
        wallsSTL(&self.leftAngle, self.length, self.width, self.height, file);
    }

    /// Лежит ли данная точка в шкафу?
    pub fn isInside(&self, point: Point) -> bool {
        return self.leftAngle.x <= point.x && self.leftAngle.x + self.length >= point.x
            && self.leftAngle.y <= point.y && self.leftAngle.y + self.width >= point.y
            && self.leftAngle.z <= point.z && self.leftAngle.z + self.height >= point.z
        ;
    }


    /// Средняя температура стойки
    pub fn getMidTemp(&self) -> f32 {
        let mut mid: f32 = 0.0;
        for sens in &self.serverSens { mid += sens.temp;}
        return mid / (self.serverSens.len() as f32);
    }

    /// Температура на заданной всоте внутри шкафа
    /// #### Аргументы:
    /// - 'h' - высота в метрах (f32)
    /// #### Вывод: температура
    pub fn getTempAtHeight(&self, h: f32) -> f32 {
        for sens in &self.serverSens {
            if (sens.position.z == h) {
                return sens.temp;
            }
        }
        return self.getMidTemp();
    }
}
