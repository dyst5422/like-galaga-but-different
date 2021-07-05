mod systems;
mod components;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::player::Player;
use crate::systems::controller::controller;
use crate::systems::ship_initialization::setup_player_ship;
use crate::systems::shooter::shooter;
use crate::systems::player_shoots::player_shoots;
use crate::systems::boundary_initialization::boundary;
use bevy::window::WindowResizeConstraints;

const GAME_HEIGHT: f32 = 200.0;
const GAME_WIDTH: f32 = 100.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            height: 200.0,
            width: 100.0,
            title: "Like Galaga But Different".to_string(),
            // scale_factor_override: Some(10.0),
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
        .run();
}


fn setup_camera(mut commands: Commands) {
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
