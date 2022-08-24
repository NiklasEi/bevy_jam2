use crate::actions::Action;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};
use leafwing_input_manager::prelude::*;
use std::f32::consts::PI;

use bevy_flycam::NoCameraPlayerPlugin;
use bevy_flycam::{player_look, player_move, FlyCamInputState, MovementSettings};

pub const PLAYER_Y: f32 = -0.55;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NoCameraPlayerPlugin)
            .insert_resource(MovementSettings {
                sensitivity: 0.00015, // default: 0.00012
                speed: 1.0,           // default: 12.0
            })
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_characters)
                    .with_system(initial_grab_cursor),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_look.before(go_into_character_control))
                    .with_system(player_move.before(go_into_character_control))
                    .with_system(go_into_character_control)
                    .with_system(follow_camera.after(go_into_character_control)),
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
            transform: Transform::from_translation(Vec3::new(0.0, PLAYER_Y, 1.0)),
            ..default()
        })
        .insert(Character)
        .insert(Controlled)
        .insert_bundle(PickableBundle::default());
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

#[derive(Component)]
pub struct Controlled;

fn go_into_character_control(
    mut commands: Commands,
    mut selection: EventReader<PickingEvent>,
    mut windows: ResMut<Windows>,
    mut fly_cam_input_state: ResMut<FlyCamInputState>,
    characters: Query<(Entity, &Transform), (With<Character>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for event in selection.iter() {
        match event {
            PickingEvent::Selection(event) => match event {
                SelectionEvent::JustSelected(selected) => {
                    if let Ok((entity, transform)) = characters.get(*selected) {
                        if let Some(window) = windows.get_primary_mut() {
                            window.set_cursor_lock_mode(true);
                            window.set_cursor_visibility(false);
                        }
                        fly_cam_input_state.pitch = 0.;
                        fly_cam_input_state.yaw = PI / 2.;
                        let mut camera_transform = camera.single_mut();
                        camera_transform.translation = transform.translation;
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

fn follow_camera(
    mut character: Query<&mut Transform, (With<Controlled>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = character.get_single_mut() {
        *transform = camera.single_mut().clone();
    }
}

// #[derive(Component)]
// pub struct Direction(Vec3);
//
// fn camera_control(
//     mut character: Query<
//         (&mut Transform, &ActionState<Action>, &mut Direction),
//         (With<Controlled>, Without<Camera>),
//     >,
//     mut camera: Query<&mut Transform, With<Camera>>,
//     time: Res<Time>,
// ) {
//     if let Ok((mut transform, action_state, mut direction)) = character.get_single_mut() {
//         let speed = 1.0;
//         let rotation_speed = 1.0;
//         if action_state.pressed(Action::Walk) {
//             println!("{}", direction.0);
//             transform.translation += direction.0 * speed * time.delta_seconds();
//         }
//         if action_state.pressed(Action::TurnLeft) {
//             let rotate_by = -rotation_speed * time.delta_seconds();
//             let rotate_to = rotate_to(rotate_by, &direction);
//             // println!(
//             //     "current {}, plus {}, rotate to {}",
//             //     direction.0.x.acos() * if direction.0.z > 0.0 { 1.0 } else { -1.0 },
//             //     rotate_by,
//             //     rotate_to
//             // );
//             direction.0 = Vec3::new(rotate_to.cos(), 0.0, rotate_to.sin());
//         }
//         if action_state.pressed(Action::TurnRight) {
//             let rotate_by = rotation_speed * time.delta_seconds();
//             let rotate_to = rotate_to(rotate_by, &direction);
//             direction.0 = Vec3::new(rotate_to.cos(), 0.0, rotate_to.sin());
//         }
//         camera.single_mut().translation = transform.translation;
//         *camera.single_mut() = camera.single_mut().looking_at(
//             transform.translation + direction.0 - Vec3::new(0.0, 0.1, 0.0),
//             Vec3::Y,
//         );
//     }
// }
//
// fn rotate_to(rotate_by: f32, direction: &Direction) -> f32 {
//     let mut rotate_to =
//         direction.0.x.acos() * if direction.0.z > 0.0 { 1.0 } else { -1.0 } + rotate_by;
//     if rotate_to < -PI {
//         rotate_to = 2.0 * PI + rotate_to
//     } else if rotate_to > PI {
//         rotate_to = rotate_to - 2.0 * PI
//     }
//
//     rotate_to
// }
