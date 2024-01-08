use std::fs::File;
use std::io::Write;

use crate::entities::point::Point;
use crate::entities::room::Room;
use crate::operators::gradient::GradientPoint;
use crate::operators::gradient::POINTS_PER_METER as POINTS_PER_METER;
use crate::utils::colour::Colour;

/// Запись треугольника в STL-файл
/// #### Аргументы:
/// * a,b,c - точки углов треугольника
/// * binColour - цвет в бинарном виде
/// * file - файл для записи
pub fn triangleSTL(a: &Point, b: &Point, c: &Point, binColour: u16, file: &mut File){
    for i in 0..3 { file.write(0f32.to_ne_bytes().as_ref()).unwrap();} // нормали (всегда 0 -> расчитывается автоматически)
    for point in [a, b, c] {
        for i in [point.x, point.y, point.z] {  // запишем vertex для каждой точки
            file.write(i.to_ne_bytes().as_ref()).unwrap();
        }
    }
    file.write(binColour.to_ne_bytes().as_ref()).unwrap(); // Attribute byte count
}

/// Запись грани в STL-файл
/// #### Аргументы:
/// * a,b,c,d - точки углов грани
/// * binColour - цвет в бинарном виде
/// * file - файл для записи
pub fn plateSTL(a: &Point, b: &Point, c: &Point, d: &Point, binColour: u16, file: &mut File) {
    triangleSTL(a, b, c, binColour, file);
    triangleSTL(a, d, c, binColour, file);
}


/// Сохранить все слои градмента в 1 STL-файл
/// #### Аргументы:
/// * room - помещение для которго производился расчет
/// * gradient - вектор-хранилище для градиента
/// * file - файл для записи
pub fn gradientSTL(room: &Room, gradient: &Vec<Vec<GradientPoint>>, file: &mut File) {
    let Z_STEP: f32 = *room.height / gradient.len() as f32;
    let mut level: f32 = 0.0;

    for layer in gradient {
        level += Z_STEP;
        gradientLayerSTL(room, layer, level, file);
    }
}


/// Сохранить 1 слой градиента в 1 STL-файл
/// #### Аргументы:
/// * room - помещение для которго производился расчет
/// * layer - вектор-хранилище для уровня
/// * height - высота данного уровня
/// * file - файл для записи
pub fn gradientLayerSTL(room: &Room, layer: &Vec<GradientPoint>, height: f32, file: &mut File) {
    let PIXEL_SIZE: f32 = (1.0 / POINTS_PER_METER) as f32;
    let X_LEN: usize = (*room.length * POINTS_PER_METER) as usize;
    let basePoint: Point = Point { x: -PIXEL_SIZE / 2.0, y: -PIXEL_SIZE / 2.0, z: 0.0 };

    let mut a = basePoint.newOffsetPoint(0.0, 0.0, height);
    let mut b = a.newOffsetPoint(PIXEL_SIZE, 0.0, 0.0);
    let mut c = a.newOffsetPoint(PIXEL_SIZE, PIXEL_SIZE, 0.0);
    let mut d = a.newOffsetPoint(0.0, PIXEL_SIZE, 0.0);

    for y in 0..(layer.len() / X_LEN){
        //переносим точки к Ox и сдвигаем на слой по Oy
        for i in [&mut a, &mut b, &mut c, &mut d] {
            i.x = basePoint.x;
            i.y += PIXEL_SIZE;
        }

        for x in 0..X_LEN {
            for i in [&mut a, &mut b, &mut c, &mut d] { i.x += PIXEL_SIZE } // свдивгаем вдоль Ox
            plateSTL(&a, &b, &c, &d, Colour::newFromTemp(layer[x + y * X_LEN].value).convertAsBinary1(), file);
        }
    }

}


/// Сохранение короба (только стенки) в stl-файл
/// #### Аргументы:
/// * p1 - базовая точка (дальний левый нижний угол)
/// * length, width, height - параметры помещения
/// * file - файл для сохранения
pub fn wallsSTL(p1: &Point, length: f32, width: f32, height: f32, file: &mut File) {
    let a1 = p1.newOffsetPoint(0.0, 0.0, 0.0);  // дальний левый нижний угол
    let b1 = p1.newOffsetPoint(0.0, 0.0, height);   // дальний левый верхний угол
    let c1 = p1.newOffsetPoint(length, 0.0, 0.0);   // дальний правый нижний угол
    let d1 = p1.newOffsetPoint(length, 0.0, height);    // дальний правый верхний угол

    let a = a1.newOffsetPoint(0.0, width, 0.0);     // ближний левый нижний угол
    let b = b1.newOffsetPoint(0.0, width, 0.0);     // ближний левый верхний угол
    let c = c1.newOffsetPoint(0.0, width, 0.0);     // ближний правый нижний угол
    let d = d1.newOffsetPoint(0.0, width, 0.0);     // ближний правый верхний угол

    plateSTL(&a1, &b1, &c1, &d1, 0u16, file);   // дальняя грань XZ
    plateSTL(&a, &b, &c, &d, 0u16, file);       // ближняя грань XZ
    plateSTL(&a, &a1, &b1, &b, 0u16, file);     // левая грань YZ
    plateSTL(&d, &d1, &c1, &c, 0u16, file);     // правая грань YZ
}


