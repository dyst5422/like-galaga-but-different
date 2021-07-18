mod components;
mod systems;

use crate::components::player::Player;
use crate::systems::boundary::boundary;
use crate::systems::controller::controller;
use crate::systems::damage::damage;
use crate::systems::enemy_spawn::{enemy_spawn, init_enemy_spawn, Enemy};
use crate::systems::player_shoots::player_shoots;
use crate::systems::position_text::{text_initialization, update_position_text};
use crate::systems::ship_initialization::setup_player_ship;
use crate::systems::shooter::shooter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GAME_HEIGHT: f32 = 50.0;
const GAME_WIDTH: f32 = 50.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            height: 100.0,
            width: 100.0,
            title: "Like Galaga But Different".to_string(),
            // resizable: true,
            scale_factor_override: Some(10.0),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_player_ship.system())
        .add_startup_system(setup_camera.system())
        .add_system(controller.system())
        .add_system(shooter.system())
        .add_system(player_shoots.system())
        .add_system(boundary.system())
        .add_startup_system(text_initialization.system())
        .add_system(update_position_text.system())
        .add_startup_system(init_enemy_spawn.system())
        .add_system(enemy_spawn.system())
        .add_system(damage.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
