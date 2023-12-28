use std::fs::File;
use std::io::Write;
use crate::entities::point::Point;
/*
    Примечание: реализован перевод только граней и плоскостей параллельных осям!!
    TODO: Сделать возможность перевода обьектов не параллельных осям
*/

pub fn plateSTL(p1: &Point, p2: &Point) -> String {
    let mut buff: String = "".to_string();
    buff += "facet normal 0 0 0\nouter loop\n";
    buff += format!("vertex {} {} {} \n", p1.x, p1.y, p1.z).as_str();

    if(p2.y > p1.y) { //YZ
        buff += format!("vertex {} {} {} \n", p1.x, p2.y, p1.z).as_str();
    }
    else if(p2.x > p1.x){
        buff += format!("vertex {} {} {} \n", p2.x, p1.y, p1.z).as_str();
    }

    buff += format!("vertex {} {} {} \n", p2.x, p2.y, p2.z).as_str();
    buff += "endloop\nendfacet\n";

    buff += "facet normal 0 0 0\nouter loop\n";
    buff += format!("vertex {} {} {} \n", p1.x, p1.y, p1.z).as_str();

    if(p2.y > p1.y){ //YZ
        buff += format!("vertex {} {} {} \n", p1.x, p1.y, p2.z).as_str();
    }
    else if(p2.x > p1.x) { //XZ
        buff += format!("vertex {} {} {} \n", p1.x, p1.y, p2.z).as_str();
    }

    buff += format!("vertex {} {} {} \n", p2.x, p2.y, p2.z).as_str();
    buff += "endloop\nendfacet\n";
    return buff;
}

pub fn wallsSTL(p1: &Point, length: f32, width: f32, height: f32, file: &mut File) {
    let a = p1.copy();                            // дальний левый нижний угол
    let b = p1.newOffsetPoint(length, width, 0.0);  // ближний правый нижний угол
    let c = p1.newOffsetPoint(length, 0.0, height); // дальний правый верхний угол
    let d = p1.newOffsetPoint(0.0, width, height);  // ближний левый верхний угол

    file.write(plateSTL(&a, &d).as_bytes()); // дальняя грань XZ
    file.write(plateSTL(&b, &c).as_bytes()); // ближняя грань XZ
    file.write(plateSTL(&a, &c).as_bytes()); // левая грань YZ
    file.write(plateSTL(&b, &d).as_bytes()); // правая грань YZ
}

pub fn cubeSTL(p1: &Point, length: f32, width: f32, height: f32, file: &mut File) {
    wallsSTL(p1, length, width, height, file);

    let a = p1.copy();                            // дальний левый нижний угол
    let b = p1.newOffsetPoint(length, width, 0.0);  // ближний правый нижний угол
    let c = p1.newOffsetPoint(length, 0.0, height); // дальний правый верхний угол
    let d = p1.newOffsetPoint(0.0, width, height);  // ближний левый верхний угол

    file.write(plateSTL(&a, &b).as_bytes()); // нижняя грань XY
    file.write(plateSTL(&c, &d).as_bytes()); // верхняя грань XY
}