use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Shoots {
    pub rate_of_fire: f64,
    pub muzzle_velocity: f32,
    pub displacement: Vec2,
    pub last_time_shot: f64,
}

impl Default for Shoots {
    fn default() -> Self {
        Shoots {
            rate_of_fire: 1.0,
            muzzle_velocity: 100.0,
            displacement: Vec2::new(0.0, 0.0),
            last_time_shot: 0.0,
        }
    }
}
