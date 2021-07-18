pub struct Ship {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            movement_speed: 40.0,
            rotation_speed: 1.0,
        }
    }
}
