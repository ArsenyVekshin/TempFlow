use crate::entities::room::Room;
use crate::entities::vector::Vector;

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


/// Расчет поля точек для заданного помещения
/// #### использует уже считанные значения температур!
pub fn calcGradient(room: &Room) {
    let resultPointsСloud = Vec
}
