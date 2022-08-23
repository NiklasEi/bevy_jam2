use crate::actions::Action;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};
use leafwing_input_manager::prelude::*;
use std::f32::consts::PI;

const PLAYER_Y: f32 = 0.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_characters))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(go_into_character_control)
                    .with_system(camera_control.after(go_into_character_control)),
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
                radius: 0.125,
                depth: 0.25,
                ..default()
            })),
            material: textures.blue.handle.clone(),
            transform: Transform::from_translation(Vec3::new(1.0, PLAYER_Y, 1.0)),
            ..default()
        })
        .insert(Character)
        .insert_bundle(PickableBundle::default())
        .insert(Direction(Vec3::X));
}

#[derive(Component)]
pub struct Controlled;

fn go_into_character_control(
    mut commands: Commands,
    mut selection: EventReader<PickingEvent>,
    characters: Query<(Entity, &Transform, &Direction), (With<Character>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for event in selection.iter() {
        match event {
            PickingEvent::Selection(event) => match event {
                SelectionEvent::JustSelected(selected) => {
                    if let Ok((entity, transform, direction)) = characters.get(*selected) {
                        let mut camera_transform = camera.single_mut();
                        camera_transform.translation = transform.translation;
                        *camera_transform = camera_transform
                            .looking_at(camera_transform.translation + direction.0, Vec3::Y);
                        commands.entity(entity).insert(Controlled).insert_bundle(
                            InputManagerBundle::<Action> {
                                action_state: ActionState::default(),
                                input_map: InputMap::new([
                                    (KeyCode::A, Action::TurnLeft),
                                    (KeyCode::D, Action::TurnRight),
                                    (KeyCode::W, Action::Walk),
                                ]),
                            },
                        );
                    }
                }
                SelectionEvent::JustDeselected(_) => {}
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct Direction(Vec3);

fn camera_control(
    mut character: Query<
        (&mut Transform, &ActionState<Action>, &mut Direction),
        (With<Controlled>, Without<Camera>),
    >,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, action_state, mut direction)) = character.get_single_mut() {
        let speed = 1.0;
        let rotation_speed = 0.01;
        if action_state.pressed(Action::Walk) {
            println!("walk");
            transform.translation += direction.0 * speed * time.delta_seconds();
        }
        if action_state.pressed(Action::TurnLeft) {
            let rotate_by = -rotation_speed * time.delta_seconds();
            let mut rotate_to =
                direction.0.x.acos() * if direction.0.z > 0.0 { 1.0 } else { -1.0 } + rotate_by;
            if rotate_to < -PI {
                rotate_to = 2.0 * PI - rotate_to
            } else if rotate_to > PI {
                rotate_to = rotate_to - 2.0 * PI
            }
            println!(
                "current {}, plus {}, rotate to {}",
                direction.0.x.acos() * if direction.0.z > 0.0 { 1.0 } else { -1.0 },
                rotate_by,
                rotate_to
            );
            direction.0 = Vec3::new(rotate_to.cos(), PLAYER_Y, rotate_to.sin());
            println!("turn left");
        }
        if action_state.pressed(Action::TurnRight) {
            println!("turn right");
        }
        camera.single_mut().translation = transform.translation;
        *camera.single_mut() = camera.single_mut().looking_at(
            transform.translation + direction.0 - Vec3::new(0.0, 0.1, 0.0),
            Vec3::Y,
        );
    }
}
