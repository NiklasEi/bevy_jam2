use crate::navigation::EnemyWayPoints;
use crate::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_enemies));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub(crate) next_way_point: Vec3,
    pub(crate) next_index: usize,
}

fn move_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy)>,
    time: Res<Time>,
    way_points: Res<EnemyWayPoints>,
) {
    let speed = 1.0;
    for (entity, mut transform, mut enemy) in &mut enemies {
        let diff = enemy.next_way_point - transform.translation;
        let mut to_move = diff.normalize() * speed * time.delta_seconds();
        if diff.length() < to_move.length() {
            let left_over = to_move.length() - diff.length();
            enemy.next_index += 1;
            let next_way_point = way_points.points.get(enemy.next_index);
            if let Some(next_way_point) = next_way_point {
                enemy.next_way_point = next_way_point.clone();
                transform.translation += to_move.clone();
                let diff = enemy.next_way_point - transform.translation;
                to_move = diff.normalize() * left_over;
            } else {
                commands.entity(entity).despawn();
            }
        }
        transform.translation += to_move;
    }
}
