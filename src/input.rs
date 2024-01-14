use std::io;

use crate::point::Point;

use super::step::*;

pub enum ReadStepResult {
    Step(Step),
    Exit,
}
pub enum ReadShipResult {
    Ship(Vec<Point>),
    Exit,
}

//чтение хода игрока с консоли
pub fn read_step() -> ReadStepResult {
    loop {
        println!("Ваш ход (а1..к10) или введите 'выход' для завершения игры");

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            let input = input.to_lowercase().trim().to_owned();

            if input.contains("выход") {
                return ReadStepResult::Exit;
            }

            if let Some(point) = parse_point(&input) {
                return ReadStepResult::Step(Step{point});
            }

            println!("Ошибка ввода хода! Попробуйте еще раз.");
        }
    }
}

//чтение координат нового корабля с консоли
pub fn read_ship() -> ReadShipResult {
    loop {
        println!("Введите координаты корабля или введите 'выход' для завершения игры.");

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {

            if input.to_lowercase().contains("выход") {
                return ReadShipResult::Exit;
            }

            if let Some(points) = parse_points(input.as_str()) {
                if let 1..= 4 = points.len() {
                    return ReadShipResult::Ship(points);
                }
            }

            println!("Ошибка ввода хода! Попробуйте еще раз.");
        }
    }
}

fn parse_point(input: &str) -> Option<Point> {
    if let Some(mut c) = input
        .chars()
        .flat_map(|c| ('а'..='к').find(|&v| c == v))
        .next()
    {
        if c == 'к' {
            c = 'й';
        }
        let x = c as i32 - 'а' as i32;

        if let Ok(y) = input
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<i32>()
        {
            if let Ok(point) = Point::new(x, y - 1) {
                return Some(point);
            }
        }
    }
    None
}

fn parse_points(input: &str) -> Option<Vec<Point>> {

    let input: Vec<&str> = input.split(&[' ', ',', ':', '.', '-', '/']).collect();

    let mut points: Vec<Point> = input.into_iter().flat_map(parse_point ).collect();

    if points.is_empty() {
        return None;
    }

    points.dedup();

    let ship_size = points.len();

    let x_min = points.iter().min_by(|v1, v2| v1.x().cmp(&v2.x())).expect("Ошибка ввода координат корабля").x();
    let x_max = points.iter().max_by(|v1, v2| v1.x().cmp(&v2.x())).expect("Ошибка ввода координат корабля").x();
    let y_min = points.iter().min_by(|v1, v2| v1.y().cmp(&v2.y())).expect("Ошибка ввода координат корабля").y();
    let y_max = points.iter().max_by(|v1, v2| v1.y().cmp(&v2.y())).expect("Ошибка ввода координат корабля").y();

    let dx = x_max - x_min;
    let dy = y_max - y_min;

    // корабль должен располагаться либо горизонтально, либо вертикально
    if dx > 0 && dy > 0 {
        return None;
    }

    // корабль должен иметь длинну соответствующую размеру
    if dx != ship_size-1 && dy != ship_size-1 {
        return None;
    }

    Some(points)
}

pub fn is_new_game() -> bool {
    loop {
        let mut input = String::new();
        println!("Хотите сыграть еще раз? (д/н)");
        if io::stdin().read_line(&mut input).is_ok() {
            return input.to_lowercase().trim().contains('д');
        }
    }
}

pub fn delay(msec: u64) {
    std::thread::sleep(std::time::Duration::from_millis(msec));
}

pub fn wait_until_press() {
    delay(1000);
    println!("Нажмите Enter чтобы продолжить");
    loop {
        if io::stdin().read_line(&mut String::new()).is_ok() {
            return;
        }
        delay(100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let parse_res = parse_point("а1");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap(), Point::new(0,0).expect("Ошибка создания хода"));

        let parse_res = parse_point("а10");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap(), Point::new(0,9).expect("Ошибка создания хода"));  

        let parse_res = parse_point("к1");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap(), Point::new(9,0).expect("Ошибка создания хода"));

        let parse_res = parse_point("к10");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap(), Point::new(9,9).expect("Ошибка создания хода"));

        assert!(parse_point("").is_none());
        assert!(parse_point("выход").is_none());
        assert!(parse_point("а0").is_none());
        assert!(parse_point("1").is_none());
        assert!(parse_point("q1").is_none());
        assert!(parse_point("а11").is_none());
    }

    #[test]
    fn test_parse_points_value() {
        let parse_res = parse_points("а1");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap().len(), 1);

        let parse_res = parse_points("qqq");
        assert!(parse_res.is_none());
    }

    #[test]
    fn test_parse_points_separators() {
        let parse_res = parse_points("а1,а2/а3:а4");
        assert!(parse_res.is_some());
        assert_eq!(parse_res.unwrap().len(), 4);
    }

    #[test]
    fn test_parse_points_repeats() {
        let parse_res = parse_points("а1 а1 а1 а4");
        assert!(parse_res.is_none());
    }

    #[test]
    fn test_parse_points_range() {
        let parse_res = parse_points("а1 а2 а3 а5");
        assert!(parse_res.is_none());

        let parse_res = parse_points("а1 б2 а3 а4");
        assert!(parse_res.is_none());
    }
}
