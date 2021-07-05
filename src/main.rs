mod systems;
mod components;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::build()
        // .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Like Galaga But Different".to_string(),
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera.system())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_ship(mut commands: Commands, asset_server: Res<AssetServer>) {

    assets.load("textures/ship.png");

    /* Create the ship. */
    let collider = ColliderBundle {
        shape: ColliderShape::convex_polyline(),
        ..Default::default()
    };
    commands.spawn_bundle(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(0.0, 10.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(rigid_body)
        .insert_bundle(collider);
}
