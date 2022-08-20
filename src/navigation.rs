use crate::map::Map;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::PickingCamera;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyWayPoints>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(walk_marker));
    }
}

pub struct NavigationPlugin;

pub struct EnemyWayPoints {
    pub points: Vec<Vec3>,
}

impl Default for EnemyWayPoints {
    fn default() -> Self {
        EnemyWayPoints {
            points: vec![
                Vec3::new(-2.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(2.5, 0.0, 0.0),
            ],
        }
    }
}

impl EnemyWayPoints {
    pub fn starting_point(&self) -> Vec3 {
        self.points.first().unwrap().clone()
    }
}

fn walk_marker(picking_source: Query<&PickingCamera, Without<Map>>, map: Query<Entity, With<Map>>) {
    if let Some((entity, intersection)) = picking_source.single().intersect_top() {
        if entity == map.single() {
            info!("hit map at {:?}", intersection.position());
        }
    } else {
        info!("None")
    }
}
