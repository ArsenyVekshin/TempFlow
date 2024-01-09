use std::ops::Index;
use std::ptr::null;
use std::string::ToString;
use crate::entities::room::Room;
use crate::entities::user::User;
use chrono::{DateTime, Duration, TimeZone, Utc};

//const OUT_PATH: String = "C:/TempFlowOut".to_string();
const sessionLifeTime: u32 = 300; // время жизни комнаты в обработке

pub struct CollectionManager {
    pub rooms: Vec<Room>,       // все комнаты
    pub users: Vec<User>,       // все зарегистрированные пользователи
    pub actualRooms: Vec<u32>,  // комнаты в обработке
    pub actualRoomsLifeTime: Vec<DateTime<Utc>> // время удаления комнаты из списка на обработку
}


impl CollectionManager {
    pub fn new() -> CollectionManager{
        return CollectionManager {
            rooms: vec![],
            users: vec![],
            actualRooms: vec![],
            actualRoomsLifeTime: vec![],
        }
    }
    /*
        pub fn lifeCycle(&self) {
            if(self.actualRooms.len() == 0) {return;}
            for i in 0.. self.rooms.len() {
                let room: Room;
                match self.getRoom(self.actualRooms[i]) {
                    Ok(value) => room = value,
                    Err(e) => panic!("Error while update room in lifecycle: {}", e)
                }
            }
        }


        /// Удалить комнату из списка обрабатываемых по id
        fn deactivateRoom(&mut self, id: u32){
            match self.getRoom(id) {
                Ok(value) => self.actualRooms.push(id),
                Err(e) => {return;}
            }
        }

        /// Добавить комнату в спиок обрабатываемых по id
        fn activateRoom(&mut self, id: u32) {
            if(self.actualRooms.contains(&id)) {
                self.actualRoomsLifeTime[self.actualRooms.iter().find(id)]
                    += Duration::from_secs(sessionLifeTime);
            }
            match self.getRoom(id) {
                Ok(value) => self.actualRooms.push(id),
                Err(e) => {return;}
            }
        }
    */
    /// Запрос данных о комнате по ID
    fn getRoomIdx(&self, id: u32) -> Result<usize, &'static str> {
        for i in 0..self.rooms.len() {
            if(self.rooms[i].id == id) { return Ok(i); }
        }
        return Err("Room with this id not found");
    }

    pub fn updateRoom(&mut self, id: u32) -> Result<bool, &'static str> {
        let mut room: &mut Room;
        match self.getRoomIdx(id) {
            Ok(value) => room = &mut self.rooms[value],
            Err(e) => return Err(e)
        }

        println!("requested room named {}", room.name);
        room.updateSensors();
        room.saveAsSTL();

        return Ok(true);
    }

}