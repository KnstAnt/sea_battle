use std::fmt::Display;

use crate::point::Point;

//Ход в игре, содержит координаты, используется для передачи между объектами.
#[derive(Debug, PartialEq)]
pub struct Step {
    pub(crate) point: Point,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point)
    }
}

impl Step {
    pub fn new (x: i32, y: i32,) -> Result<Self, String> {
        Ok(Self{ point: Point::new( x, y)? })
    }

    pub fn x(&self) -> usize {
        self.point.x()
    }

    pub fn y(&self) -> usize {
        self.point.y()
    }
}
