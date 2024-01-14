use crate::point::Point;

pub mod pc; 
pub mod user; 

pub enum PlayerStepResult {
    Miss,
    Hit,
    Win,
    Exit,
}

// Интерфейс игрока, общий для пользователя и пк. Содержит в себе карту противника 
// для облегчения доступа к ней. В начале игры расставляет корабли и передает их
// противнику. После начала игры ходит и проверяет условия выигрыша. Отдает данные
// в обьект игры для отображения карты.
pub trait Player {
    fn new() -> Self;
    fn make_ships(&self) -> Option<Vec<Vec<Point>>>;
    fn add_ships(&mut self, ship_positions: Vec<Vec<Point>>);
    fn step(&mut self) -> PlayerStepResult;    
    fn format(&self) -> Vec<String>;

    //проверка что новый корабль не стоит рядом с уже расставленными
    fn check_new_ship_pos(ships: &[Vec<Point>], new_ship: &[Point]) -> bool {
        new_ship.iter().any(|ns| ships.iter().any(|s| s.iter().any(|p| {
            (p.x() as i32 - ns.x() as i32).abs() <= 1 &&
            (p.y() as i32 - ns.y() as i32).abs() <= 1
        } ) ) )
    }

    //проверка количества уже расставленных кораблей вместе с новым кораблем
    fn check_new_ship_size(ships: &[Vec<Point>], new_ship: &[Point]) -> bool {
        let lens = ships.iter().map(|s| s.len()).collect::<Vec<usize>>();
        let lens = lens.iter().filter(|&&v| v == new_ship.len() ).collect::<Vec<_>>().len();

        lens > 4 - new_ship.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::user::*;

    #[test]
    fn test_player_new_ship_pos() {
        let old_ships = vec![
            vec![Point::new(1, 1).unwrap()], 
            vec![Point::new(3, 1).unwrap()],
            vec![Point::new(5, 1).unwrap(), Point::new(5, 2).unwrap(), Point::new(5, 3).unwrap(), Point::new(5, 4).unwrap()]
        ];

        let new_ship = vec![Point::new(7, 1).unwrap(), Point::new(7, 2).unwrap(), Point::new(7, 3).unwrap()];
        assert!(!User::check_new_ship_pos(&old_ships, &new_ship));

        let new_ship = vec![Point::new(6, 1).unwrap()];
        assert!(User::check_new_ship_pos(&old_ships, &new_ship));

        let new_ship = vec![Point::new(5, 2).unwrap()];
        assert!(User::check_new_ship_pos(&old_ships, &new_ship));

        let new_ship = vec![Point::new(6, 5).unwrap()];
        assert!(User::check_new_ship_pos(&old_ships, &new_ship));
    }

    #[test]
    fn check_new_ship_size() {
        let old_ships = vec![
            vec![Point::new(0, 0).unwrap()], 
            vec![Point::new(0, 2).unwrap()], 
            vec![Point::new(0, 4).unwrap()], 
            vec![Point::new(0, 6).unwrap()],
            vec![Point::new(2, 0).unwrap(), Point::new(2, 1).unwrap()],
            vec![Point::new(2, 3).unwrap(), Point::new(2, 4).unwrap()],
            vec![Point::new(2, 6).unwrap(), Point::new(2, 7).unwrap()],
            vec![Point::new(4, 0).unwrap(), Point::new(4, 1).unwrap(), Point::new(4, 2).unwrap()],
            vec![Point::new(4, 4).unwrap(), Point::new(4, 5).unwrap(), Point::new(4, 6).unwrap()],
            vec![Point::new(6, 0).unwrap(), Point::new(6, 1).unwrap(), Point::new(6, 2).unwrap(), Point::new(6, 3).unwrap()],
        ];

        let new_ship = vec![Point::new(8, 0).unwrap()];
        assert!(User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap()];
        assert!(User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap(), Point::new(8, 2).unwrap()];
        assert!(User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap(), Point::new(8, 2).unwrap(), Point::new(8, 3).unwrap()];
        assert!(User::check_new_ship_size(&old_ships, &new_ship));


        let old_ships = vec![
            vec![Point::new(0, 0).unwrap()], 
            vec![Point::new(0, 2).unwrap()], 
            vec![Point::new(0, 4).unwrap()], 
            vec![Point::new(2, 0).unwrap(), Point::new(2, 1).unwrap()],
            vec![Point::new(2, 3).unwrap(), Point::new(2, 4).unwrap()],
            vec![Point::new(4, 0).unwrap(), Point::new(4, 1).unwrap(), Point::new(4, 2).unwrap()],
        ];

        let new_ship = vec![Point::new(8, 0).unwrap()];
        assert!(!User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap()];
        assert!(!User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap(), Point::new(8, 2).unwrap()];
        assert!(!User::check_new_ship_size(&old_ships, &new_ship));

        let new_ship = vec![Point::new(8, 0).unwrap(), Point::new(8, 1).unwrap(), Point::new(8, 2).unwrap(), Point::new(8, 3).unwrap()];
        assert!(!User::check_new_ship_size(&old_ships, &new_ship));
    }

}