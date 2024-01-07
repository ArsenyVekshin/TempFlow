use std::collections::VecDeque;
use std::fs::{File, read};
use std::i32::MAX as MAX_I32;

use crate::entities::point::Point;
use crate::entities::room::Room;
use crate::utils::colour::Colour;
use crate::utils::stl::plateSTL;

///Количество точек на площади 1м2
const POINTS_PER_SQRT_METER: i32 = 1000;
///Количество точек на прямой длинной 1м
const POINTS_PER_METER: i32 = 100;
///растояние от датчика, при котором температура полностью совпадает с его показаниями
const SENSOR_VALID_DISTANCE: f32 = 0.1;
/// растояние между слоями температур по высоте
const HEIGHT_STEP: f32 = 1.0;


pub struct GradientPoint {
    value: f32,
    power: i32
}
impl GradientPoint {
    ///Точка с точно известной температурой
    fn newAbsolutePoint(value: f32) -> GradientPoint {
        return GradientPoint {value, power: i32::MAX }
    }
    pub fn isAbsolutePoint(&self) -> bool {
        return self.power == i32::MAX;
    }

    /// Расчет как изменится состояние данной точки, из-за влияния другой
    /// #### Аргументы
    /// * point: точка влияние которой необходимо просчитать
    /// #### Вывод: нужно ли расчитывать эту точку повторно?
    pub fn calcImpactBy(&mut self, point: &GradientPoint) -> bool {
        if (self == GradientPoint::default()) { // данная точка "пустая"
            self.value = point.value;
            self.power = if (point.power > 0) { point.power - 1 } else { 0 };
            return true;
        } else if (point.power > self.power) {
            self.value = (self.value * self.power + point.value * point.power) / (self.power + point.power);
            self.power = point.power - self.power;
            return true;
        } else if (self.power == point.power && (self.value - point.value).abs() > 1.0) {
            self.value = (self.value + point.value) / 2
        }
        return false;
    }

    pub fn calcImpactByWithOffset(&mut self, point: &GradientPoint, delta: i32) {
        let buff = GradientPoint {value: point.value, power: point.power - delta};
        if(self.power > buff.power) { return; }
        self.calcImpactBy(&buff);
    }
}




/// Расчет зон прямого действия температур
/// (Зоны без расчета пересечений )
fn calcMatrixForHeight(room: &Room, level: f32) -> Vec<GradientPoint> {
    let X_LEN: i32 = room.length * POINTS_PER_METER;
    let POINTS_NUM: i32 = room.length * room.width * POINTS_PER_SQRT_METER;


    let matrix: Vec<GradientPoint> = Vec::with_capacity(POINTS_NUM as usize);         // матрица температур (искомый градиент)

    let mut contourPoints: VecDeque<i32> = VecDeque::with_capacity(1000);


    let calcArrayPointer = | x: f32, y: f32| -> i32 {
        return (x + y * room.length) * POINTS_PER_METER as i32;
    };

    /// Точка граничная?
    let isZoneBorder = |point: i32| -> bool {
        for i in [point + 1, point - 1, point - X_LEN, point + X_LEN] {
            if (i < 0 || i >= POINTS_NUM) { continue; }
            if (matrix[point] != matrix[i]) {
                return true;
            }
        }
        return false;
    };

    /// Заполним сектора стоек в матрице (считаем что в шкафу температура линейна)
    /// TODO: костыль с индексами, так как for не переваривает дробные числа
    for rack in room.map {
        let lvl_temp: f32 = rack.getTempAtHeight(level); // температура шкафа
        for x in rack.leftAngle.x * POINTS_PER_METER..=(rack.leftAngle.x + rack.length) * POINTS_PER_METER {
            for y in rack.leftAngle.y * POINTS_PER_METER..=(rack.leftAngle.y + rack.width) * POINTS_PER_METER {
                let ptr = x + y * X_LEN;
                matrix[ptr] = GradientPoint::newAbsolutePoint(lvl_temp);
                if (isZoneBorder(ptr)
                    && rack.hotend.inDirection(&Point { z: level, ..rack.getCenter() },
                                               &Point { x: x / POINTS_PER_METER, y: y / POINTS_PER_METER, z: level }))
                {
                    contourPoints.push(ptr);
                    matrix[ptr].power = rack.hotend.length() * POINTS_PER_METER;
                }
            }
        }
    }

    // Отметим все сенсоры (подходящие по высоте) на градиенте
    for sens in room.sensors {
        if (sens.position.z != level) { continue; }
        let ptr = calcArrayPointer(sens.position.x, sens.position.y);
        matrix[ptr] = GradientPoint { value: sens.temp, power: SENSOR_VALID_DISTANCE*POINTS_PER_METER };
        contourPoints.push(ptr);
    }


    // увеличиваем контуры шкафов и датчиков, до тех пор пока есть незаполненные точки
    while let Some(point) = contourPoints.pop_front() {
        for i in [point + 1, point - 1, point - X_LEN, point + X_LEN] {
            if (i < 0 || i >= POINTS_NUM) { continue; }
            if(matrix[i].calcImpactBy(matrix[point])) {contourPoints.push_back(i);}
        }
    }
    return matrix;
}

