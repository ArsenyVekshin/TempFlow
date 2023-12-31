use std::fs::File;
use std::io::prelude::*;
use crate::entities::point::Point;
use crate::entities::vector::Vector;
use crate::utils::stl::cubeSTL;

/// Контейнер для хранения отдельной серверной стойки
pub struct Rack {
    /// имя стойки отображаемое для пользователя
    pub(crate) name: str,
    /// левый верхний угол стойки в координатной сетке
    leftAngle: Point,
    length: f32,
    width: f32,
    height: f32,
    /// Направление воздуха ИЗ стойки (в "теплый коридор")
    hotend: Vector,
    /// Количество слотов в стойке
    size: i8,
}

impl Rack {
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

    fn isInside(&self, point: Point) -> bool {
        return self.leftAngle.x <= point.x && self.leftAngle.x + self.length >= point.x
            && self.leftAngle.y <= point.y && self.leftAngle.y + self.width >= point.y
            && self.leftAngle.z <= point.z && self.leftAngle.z + self.height >= point.z
        ;
    }
}
