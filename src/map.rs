use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::{Highlighting, PickableBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map));
    }
}

#[derive(Component)]
pub struct Map;

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: textures.green.handle.clone(),
            ..default()
        })
        .insert(Map)
        .insert_bundle(PickableBundle::default())
        .insert(Highlighting {
            initial: textures.green.handle.clone(),
            hovered: None,
            pressed: None,
            selected: None,
        });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
