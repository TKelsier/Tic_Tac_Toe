pub struct Player {
    pub name: String,
    pub points: u16,
}

pub enum WonPlayer {
    Circle,
    Crosses,
    None,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: 0,
        }
    }
    pub fn add_point(&mut self) {
        self.points += 1;
    }
}
