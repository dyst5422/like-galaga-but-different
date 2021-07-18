pub struct Health {
    pub max_health: f64,
    pub current_health: f64
}


impl Default for Health {
    fn default() -> Self {
        Health {
            max_health: 10.0,
            current_health: 10.0
        }
    }
}
