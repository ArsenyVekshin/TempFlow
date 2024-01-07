const RED_COLOUR_TEMP: f32 = 100.0;
const GREEN_COLOUR_TEMP: f32 = 50.0;
const BLUE_COLOUR_TEMP: f32 = 0.0;



pub struct Colour {
    r: u8,
    g: u8,
    b: u8
}


impl Colour {

    fn new(r: u8, g: u8, b: u8) -> Colour {
        return Colour {r, g, b};
    }

    /// Генерирует цвет из значения температуры в диапазоне (синий) 0 -> 100 (красный)
    pub fn newFromTemp(value: f32) -> Colour {
        let delta = value - GREEN_COLOUR_TEMP;
        return Colour {
            r: if (delta > 0.0) {(32 * (delta/GREEN_COLOUR_TEMP)) % 32} else {0} ,
            g: (32 * (1 - delta/GREEN_COLOUR_TEMP)) % 32,
            b: if (delta < 0.0) {(-32 * (delta/GREEN_COLOUR_TEMP)) % 32} else {0}
        }
    }

    /// Конвертация цвета в бинарный вид, для сохранения в stl
    /// Для стандартов: ```VisCam и SolidView```
    pub fn convertAsBinary1(&self) -> u16 {
        let mut out: u16 = 0b1000000000000000;  // флаг "у фигуры свой цвет" (15 бит)
        out += self.b;          // b на позициях 0-4
        out += self.g << 5 ;    // g на позициях 5-9
        out += self.r << 10 ;   // r на позициях 10-14
        return out;
    }

    /// Конвертация цвета в бинарный вид, для сохранения в stl
    /// Для стандартов: ```Materialise Magics```
    pub fn convertAsBinary2(&self) -> u16 {
        let mut out: u16 = 0b1000000000000000;  // флаг "у фигуры свой цвет" (15 бит)
        out += self.r;          // r на позициях 0-4
        out += self.g << 5 ;    // g на позициях 5-9
        out += self.b << 10 ;   // b на позициях 10-14
        return out;
    }

}