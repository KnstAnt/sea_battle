use rand::Rng;

use crate::{field::*, point::Point, step::Step, delay};

use super::{Player, PlayerStepResult};

//Игрок управляемый компьютером. Перед игрой расставляет корабли используя рандом и 
//проверяя новый корабль на корректность.
//Во время игры ходит используя рандом и проверяя ход на попадание в список
//доступных для хода клеток.
pub struct PC {
    field: Field,
}

impl PC {
    fn generate_ship(ships: &[Vec<Point>], size: i32) -> Vec<Point> {
        let mut rng = rand::thread_rng();        

        loop {
            let new_ship = match rng.gen_range(0..2) {
                0 => {
                    let x = rng.gen_range(0..FIELD_SIZE - size);
                    let y = rng.gen_range(0..FIELD_SIZE);

                    (x..x+size).map(|x| Point::new(x, y).unwrap()).collect::<Vec<Point>>()
                },
                _ => {
                    let x = rng.gen_range(0..FIELD_SIZE);
                    let y = rng.gen_range(0..FIELD_SIZE - size);

                    (y..y+size).map(|y| Point::new(x, y).unwrap()).collect::<Vec<Point>>()
                }
            };

            if !Self::check_new_ship_pos(ships, &new_ship) {
                break new_ship;
            }
        }
    }    
}

impl Player for PC {    
    fn new() -> Self {
        Self{ field: Field::new(), }
    } 

    fn make_ships(&self) -> Option<Vec<Vec<Point>>> {

        let mut ships = Vec::new();

        ships.push(Self::generate_ship(&ships, 4));
        (0..2).for_each(|_| ships.push(Self::generate_ship(&ships, 3)) );
        (0..3).for_each(|_| ships.push(Self::generate_ship(&ships, 2)) );
        (0..4).for_each(|_| ships.push(Self::generate_ship(&ships, 1)) );

        Some(ships)
    }

    fn add_ships(&mut self, ship_positions: Vec<Vec<Point>>) {
        self.field.add_ships(&ship_positions);
    }

    fn step(&mut self) -> PlayerStepResult {

        let field_data = self.field.get_non_hitted_tiles();

        let mut rng = rand::thread_rng();

        let step = loop {
            let x = rng.gen_range(0..FIELD_SIZE);
            let y = rng.gen_range(0..FIELD_SIZE);

            if !field_data[y as usize][x as usize] {
                continue;
            }

            if let Ok(step) = Step::new(x, y) {
                break step;
            }
        };

        delay(200);
        print!("Противник ходит {step}...");
        delay(500);
        match self.field.apply_step(step) {
            FieldStepResult::Miss => {
                println!(" Промах!");
                PlayerStepResult::Miss
            },
            FieldStepResult::Damaged => {
                println!(" Ранен!");
                PlayerStepResult::Hit
            },
            FieldStepResult::Destroyed => {
                println!(" Убит!");
                PlayerStepResult::Hit
            },
            FieldStepResult::Win => {
                println!(" Убит!");
                PlayerStepResult::Win
            }
            FieldStepResult::Error(_) => {
                self.step()
            }
        }           
    }

    fn format(&self) -> Vec<String> {
        let mut res = Vec::new();
        res.push("   Ваши корабли".to_string());
        res.append(&mut self.field.format());

        res
    }
}