use std::collections::VecDeque;

use crate::entities::point::Point;
use crate::entities::room::Room;

const POINTS_PER_SQRT_METER: i32 = 1000;
const POINTS_PER_METER: i32 = 100;


pub struct GradientPoint {
    pub(crate) value: f32,
    pub(crate) factors: Vec<f32>
}

impl GradientPoint {
    pub fn calcResult (&mut self){
        self.value = self.factors.iter().sum() / self.factors.len();
        self.factors.clear();
    }

    pub fn addFactor(&mut self, factor: f32) {
        self.factors.push(factor);
    }

}


/// Расчет зон прямого действия температур
/// (Зоны без расчета пересечений )
pub fn calcMatrixForHeight(room: &Room, level: f32) -> Vec<f32> {
    let X_LEN: i32 = room.length * POINTS_PER_METER;
    let POINTS_NUM: i32 = room.length * room.width * POINTS_PER_SQRT_METER;
    let matrix: Vec<f32> = Vec::with_capacity(POINTS_NUM as usize);

    let mut contourPoints: VecDeque<i32> = VecDeque::with_capacity(1000);

    let calcArrayPointer = |x: f32, y: f32| -> i32 {
        return (x + y * room.length) * POINTS_PER_METER as i32;
    };

    /// Точка - граничная?
    let isZoneBorder = |point: i32| -> bool {
        for i in [point + 1, point - 1, point - X_LEN, point + X_LEN] {
            if (i < 0 || i >= POINTS_NUM) { continue; }
            if (matrix[point] != matrix[i]) {
                return true;
            }
        }
        return false;
    };

    // Заполним содержимое шкафа (считаем что в шкафу температура линейна)
    // TODO: костыль с индексами, так как for не переваривает дробные числа
    for rack in room.map {
        let lvl_temp: f32 = rack.getTempAtHeight(level); // температура шкафа
        for x in rack.leftAngle.x * POINTS_PER_METER..=(rack.leftAngle.x + rack.length) * POINTS_PER_METER {
            for y in rack.leftAngle.y * POINTS_PER_METER..=(rack.leftAngle.y + rack.width) * POINTS_PER_METER {
                let ptr = x + y * X_LEN;
                matrix[ptr] = lvl_temp;
                if (isZoneBorder(ptr) && rack.hotend.inDirection(&Point {z: level, ..rack.getCenter()}, &Point { x: x / POINTS_PER_METER, y: y / POINTS_PER_METER, z: level })) {
                    contourPoints.push(ptr);
                }
            }
        }
    }

    // Отметим все сенсоры (подходящие по высоте) на градиенте
    for sens in room.sensors {
        if (sens.position.z != level) { continue; }
        matrix[calcArrayPointer(sens.position.x, sens.position.y)] = sens.temp;
        contourPoints.push(calcArrayPointer(sens.position.x, sens.position.y));
    }


    // увеличиваем контуры шкафов и датчиков, до тех пор пока есть незаполненные точки
    while let Some(point) = contourPoints.pop_front() {
        for i in [point + 1, point - 1, point - X_LEN, point + X_LEN] {
            if (i < 0 || i >= POINTS_NUM) { continue; }
            if (matrix[point] != matrix[i] && matrix[i] == 0) {
                contourPoints.push(i);
                matrix[i] = matrix[point].copy();
            }
        }
    }
    return matrix;
}


