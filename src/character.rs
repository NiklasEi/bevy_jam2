use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_characters))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(go_into_character_control),
            );
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

fn go_into_character_control(
    mut selection: EventReader<PickingEvent>,
    characters: Query<&Transform, (With<Character>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for event in selection.iter() {
        info!("Got {:?}", event);
        match event {
            PickingEvent::Selection(event) => match event {
                SelectionEvent::JustSelected(selected) => {
                    info!("selected!");
                    if let Ok(transform) = characters.get(*selected) {
                        info!("got character!");
                        let mut camera_transform = camera.single_mut();
                        camera_transform.translation = transform.translation;
                        *camera_transform = camera_transform.looking_at(Vec3::ZERO, Vec3::Y);
                    }
                }
                SelectionEvent::JustDeselected(_) => {}
            },
            _ => {}
        }
    }
}
