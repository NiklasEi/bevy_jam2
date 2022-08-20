use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_characters));
    }
}

#[derive(Component)]
pub struct Character;

fn spawn_characters(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.5,
                ..default()
            })),
            material: textures.blue.handle.clone(),
            transform: Transform::from_translation(Vec3::new(1.0, 0.5, 1.0)),
            ..default()
        })
        .insert(Character)
        .insert_bundle(PickableBundle::default());
}
