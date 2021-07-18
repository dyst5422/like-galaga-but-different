use crate::components::player::Player;
use crate::components::ship::Ship;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RigidBodyVelocity, &Ship), With<Player>>,
) {
    let w = keyboard_input.pressed(KeyCode::W);
    let a = keyboard_input.pressed(KeyCode::A);
    let s = keyboard_input.pressed(KeyCode::S);
    let d = keyboard_input.pressed(KeyCode::D);

    let mut vec = Vec2::new(0.0, 0.0);

    if w & !s {
        vec.y = 1.0;
    } else if s & !w {
        vec.y = -1.0;
    }

    if a & !d {
        vec.x = -1.0;
    } else if d & !a {
        vec.x = 1.0;
    }

    let unit = vec.normalize_or_zero();
    for (mut velocity, ship) in query.iter_mut() {
        velocity.linvel = (unit * ship.movement_speed).into();
    }
}
