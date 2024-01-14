mod ship;
mod step;
mod field;
mod player; 
mod tile;
mod game;
mod point;
use game::*; 
mod input;
use input::*; 

fn main() {
    println!("\nДобро пожаловать!\n");
    delay(1000);

    loop {
        if let Some(mut game) = Game::new() {
            game.begin();     
        }

        if !is_new_game() {
            break;
        }
    }

    println!("\nДо Свидания!\n");
    delay(1000);
}

