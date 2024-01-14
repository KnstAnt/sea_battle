use crate::{delay, field::*, read_ship, read_step, ReadStepResult, point::Point, wait_until_press};

use super::{Player, PlayerStepResult};

//Живой игрок. Перед игрой руками расставляет корабли. Во время игры вводит ход с консоли.
pub struct User {
    field: Field,
}

impl User {
    fn draw_ships(&self, ships: &[Vec<Point>]) {
        let mut field = Field::new();
        field.add_ships(ships);
        let draw_data = field.format();
        println!();
        draw_data.into_iter().for_each(|u| println!("{:^25}", u));
        println!();
    }
}

impl Player for User {
    fn new() -> Self {
        Self {
            field: Field::new(),
        }
    }
    fn make_ships(&self) -> Option<Vec<Vec<Point>>> {
        println!("Введите координаты кораблей. Количество кораблей: 4 однотрубных,");
        println!("3 двухтрубных, 2 трехтрубных и  1 четырехтрубный.");
        println!("Корабли должен стоять не ближе чем в одной клетке друг от друга.");
        println!("Количество точек координат соответствует размеру корабля.");
        println!("Координаты должны лежать на одной линии. После ввода нажмите Enter. ");
        println!("Пример: а1 - однотрубный корабль, б3 б4 б5 - трехтрубный.");

        wait_until_press();

        let mut ships = Vec::new();
        let (mut q1, mut q2, mut q3, mut q4) = (4, 3, 2, 1);

        self.draw_ships(&ships);
        
        loop {
            match read_ship() {
                crate::ReadShipResult::Ship(ship_pos) => {
                    if Self::check_new_ship_pos(&ships, &ship_pos) {
                        delay(300);
                        println!("Корабли должен стоять не ближе чем в одной клетке друг от друга!");
                        delay(1000);
                        continue;
                    }

                    if Self::check_new_ship_size(&ships, &ship_pos) {
                        delay(300);
                        println!("Вы пытаетесь разместить больше кораблей чем разрешено правилами");
                        delay(1000);
                        continue;
                    }

                    match ship_pos.len() {
                        1 => q1 -= 1,
                        2 => q2 -= 1,
                        3 => q3 -= 1,
                        4 => q4 -= 1,
                        _ => {
                            println!("Ошибка ввода корабля.");
                            continue;
                        },
                    }

                    ships.push(ship_pos);
                },
                crate::ReadShipResult::Exit => return None,
            }

            self.draw_ships(&ships);
  
            delay(1000);

            if q1 == 0 && q2 == 0 && q3 == 0 && q4 == 0 {
                break;
            }
        }

        println!("Корабли расставленны.");

        delay(1000);

        Some(ships)
    }

    fn add_ships(&mut self, ship_positions: Vec<Vec<Point>>) {
        self.field.add_ships(&ship_positions);
    }

    fn step(&mut self) -> PlayerStepResult {
        match read_step() {
            ReadStepResult::Step(step) => {
                delay(200);
                print!("Вы ходите {step}...");
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
                    FieldStepResult::Error(message) => {
                        println!(" {message} Попробуйте еще раз.");
                        self.step()
                    }
                }
            }
            ReadStepResult::Exit => PlayerStepResult::Exit,
        }
    }

    fn format(&self) -> Vec<String> {
        let mut res = Vec::new();
        res.push("    Корабли противника ".to_string());
        res.append(&mut self.field.format());
        res.into_iter()
            .map(|mut s| s.as_mut_str().replace('O', " "))
            .collect()
    }  
}
