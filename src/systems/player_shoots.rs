use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::shoots::Shoots;
use crate::components::bullet::Bullet;
use crate::components::player::Player;

pub fn player_shoots(
    mut commands: Commands,
    mut query: Query<(&RigidBodyPosition, &mut Shoots), With<Player>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>
) {
    let texture_handle = asset_server.load("textures/bullet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1.0, 3.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    if input.pressed(KeyCode::Space) {
        for (body, mut shoot) in query.iter_mut() {
            let current_time = time.time_since_startup().as_secs_f64();
            if current_time - shoot.last_time_shot > 1.0 / shoot.rate_of_fire {
                shoot.last_time_shot = shoot.last_time_shot + 1.0 / shoot.rate_of_fire;

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        position: Vec2::new(body.position.translation.x + shoot.displacement.x, body.position.translation.y + shoot.displacement.y).into(),
                        velocity: RigidBodyVelocity {
                            linvel: Vec2::new(0.0, shoot.muzzle_velocity).into(),
                            angvel: 0.0
                        },
                        activation: RigidBodyActivation::cannot_sleep(),
                        ccd: RigidBodyCcd {
                            ccd_enabled: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(RigidBodyPositionSync::Discrete)
                    .insert(Bullet);
            }
        }
    }
}
