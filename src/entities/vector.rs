use crate::entities::point::Point;

pub struct Vector(f32, f32, f32);

impl Vector {

    fn new(begin: &Point, end: &Point) -> Vector {
        return Vector(end.x-begin.x, end.y-begin.y, end.y-begin.y);
    }

    fn angleWith(&self, v2: Vector) -> f32 {
        re
    }

    /// Проверка, находится ли точка в направленни действия вектора
    /// #### Аргументы
    /// * 'begin' - точка от которой отложен вектор
    /// * 'check' - точка для проверки
    /// #### Вывод: bool
    pub fn inDirection(&self, begin: &Point, check: &Point) -> bool{
        let p_vector = Vector::new(begin, check);

        return false;
    }

}
