use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::ship::Ship;

pub fn controller(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RigidBodyVelocity, &Ship), With<Player>>,
) {
    let w = input.pressed(KeyCode::W);
    let a = input.pressed(KeyCode::A);
    let s = input.pressed(KeyCode::S);
    let d = input.pressed(KeyCode::D);

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

    for (mut body, ship) in query.iter_mut() {
        body.linvel = (vec * ship.movement_speed).into();
    }
}
