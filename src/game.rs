use crate::delay;
use crate::player::PlayerStepResult;

use super::player::Player;
use super::player::pc::*;
use super::player::user::*;

//Основной класс игры. Содержит объекты игрока и пк. Создает их выхывая создание и заполнение карт,
//потом запускает игровой цикл. В цикле по очереди вызывает метод хода у игроков и перерисовывает карты
pub struct Game {
    pc: PC,
    user: User,  
}

impl Game {
    pub fn new() -> Option<Self> {
        let mut pc = PC::new();
        let mut user = User::new();

        pc.add_ships(user.make_ships()?);

        user.add_ships(pc.make_ships()?);

        Some(Self { pc, user })
    } 

    pub fn begin(&mut self) {
        loop {
            loop {
                self.draw_fields();

                match self.user.step() {
                    PlayerStepResult::Miss => break,
                    PlayerStepResult::Hit => continue,
                    PlayerStepResult::Win => {
                        self.draw_fields();
                        println!("\nПоздравляем! Вы победили!");
                        return;
                    },
                    PlayerStepResult::Exit => return,
                }
            }

            loop {
                self.draw_fields();

                match self.pc.step() {
                    PlayerStepResult::Miss => break,
                    PlayerStepResult::Hit => continue,
                    PlayerStepResult::Win => {
                        self.draw_fields();
                        println!("\nПримите наши соболезнования. Вы проиграли!");
                        return;
                    },
                    PlayerStepResult::Exit => return,
                }
            }
        }
    }

    fn draw_fields(&self) {  
        delay(1000);      
        println!();

        let mut user_iter = self.user.format().into_iter();

        self.pc.format().into_iter().for_each(|u| {
            println!("{:^25}{:^25}", u, user_iter.next().expect("Ошибка отображения поля ПК"));
        });

        println!();
        delay(1000);
    }
} 