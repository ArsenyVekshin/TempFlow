//mod entities { mod point; }

use std::fs::File;
use std::io;
use std::io::Write;
use crate::entities::point::Point;
use crate::entities::rack::Rack;
use crate::entities::room::Room;
use crate::entities::sensor::{Sensor, SensorType, Trigger};
use crate::entities::user::User;
use crate::entities::vector::Vector;
use crate::managers::collectionManager::CollectionManager;
use crate::operators::gradient::HEIGHT_STEP;
use crate::utils::stl::plateSTL;

pub mod entities { pub mod point; pub mod rack; pub mod room; pub mod sensor; pub mod user; pub mod vector;}
pub mod utils { pub mod stl; pub mod colour;}
pub mod operators {pub mod gradient;}
pub mod managers {pub mod collectionManager;}


fn demoPlateSTL(){
    let filename = format!("C:/TempFlowOut/test.stl");
    let mut file = File::create(filename).unwrap();
    let header: [u8; 10] = [0; 10]; // записываем пустой заголовок -> файл будет интерпретирован как бинарный
    file.write(&header);
    file.write(2_u32.to_ne_bytes().as_ref());   // количество треугольнико в конечноом файле

    plateSTL(&Point { x: 0.0, y: 0.0, z: 0.0 },
             &Point { x: 1.0, y: 0.0, z: 0.0 },
             &Point { x: 1.0, y: 1.0, z: 0.0 },
             &Point { x: 0.0, y: 1.0, z: 0.0 },
             0,
             &mut file);
}

fn main() {
    demoPlateSTL();
    let mut manager: CollectionManager = CollectionManager::new();
    initDemoRack(&mut manager);

    println!("ROOM:");
    manager.rooms[0].print();
    manager.rooms[0].emulateSensors();
    manager.rooms[0].print();



    println!("Demo room inited");
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");


        input.trim();
        match input.as_str() {
            "build" =>{
                println!("Команда принята - начат просчет");
                manager.updateRoom(0);
                println!("Расчеты завершены");
            }
            "да" => {
                println!("Продолжаем!");
            }
            "нет" => {
                println!("Хорошо, выход!");
                break;
            }
            _ => {
                println!("Извините, не понимаю ваш ответ. Пожалуйста, введите 'да' или 'нет'.");
                println!("Вы ввели: {}", input);
                println!("Команда принята - начат просчет");
                manager.updateRoom(0);
                println!("Расчеты завершены");
            }
        }
    }
}


fn initDemoRack(manager: &mut CollectionManager) {
    manager.users.push(User{
        id: 0,
        username: "admin".to_string(),
        password: "admin".to_string(),
        contact: "kilmepls".to_string(),
    });
    manager.rooms.push(Room {
        id: 0,
        name: "demo".to_string(),
        owner: User{
            id: 0,
            username: "admin".to_string(),
            password: "admin".to_string(),
            contact: "kilmepls".to_string(),
        },
        length: &7.0,
        width: &7.5,
        height: &4.0,
        map: vec![],
        sensors: vec![],
    });


    let mut id: u32 = 0;
    for x in [1f32, 4f32]{
        for y in [1f32, 3f32, 5f32]{
            manager.rooms[0].map.push(
                Rack {
                    id: id,
                    name: format!("rack{}", id),
                    leftAngle: Point { x, y, z: 0.0 },
                    length: 2.0,
                    width: 2.0,
                    height: 3.0,
                    hotend: if x==1f32 { Vector(1.0, 0.0, 0.0) } else {Vector(-1.0, 0.0, 0.0)},
                    size: 6,
                    serverSens: vec![],
                }
            );
            id+=1;
        }
    }

    // добавим серверные дачтики
    let mut id: u32 = 0;
    for mut rack in &mut manager.rooms[0].map {
        for i in 0..rack.size {
            rack.addSensAt(Sensor{
                id: id,
                name: format!("sens{}", id),
                position: Point { x: 0.0,y: 0.0,z: 0.0,},
                temp: 0.0,
                trig: Trigger { min: 0.0, max: 100.0 },
                address: "127.0.0.1:5000".to_string(),
                protocol: SensorType::GET,
                key: format!("rack{}/sens/{}", rack.name, i),
            }, i as u8);
            id += 1;
        }
    }

    for y in 1..7 {
        manager.rooms[0].sensors.push(
            Sensor {
                id: id,
                name: format!("sens{}", id),
                position: Point {
                    x: 3.5,
                    y: y as f32,
                    z: 3.0,
                },
                temp: 0.0,
                trig: Trigger { min: 0.0, max: 100.0 },
                address: "127.0.0.1:5000".to_string(),
                protocol: SensorType::GET,
                key: format!("rooms/sens/{}", y),
            }
        );
        id+=1;
    }

}