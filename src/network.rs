


pub struct NetworkSetting {
    pub difficulty: usize,
}

impl NetworkSetting {
    pub fn new(difficulty: usize) -> Self {
        NetworkSetting {
            difficulty: difficulty,
        }
    }
}