/// Расчет влияния слоев друг на друга
/// #### Аргументы:
/// * layers - набор расчитанных ранее слоев-градиентов
/// берем точку слоя n-1 -> считаем ее влияние на точку выше
fn calcLayersImpact(layers: & mut Vec<Vec<GradientPoint>>) {
    let Z_POWER_DELTA: i32 = (HEIGHT_STEP * POINTS_PER_METER) as i32;

    for i in 1..layers.len() {
        for ptr in 0..layers[i-1].len(){ // применяем к каждой точке слоя
            if (layers[i-1][ptr].isAbsolutePoint() || layers[i][ptr].isAbsolutePoint()) {continue;}
            layers[i][ptr].calcImpactByWithOffset(&layers[i-1][ptr], Z_POWER_DELTA);
        }
    }
}


pub fn calcGradient(room: Room, path: &str) {
    let mut gradientPack: Vec<Vec<GradientPoint>>;
    for i in (0.0 as f32..room.height).step_by(1) {
        gradientPack.push(calcMatrixForHeight(&room, i));
    }
    calcLayersImpact(&mut gradientPack);

}

pub fn gradientSTL(room: &Room, gradient: Vec<Vec<GradientPoint>>, file: &mut File) {
    const Z_STEP: f32 = room.height/gradient.len();
    const PIXEL_SIZE: f32 = (1 / POINTS_PER_METER) as f32;
    const X_LEN: i32 = room.length * POINTS_PER_METER;
    const basePoint: Point = Point {x: -PIXEL_SIZE/2, y: -PIXEL_SIZE/2, z: 0.0};

    for layer in gradient {
        let mut a = basePoint.newOffsetPoint(0.0, 0.0, Z_STEP);
        let mut b = a.newOffsetPoint(PIXEL_SIZE, 0.0 ,0.0);
        let mut c = a.newOffsetPoint(PIXEL_SIZE, PIXEL_SIZE ,0.0);
        let mut d = a.newOffsetPoint(0.0, PIXEL_SIZE ,0.0);

        for y in 0..layer.len()/X_LEN {
            //переносим точки к Ox и сдвигаем на слой по Oy
            for i in [&mut a, &mut b, &mut c, &mut d] {
                i.x = basePoint.x;
                i.y += PIXEL_SIZE;
            }

            for x in 0..X_LEN {
                for i in [&mut a, &mut b, &mut c, &mut d] { i.x += PIXEL_SIZE}
                plateSTL(&a, &b, &c, &d, Colour::newFromTemp(layer[x + y*X_LEN]).convertAsBinary1(), file);
            }
        }
    }
}