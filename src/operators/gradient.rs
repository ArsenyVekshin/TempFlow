use std::collections::VecDeque;
use std::fs::{File};

use crate::entities::point::Point;
use crate::entities::room::Room;

///Количество точек на прямой длинной 1м
pub(crate) const POINTS_PER_METER: f32 = 50.0;

///Количество точек на площади 1м2
pub(crate) const POINTS_PER_SQRT_METER: i32 = 2500у;

///растояние от датчика, при котором температура полностью совпадает с его показаниями
const SENSOR_VALID_DISTANCE: f32 = 0.1;
/// растояние между слоями температур по высоте
pub(crate) const HEIGHT_STEP: f32 = 0.1;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct GradientPoint {
    pub(crate) value: f32,
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

    pub fn isDefault(&self) -> bool {
        return (self.value == 0.0) && (self.power == 0);
    }

    pub fn isTheSame(&self, other: &GradientPoint) -> bool {
        return (self.value == other.value) && (self.power == other.power);
    }

    /// Расчет как изменится состояние данной точки, из-за влияния другой
    /// #### Аргументы
    /// * point: точка влияние которой необходимо просчитать
    /// #### Вывод: нужно ли расчитывать эту точку повторно?
    pub fn calcImpactBy(&mut self, point: &GradientPoint) -> bool {
        if (self.isDefault()) { // данная точка "пустая"
            self.value = point.value;
            self.power = if (point.power > 0) { point.power - 1 } else { 0 };
            return true;
        } else if (point.power - self.power > POINTS_PER_METER as i32) {
            self.value = (self.value * (self.power as f32) + point.value * (point.power as f32)) / (self.power + point.power) as f32;
            self.power = point.power - self.power;
            return true;
        } else if ((self.value - point.value).abs() > 1.0) {
            self.value = (self.value + point.value) / 2.0

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
    let X_LEN: usize = (room.length * POINTS_PER_METER) as usize;
    let POINTS_NUM: usize = (room.length * room.width * (POINTS_PER_SQRT_METER as f32)) as usize;

    println!("Расчет градиента для h = {}m, точек для расчета {}", level, POINTS_NUM);

    let mut matrix: Vec<GradientPoint> =vec![GradientPoint{ value: 0.0, power: 0 }; POINTS_NUM];         // матрица температур (искомый градиент)
    let mut contourPoints: VecDeque<usize> = VecDeque::with_capacity(1000);

    let calcArrayPointer = | x: f32, y: f32| -> usize {
        return ((x + y * room.length) * POINTS_PER_METER) as usize;
    };

    /// Заполним сектора стоек в матрице (считаем что в шкафу температура линейна)
    /// TODO: костыль с индексами, так как for не переваривает дробные числа
    print!("\tРасчет точек внутри шкафов"); //DEBUG
    for rack in &room.map {
        print!("."); //DEBUG
        let lvl_temp: f32 = rack.getTempAtHeight(level); // температура шкафа

        for x in (rack.leftAngle.x*POINTS_PER_METER) as usize..=((rack.leftAngle.x + rack.length) * POINTS_PER_METER) as usize {
            for y in (rack.leftAngle.y*POINTS_PER_METER) as usize..=((rack.leftAngle.y + rack.width) * POINTS_PER_METER) as usize {
                let ptr: usize = x + y * X_LEN;
                matrix[ptr] = GradientPoint::newAbsolutePoint(lvl_temp);
                for i in [ptr + 1, ptr - 1, ptr - X_LEN, ptr + X_LEN] {
                    if (i < 0 || i >= POINTS_NUM) { continue; }
                    if (!matrix[ptr].isTheSame(&matrix[i])
                        && rack.hotend.inDirection(&Point { z: level, ..rack.getCenter() },
                                                &Point { x: x as f32 / POINTS_PER_METER , y: y as f32 / POINTS_PER_METER, z: level })) {
                        contourPoints.push_back(ptr);
                        matrix[ptr].power = (rack.hotend.length() * POINTS_PER_METER) as i32;
                    }
                }
            }
        }
    }
    println!("- done"); //DEBUG

    print!("\tРасчет точек датчиков"); //DEBUG
    // Отметим все сенсоры (подходящие по высоте) на градиенте
    for sens in &room.sensors {
        print!("."); //DEBUG
        if (sens.position.z != level) { continue; }
        let ptr = calcArrayPointer(sens.position.x, sens.position.y);
        matrix[ptr] = GradientPoint { value: sens.temp, power: (SENSOR_VALID_DISTANCE * POINTS_PER_METER) as i32 };
        contourPoints.push_back(ptr);
    }
    println!("- done"); //DEBUG


    print!("\tРасчет влияния точек друг на друга {}/{} ", contourPoints.len(), matrix.len()); //DEBUG
    // увеличиваем контуры шкафов и датчиков, до тех пор пока есть незаполненные точки
    while let Some(point) = contourPoints.pop_front() {
        //println!("{} - calc point \t{}\t{}\t{}", contourPoints.len(), point, matrix[point].value, matrix[point].power); //DEBUGe
        for i  in [(point + 1)as i32, (point as i32 - 1) , (point as i32- X_LEN as i32), (point + X_LEN) as i32]  {
            if (i < 0 || i >= POINTS_NUM as i32) { continue; }

            let buff = matrix[point];   // TODO: костыль чтобы решить проблему с количеством ссылок
            if(matrix[i as usize].calcImpactBy(&buff)) {
                if(!contourPoints.contains(&(i as usize))){
                    contourPoints.push_back(i as usize);
                }
            }
            matrix[point] = buff;
        }
    }
    println!("- done"); //DEBUG
    return matrix;
}

/// Расчет влияния слоев друг на друга
/// #### Аргументы:s
/// * layers - набор расчитанных ранее слоев-градиентов
/// берем точку слоя n-1 -> считаем ее влияние на точку выше`
fn calcLayersImpact(layers: & mut Vec<Vec<GradientPoint>>) {
    let Z_POWER_DELTA: i32 = (HEIGHT_STEP * POINTS_PER_METER) as i32;

    print!("Расчет влияния слоев градиента друг на друга"); //DEBUG
    for i in 1..layers.len() {
        print!("."); //DEBUG
        for ptr in 0..layers[i-1].len(){ // применяем к каждой точке слоя
            if (layers[i-1][ptr].isAbsolutePoint() || layers[i][ptr].isAbsolutePoint()) {continue;}
            let buff = layers[i-1][ptr]; // TODO: костыль чтобы решить проблему с количеством ссылок
            layers[i][ptr].calcImpactByWithOffset(&buff, Z_POWER_DELTA);
            layers[i-1][ptr] = buff;
        }
    }
    println!("- done"); //DEBUG

}


pub fn calcGradient(room: &Room) -> Vec<Vec<GradientPoint>>{
    let mut gradientPack: Vec<Vec<GradientPoint>> = vec![];
    for i in 0..(room.height / HEIGHT_STEP) as i32 {
        gradientPack.push(calcMatrixForHeight(&room, (i as f32)*HEIGHT_STEP));
    }
    calcLayersImpact(&mut gradientPack);
    return gradientPack;
}

