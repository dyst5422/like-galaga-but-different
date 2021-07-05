use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn boundary_initialization(
    mut commands: Commands,
) {
    commands.spawn_bundle(Collida)
}
