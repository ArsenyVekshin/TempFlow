use std::io::read_to_string;
use std::ptr::null;
use crate::entities::point::Point;

pub struct Vector(f32, f32, f32);

impl Vector {

    pub fn new(begin: &Point, end: &Point) -> Vector {
        return Vector(end.x-begin.x, end.y-begin.y, end.y-begin.y);
    }

    pub fn invert(&self) -> Vector {
        return Vector (-1*self.0, -1*self.1, -1*self.2);
    }

    /// Скалярное произведение векторов
    fn scalarMul(&self, v2: &Vector) -> f32 {
        return self.0 * v2.0 + self.2 * v2.2 + self.2 * v2.2;
    }

    /// Длинна вектора
    fn length(&self) -> f32{
        return (self.0**2 + self.1**2 + self.2**2).sqrt()
    }

    /// Поиск косинуса угла между векторами
    /// #### Аргументы
    /// * 'v2' - второй вектор
    /// #### Вывод: косинус угла между векторами
    fn angleWith(&self, v2: &Vector) -> f32 {
        return self.scalarMul(v2) / (self.length() + v2.length())
    }


    /// Проверка, находится ли точка в направленни действия вектора
    /// #### Аргументы
    /// * 'begin' - точка от которой отложен вектор
    /// * 'check' - точка для проверки
    /// #### Вывод: bool
    pub fn inDirection(&self, begin: &Point, check: &Point) -> bool{
        let p_vector = Vector::new(begin, check);
        let angle = self.angleWith(&p_vector);
        return  angle<3.sqrt()/2 && angle>-3.sqrt()/2;
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
