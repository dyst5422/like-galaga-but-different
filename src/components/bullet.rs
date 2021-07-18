pub struct Bullet {
    pub damage: f64
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            damage: 1.0
        }
    }
}
