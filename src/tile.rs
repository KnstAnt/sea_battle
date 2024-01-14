use std::fmt::Display;

pub const SYMBOL_EMPTY: char = ' ';
const SYMBOL_AROUND_BROKEN_SHIP: char = 'X';
const SYMBOL_SHIP: char = 'O';
const SYMBOL_MISS: char ='.';
const SYMBOL_HIT: char = '#';

pub enum TileStepResult {
    Miss,
    Hit(usize),
    Error(String),
}

//Клетка карты, содержит информацию для отображения
#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    AroundBrokenShip,
    Ship(usize),
    Miss,
    Hit(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let c = match self {
            Tile::Empty => SYMBOL_EMPTY,
            Tile::AroundBrokenShip => SYMBOL_AROUND_BROKEN_SHIP,
            Tile::Ship(_) => SYMBOL_SHIP,
            Tile::Miss => SYMBOL_MISS,
            Tile::Hit(_) => SYMBOL_HIT,
        };

        write!(f, "{}", c)
    }
}

impl Tile {
    pub fn on_step(&mut self) -> TileStepResult {
        match self.clone() {
            Tile::Empty | Tile::AroundBrokenShip => {
                *self = Tile::Miss;
                TileStepResult::Miss
            },
            Tile::Ship(ship_id) => {
                *self = Tile::Hit(ship_id);
                TileStepResult::Hit(ship_id)
            },
            _ => TileStepResult::Error("Вы уже стреляли в это место!".to_owned()),
        }
    }

    pub fn mark_around_hitted_ship(&mut self) {
        if let Tile::Empty = self.clone() {
            *self = Tile::AroundBrokenShip;
        }
    }
}
