use crate::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::navigation::EnemyWayPoints;
use crate::GameState;
use bevy::prelude::*;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn));
    }
}

struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::from_seconds(1.5, true))
    }
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    textures: Res<TextureAssets>,
    way_points: Res<EnemyWayPoints>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: textures.red.handle.clone(),
                transform: Transform::from_translation(way_points.starting_point()),
                ..default()
            })
            .insert(Enemy {
                next_way_point: way_points.points.get(1).unwrap().clone(),
                next_index: 1,
            });
    }
}
