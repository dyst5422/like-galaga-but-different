use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::shoots::Shoots;

pub fn setup_player_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/ship.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(11.0, 11.0), 1, 1);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased,
            position: Vec2::new(0.0, -40.0).into(),
            activation: RigidBodyActivation::cannot_sleep(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RapierConfiguration {
            scale: 1.0,
            ..Default::default()
        })
        .insert(Shoots {
            displacement: Vec2::new(0.0, 7.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player)
        .insert(Ship);
}
