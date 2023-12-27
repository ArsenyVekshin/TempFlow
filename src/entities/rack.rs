use std::fs::File;
use std::io::prelude::*;

pub struct Rack {
    leftAngle: Point,
    length: f32,
    width: f32,
    height: f32,
    name: str,
}

impl Rack {
    pub fn getCenter(&self) -> Point {
        return Point {
            x: self.leftAngle.x + self.length / 2,
            x: self.leftAngle.y + self.width / 2,
            x: self.leftAngle.z + self.height / 2,
        }
    }

    /// записать модель шкафа в stl-файл
    pub fn toSTL(&self, file: &mut File) {
        cubeSTL(self.leftAngle, self.length, self.width, self.height, file);
    }
}
