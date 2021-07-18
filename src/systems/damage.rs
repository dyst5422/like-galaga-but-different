use crate::systems::enemy_spawn::Enemy;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::health::Health;
use crate::components::bullet::Bullet;

pub fn damage(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut enemy_health_query: Query<&mut Health, With<Enemy>>,
    bullet_damage_query: Query<&Bullet>,
    mut commands: Commands
) {
    for event in intersection_events.iter().filter(|e| e.intersecting) {
        let bullet_entity = event.collider1.entity();
        let enemy_entity = event.collider2.entity();

        let bullet = bullet_damage_query.get(bullet_entity).unwrap();
        let mut health = enemy_health_query.get_mut(enemy_entity).unwrap();
        health.current_health -= bullet.damage.clone();
        commands.entity(bullet_entity).despawn();

        if health.current_health <= 0.0 {
            commands.entity(enemy_entity).despawn();
        }
    }
}
