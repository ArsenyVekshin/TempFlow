use std::fs::File;

use crate::entities::point::Point;
use crate::entities::sensor::Sensor;
use crate::entities::vector::Vector;
use crate::utils::stl::cubeSTL;

/// Контейнер для хранения отдельной серверной стойки
pub struct Rack {
    /// имя стойки отображаемое для пользователя
    pub(crate) name: str,
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
            x: self.leftAngle.x + self.length / 2,
            y: self.leftAngle.y + self.width / 2,
            z: self.leftAngle.z + self.height / 2,
        }
    }

    /// записать модель шкафа в stl-файл
    pub fn toSTL(&self, file: &mut File) {
        cubeSTL(&self.leftAngle, self.length, self.width, self.height, file);
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
        return self.serverSens.iter().map(|sens| sens.temp).sum() / self.serverSens.len();
    }

    /// Температура на заданной всоте внутри шкафа
    /// #### Аргументы:
    /// - 'h' - высота в метрах (f32)
    /// #### Вывод: температура
    pub fn getTempAtHeight(&self, h: f32) -> f32 {
        for sens in self.serverSens {
            if (sens.position.z == h) {
                return sens.temp;
            }
        }
        return self.getMidTemp();
    }
}
