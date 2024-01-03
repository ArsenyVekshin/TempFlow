pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32
}

impl Point {
    /// Расстояние до другой точки
    pub fn distance(&self, p2: &Point) -> f32 {
        let dx = p2.x - self.x;
        let dy = p2.y - self.y;
        let dz = p2.z - self.z;
        return (dx*dx + dy*dy + dz*dz).sqrt()
    }

    /// Создание новой точки с заданным смещением
    pub fn newOffsetPoint(&self, dx: f32, dy: f32, dz: f32) -> Point {
        return Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz
        };
    }
}


