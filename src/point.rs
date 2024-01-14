use std::fmt::Display;

use crate::field::FIELD_SIZE;

//Точка для задания координат на карте, имеет проверку на корректность при создании
#[derive(Debug, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new (x: i32, y: i32,) -> Result<Self, String> {
        if !(0..FIELD_SIZE).contains(&x) {
            return Err("ошибка координат!".to_owned());
        } 

        if !(0..FIELD_SIZE).contains(&y) {
            return Err("ошибка координат!".to_owned());
        } 

        Ok(Self{  x: x.try_into().expect("ошибка преобразования х в Point"), 
                  y: y.try_into().expect("ошибка преобразования y в Point"),
        })
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = char::from_u32('а' as u32 + self.x as u32).expect("Ошибка отобрашения точки");
        if c == 'й' {
            c = 'к';
        }
        write!(f, "{}{}", c, self.y+1)
    }
}
