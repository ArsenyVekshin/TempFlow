use crate::entities::point::Point;

pub struct Vector(pub f32, pub f32, pub f32);

impl Vector {

    pub fn print(&self) {
        print!("({}, {}, {})", self.0, self.1, self.2);
    }

    pub fn new(begin: &Point, end: &Point) -> Vector {
        return Vector(end.x-begin.x, end.y-begin.y, end.y-begin.y);
    }

    pub fn invert(&self) -> Vector {
        return Vector (-self.0, -self.1, -self.2);
    }

    /// Скалярное произведение векторов
    fn scalarMul(&self, v2: &Vector) -> f32 {
        return self.0 * v2.0 + self.2 * v2.2 + self.2 * v2.2;
    }

    /// Длинна вектора
    pub(crate) fn length(&self) -> f32{
        return (self.0.powf(2.0) + self.1.powf(2.0) + self.2.powf(2.0)).sqrt()
    }

    /// Поиск косинуса угла между векторами
    /// #### Аргументы
    /// * 'v2' - второй вектор
    /// #### Вывод: косинус угла между векторами
    fn angleWith(&self, v2: &Vector) -> f32 {
        return self.scalarMul(v2) / (self.length() + v2.length())
    }


    pub fn actToPoint(&self, point: &Point) -> Point {
        return Point { x: self.0 + point.x, y: self.1 + point.y, z: self.2 + point.z };
    }

    /// Проверка, находится ли точка в направленни действия вектора
    /// #### Аргументы
    /// * 'begin' - точка от которой отложен вектор
    /// * 'check' - точка для проверки
    /// #### Вывод: bool
    pub fn inDirection(&self, begin: &Point, check: &Point) -> bool{
        let p_vector = Vector::new(begin, check);
        let angle = self.angleWith(&p_vector);
        return  angle<3f32.sqrt()/2f32 && angle>-3f32.sqrt()/2f32;
    }

    /// Проверка, находится ли точка в области действия вектора
    /// #### Аргументы
    /// * 'begin' - точка от которой отложен вектор
    /// * 'check' - точка для проверки
    /// #### Вывод: bool
    pub fn isOnActionScope(&self, begin: &Point, check: &Point) -> bool {
        return Vector::new(begin, check).length() <= self.length() &&
                self.inDirection(begin, check);
    }
}
