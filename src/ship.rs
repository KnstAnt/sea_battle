pub enum ShipResult {
    Damaged,
    Destroyed,
}

//Корабль, расположенный на карте. Получает урон при попадании в клетку со своим ID.
pub struct Ship {
    hp: usize, 
}

impl Ship {
    pub fn new(size: usize) -> Self {
        Self{hp: size}
    }
    pub fn damage(&mut self) -> ShipResult {
        assert!(self.hp > 0);

        self.hp -= 1;

        if self.hp > 0 {
            ShipResult::Damaged
        } else {
            ShipResult::Destroyed
        }
    }

    pub fn is_live(&self) -> bool {
        self.hp > 0
    }
}
