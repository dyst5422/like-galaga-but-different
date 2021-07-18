use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::health::Health;

const SPAWN_RATE: f64 = 0.1;

pub struct Spawn {
    pub last_time_spawned: f64,
}

impl Default for Spawn {
    fn default() -> Self {
        Spawn {
            last_time_spawned: 0.0,
        }
    }
}

pub struct Enemy;

pub fn init_enemy_spawn(mut commands: Commands) {
    commands.spawn().insert(Spawn::default());
}

pub fn enemy_spawn(
    mut commands: Commands,
    mut query: Query<&mut Spawn>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window: Res<WindowDescriptor>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let texture_handle = asset_server.load("textures/baddie.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(10.0, 10.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for mut spawner in query.iter_mut() {
        let current_time = time.time_since_startup().as_secs_f64();
        if current_time - spawner.last_time_spawned.clone() > 1.0 / SPAWN_RATE {
            spawner.last_time_spawned = current_time.clone();
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(5.5, 5.5),
                    collider_type: ColliderType::Sensor,
                    // flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                    // position: (Vec2::new(2.0, 0.0), 0.4).into(),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: Vec2::new(0.0, window.height.clone() / 2.0 * 0.9).into(),
                    velocity: RigidBodyVelocity {
                        linvel: Vec2::new(1.0, 0.0).into(),
                        ..Default::default()
                    },
                    forces: RigidBodyForces {
                        gravity_scale: 0.0,
                        ..Default::default()
                    },
                    activation: RigidBodyActivation::cannot_sleep(),
                    ccd: RigidBodyCcd {
                        ccd_enabled: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Health::default())
                .insert(RigidBodyPositionSync::Discrete)
                .insert(Enemy);
        }
    }
}
