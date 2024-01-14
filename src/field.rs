use crate::point::Point;

use super::tile::*;
use super::ship::*;
use super::step::*;

pub const FIELD_SIZE: i32 = 10;

pub enum FieldStepResult {
    Miss,
    Damaged,
    Destroyed,
    Win,
    Error(String),
}

//Игровое поле. Содержит клетки поля и список кораблей. В начале игры получает список кораблей.
//При ходе дергает соответствующую клетку и при попадании дергает соответсвующий корабль. 
pub struct Field {
    tiles: Vec<Vec<Tile>>,
    ships: Vec<Ship>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            tiles: (0..FIELD_SIZE).map(|_| (0..FIELD_SIZE).map(|_| Tile::Empty ).collect::<Vec<_>>() ).collect::<Vec<_>>(),

            ships: Vec::new(),
        }
    }
    
    pub fn add_ships(&mut self, ship_positions: &[Vec<Point>]) {

        ship_positions.iter().for_each(|points| {
            
            if (1..4).contains(&points.len()) {
                self.ships.push(Ship::new(points.len()));
            }

            points.iter().for_each(|pos| {
                assert!((pos.x() as i32) < FIELD_SIZE && (pos.y() as i32) < FIELD_SIZE);
                self.tiles[pos.y()][pos.x()] = Tile::Ship(self.ships.len()-1);  
            }); 
        });
    }

    pub fn apply_step(&mut self, step: Step) -> FieldStepResult {
        match self.apply_step_to_tile(&step) {
            TileStepResult::Miss => {
                FieldStepResult::Miss
            },
            TileStepResult::Hit(ship_id) => {
                let ship_hit_res = self.apply_damage_to_ship(ship_id);  
                self.mark_fields_around_ship(&step, ship_id);              
    
                if self.is_live() {
                    match ship_hit_res {
                        ShipResult::Damaged => FieldStepResult::Damaged,
                        ShipResult::Destroyed => FieldStepResult::Destroyed,
                    }
                } else {
                    FieldStepResult::Win
                }
            },
            TileStepResult::Error(message) => {
                FieldStepResult::Error(message)
            },
        }
    }

    //Получение списка клеток по которым еще можно стрелять. Используется для выбора цели ПК
    pub fn get_non_hitted_tiles(&self) -> Vec<Vec<bool>> {
        self.tiles.iter().map(|v| {
            v.iter().map(|t| matches!(t, Tile::Empty | Tile::Ship(_)) ).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }

     //Форматирование поля для отображения. Выводит поля в массов строк в виде символов асции
    pub fn format(&self) -> Vec<String> {
        let mut res = Vec::new();
        res.push("   a б в г д е ж з и к".to_string());
    //    res.push(format!("   ") + &(0..19).map(|_| '_' ).collect::<String>());

        for i in 0..self.tiles.len() {
            
            let mut string = if i < 9 {
                format!(" {}", i+1)
            } else {
                format!("{}", i+1)
            };

            let mut iter = self.tiles[i].iter();
            string += &format!("|{}", iter.next().expect("Ошибка форматирования игрового поля"));
            string += &iter.map(|t| { format!(" {t}") }).collect::<String>();
            string += "|";

            res.push(string);
        }

        res.push("  ".to_string() + &String::from_utf16(&[0xAF; 21]).unwrap());

        res
    }

    fn apply_step_to_tile(&mut self, step: &Step) -> TileStepResult {
        self.tiles[step.y()][step.x()].on_step()
    }

    fn apply_damage_to_ship(&mut self, ship_id: usize) -> ShipResult {
        assert!(self.ships.len() > ship_id);
        self.ships[ship_id].damage()
    }

    //Проверка что есть живые корабли, если их нет, то мы победили
    fn is_live(&self) -> bool {
        self.ships.iter().any(|ship| ship.is_live())
    }

    //Отмечаем клетки вокруг подбитых или потопленных кораблей.
    fn mark_fields_around_ship(&mut self, step: &Step, ship_id: usize) {
        assert!(self.ships.len() > ship_id);

        //если корабль не потоплен отмечаем занятыми клетки по диагонали
        if self.ships[ship_id].is_live() {
            if let Ok(point) = Point::new(step.x() as i32 + 1, step.y() as i32 + 1) {
                self.tiles[point.y()][point.x()].mark_around_hitted_ship();
            }

            if let Ok(point) = Point::new(step.x() as i32 + 1, step.y() as i32 - 1) {
                self.tiles[point.y()][point.x()].mark_around_hitted_ship();
            }

            if let Ok(point) = Point::new(step.x() as i32 -1, step.y() as i32 + 1) {
                self.tiles[point.y()][point.x()].mark_around_hitted_ship();
            }

            if let Ok(point) = Point::new(step.x() as i32 - 1, step.y() as i32 - 1) {
                self.tiles[point.y()][point.x()].mark_around_hitted_ship();
            }
        } else {//если корабль потоплен, то отмечаем занятыми все соседние свободные клетки
            for y in 0..FIELD_SIZE {
                for x in 0..FIELD_SIZE {
                    if self.tiles[y as usize][x as usize] == Tile::Hit(ship_id) {
                        let x_min = 0.max(x - 1) as usize;
                        let x_max = (FIELD_SIZE-1).min(x + 1) as usize;
                
                        let y_min = 0.max(y - 1) as usize;
                        let y_max = (FIELD_SIZE-1).min(y + 1) as usize;
                        
                        (y_min..=y_max).for_each(|y| (x_min..=x_max).for_each(|x| self.tiles[y][x].mark_around_hitted_ship() ));
                    }
                }
            }
        }
    }
}

