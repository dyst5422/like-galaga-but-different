pub struct Ship {
    movement_speed: f64,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            movement_speed: 40.0,
        }
    }
}
