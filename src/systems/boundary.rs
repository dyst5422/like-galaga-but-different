use crate::components::player::Player;
use crate::{GAME_HEIGHT, GAME_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn boundary(mut query: Query<&mut RigidBodyPosition>, window: Res<WindowDescriptor>) {
    let width = window.width.clone() / 2.0;
    let height = window.height.clone() / 2.0;
    for mut item in query.iter_mut() {
        // println!("{:#?}  -  [{}, {}]", item.next_position.translation, window.width, window.height);
        if item.position.translation.x > width.clone() {
            item.position.translation.x = -width.clone();
        } else if item.position.translation.x < -width.clone() {
            item.position.translation.x = width.clone();
        }

        if item.position.translation.y > height.clone() {
            item.position.translation.y = -height.clone();
        } else if item.position.translation.y < -height.clone() {
            item.position.translation.y = height.clone();
        }
    }
}
