use crate::actions::Action;
use crate::loading::{LabyrinthLevel, MazeAssets, TextureAssets};
use crate::map::{MyRaycastSet, PlaneAsset, PIXEL_WORLD_SIZE, WALL_HEIGHT};
use crate::ui::Notification;
use crate::GameState;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_raycast::{DebugCursor, DebugCursorTail, RaycastSystem};
use leafwing_input_manager::prelude::*;

pub const PLAYER_Y: f32 = -WALL_HEIGHT + PLAYER_RADIUS;
pub const PLAYER_RADIUS: f32 = 0.125;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings {
            sensitivity: 0.00017, // default: 0.00012
            speed: 1.5,           // default: 12.0
        })
        .init_resource::<CamInputState>()
        .add_event::<LeaveLabyrinthEvent>()
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_characters)
                .with_system(initial_grab_cursor),
        )
        .add_system_set(SystemSet::on_resume(GameState::Playing).with_system(initial_grab_cursor))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_look.before(follow_camera))
                .with_system(player_move.before(follow_camera))
                .with_system(leave_labyrinth.after(player_move))
                .with_system(attempt_combine)
                .with_system(follow_camera)
                .with_system(draw_markers.after(RaycastSystem::UpdateDebugCursor::<MyRaycastSet>))
                .with_system(switch_character_control.after(follow_camera)),
        );
    }
}

#[derive(Component)]
pub struct Character {
    numbers: Vec<u8>,
    pub color: CharacterColor,
}

pub enum CharacterColor {
    Green,
    Blue,
    Red,
}

impl CharacterColor {
    fn from_number(number: u8) -> CharacterColor {
        match number {
            1 => CharacterColor::Green,
            2 => CharacterColor::Blue,
            _ => CharacterColor::Red,
        }
    }
}

fn spawn_characters(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    maze_assets: Res<MazeAssets>,
    maze_levels: Res<Assets<LabyrinthLevel>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: PLAYER_RADIUS,
        subdivisions: 5,
    }));
    let maze_level = maze_levels.get(&maze_assets.one_level).unwrap();
    for (index, starting_position) in maze_level.spawns.iter().enumerate() {
        let character_number = (index as u8) + 1;
        let mut character = commands.spawn_bundle(PbrBundle {
            mesh: player_mesh.clone(),
            material: textures.get_character_texture(character_number),
            transform: Transform::from_translation(Vec3::new(
                starting_position[0] * PIXEL_WORLD_SIZE,
                PLAYER_Y,
                starting_position[1] * PIXEL_WORLD_SIZE,
            )),
            ..default()
        });
        character
            .insert(Character {
                numbers: vec![character_number],
                color: CharacterColor::from_number(character_number),
            })
            .insert(CamInputState::default());
        if character_number == 1 {
            character.insert(Controlled);
        }
    }
}

#[derive(Component)]
pub struct MarkerMask;

fn draw_markers(
    input: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut cursor: Query<
        (
            Entity,
            &mut Handle<Mesh>,
            &mut Handle<StandardMaterial>,
            &mut Transform,
        ),
        (With<DebugCursor<MyRaycastSet>>, Without<Controlled>),
    >,
    textures: Res<TextureAssets>,
    current_character: Query<(&Character, &Transform), With<Controlled>>,
    plane: Res<PlaneAsset>,
    tail: Query<Entity, With<DebugCursorTail<MyRaycastSet>>>,
) {
    let (character, char_transform) = current_character.single();
    if let Ok((entity, mut mesh, mut material, mut transform)) = cursor.get_single_mut() {
        if transform.translation.distance(char_transform.translation) < 1. {
            let up = transform.up();
            transform.translation += up.normalize() * 0.005; // 0.005
            *mesh = plane.0.clone();
            *material = match character.color {
                CharacterColor::Green => textures.green_marker_mask.clone(),
                CharacterColor::Blue => textures.blue_marker_mask.clone(),
                CharacterColor::Red => textures.red_marker_mask.clone(),
            };
            commands.entity(entity).insert(NotShadowCaster);
            if input.just_pressed(MouseButton::Left) {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: plane.0.clone(),
                        transform: transform.clone(),
                        material: match character.color {
                            CharacterColor::Green => textures.green_marker.clone(),
                            CharacterColor::Blue => textures.blue_marker.clone(),
                            CharacterColor::Red => textures.red_marker.clone(),
                        },
                        ..default()
                    })
                    .insert(NotShadowCaster);
            }
            if let Ok(tail) = tail.get_single() {
                commands.entity(tail).despawn();
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// fn draw_markers(mut commands: Commands, query: Query<(&Transform, &Intersection<MyRaycastSet>)>, old_marker_masks: Query<Entity, With<MarkerMask>>, textures: Res<TextureAssets>, current_character: Query<(&Character, &Transform), With<Controlled>>) {
// let (character, char_transform) = current_character.single();
// println!("draw_markers");
// for (transform, intersection) in &query {
//     println!("intersect");
//     if let Some(position) = intersection.position() {
//         if position.distance(char_transform.translation) < 100. {
//             println!("draw at {:?}", position);
//             let mut mask_transform = transform.clone();
//             mask_transform = mask_transform.with_translation(position.clone());
//             commands.spawn_bundle(PbrBundle {
//                 material: textures.green_marker_mask.clone(),
//                 transform: mask_transform,
//                 ..default()
//             }).insert(MarkerMask);
//         }
//     }
// }
// old_marker_masks.iter().for_each(|entity| commands.entity(entity).despawn());
// }

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

fn switch_character_control(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut fly_cam_input_state: ResMut<CamInputState>,
    mut controlled_character: Query<(Entity, &mut CamInputState), With<Controlled>>,
    characters: Query<
        (Entity, &Transform, &Character, &CamInputState),
        (Without<Camera>, Without<Controlled>),
    >,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let pressed = if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Key1) {
        1u8
    } else if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::Key2) {
        2u8
    } else if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::Key3) {
        3u8
    } else {
        0u8
    };
    if pressed > 0 {
        if let Some((entity, transform, _, cam_character_state)) = characters
            .iter()
            .find(|(_, _, character, _)| character.numbers.contains(&pressed))
        {
            let (controlled_entity, mut cam_state) = controlled_character.single_mut();
            cam_state.yaw = fly_cam_input_state.yaw;
            cam_state.pitch = fly_cam_input_state.pitch;
            commands
                .entity(controlled_entity)
                .remove_bundle::<InputManagerBundle<Action>>()
                .remove::<Controlled>();
            fly_cam_input_state.pitch = cam_character_state.pitch;
            fly_cam_input_state.yaw = cam_character_state.yaw;
            let mut camera_transform = camera.single_mut();
            camera_transform.translation = transform.translation;
            camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, cam_character_state.yaw)
                * Quat::from_axis_angle(Vec3::X, cam_character_state.pitch);
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
}

pub struct LeaveLabyrinthEvent;

fn follow_camera(
    mut character: Query<&mut Transform, (With<Controlled>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = character.get_single_mut() {
        transform.translation = camera.single_mut().translation.clone();
    }
}

fn attempt_combine(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    characters: Query<(Entity, &Transform, &Character), Without<Controlled>>,
    mut notification: ResMut<Notification>,
    mut controlled_character: Query<(&Transform, &mut Character), With<Controlled>>,
) {
    if notification.text == Some("Press space to combine parts".to_string()) {
        notification.text = None;
    }
    let (controlled_transform, mut controlled_character) = controlled_character.single_mut();
    for (entity, transform, character) in &characters {
        if transform
            .translation
            .distance(controlled_transform.translation)
            < PLAYER_RADIUS * 2.
        {
            notification.text = Some("Press space to combine parts".to_string());
            if !input.just_pressed(KeyCode::Space) {
                return;
            }
            character
                .numbers
                .iter()
                .for_each(|number| controlled_character.numbers.push(*number));
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Modified from bevy_flycam (see credits directory for copyright notice and license file)
#[derive(Default, Component)]
pub struct CamInputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pub pitch: f32,
    pub yaw: f32,
}

/// Modified from bevy_flycam (see credits directory for copyright notice and license file)
/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

/// Modified from bevy_flycam (see credits directory for copyright notice and license file)
/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

/// Modified from bevy_flycam (see credits directory for copyright notice and license file)
/// Handles keyboard input and movement
pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    maze_assets: Res<MazeAssets>,
    maze_levels: Res<Assets<LabyrinthLevel>>,
    mut leave_labyrinth_events: EventWriter<LeaveLabyrinthEvent>,
    images: Res<Assets<Image>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Some(window) = windows.get_primary() {
        if !window.cursor_locked() {
            return;
        }
        let maze_image = images.get(&maze_assets.one_data).unwrap();
        let pixel_per_row = maze_image.texture_descriptor.size.width as usize;
        let world_width = pixel_per_row as f32 * PIXEL_WORLD_SIZE;
        let maze_level = maze_levels.get(&maze_assets.one_level).unwrap();
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    #[cfg(debug_assertions)]
                    KeyCode::LShift => velocity += Vec3::Y,
                    #[cfg(debug_assertions)]
                    KeyCode::LControl => velocity -= Vec3::Y,
                    _ => (),
                }
            }

            velocity = velocity.normalize_or_zero();
            let mut movement = velocity * time.delta_seconds() * settings.speed;

            #[cfg(debug_assertions)]
            if transform.translation.y > 0.0 {
                transform.translation += movement;
                continue;
            }

            if movement.z > 0.
                && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE)
                    + PLAYER_RADIUS
                    + movement.z
                    > PIXEL_WORLD_SIZE
            {
                let slot_y = ((transform.translation.z + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let slot_x = ((transform.translation.x + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let next_pixel = (slot_y + 1) * pixel_per_row + slot_x;
                if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                    if maze_level.exit[0] == slot_x && maze_level.exit[1] == slot_y + 1 {
                        leave_labyrinth_events.send(LeaveLabyrinthEvent);
                    }
                    movement.z = 0.0;
                }
            } else if movement.z < 0.
                && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE) - PLAYER_RADIUS
                    + movement.z
                    < 0.0
            {
                let slot_y = ((transform.translation.z + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let slot_x = ((transform.translation.x + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let next_pixel = (slot_y - 1) * pixel_per_row + slot_x;
                if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                    if maze_level.exit[0] == slot_x && maze_level.exit[1] == slot_y - 1 {
                        leave_labyrinth_events.send(LeaveLabyrinthEvent);
                    }
                    movement.z = 0.0;
                }
            }

            if movement.x > 0.
                && ((transform.translation.x + world_width) % PIXEL_WORLD_SIZE)
                    + PLAYER_RADIUS
                    + movement.x
                    > PIXEL_WORLD_SIZE
            {
                let slot_y = ((transform.translation.z + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let slot_x = ((transform.translation.x + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                // corners...
                if movement.z > 0.
                    && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE)
                        + PLAYER_RADIUS
                        + movement.z
                        > PIXEL_WORLD_SIZE
                {
                    let next_pixel = (slot_y + 1) * pixel_per_row + slot_x + 1;
                    if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                        if movement.z.abs() > movement.x.abs() {
                            movement.z = 0.0;
                        } else {
                            movement.x = 0.0;
                        }
                    }
                } else if movement.z < 0.
                    && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE) - PLAYER_RADIUS
                        + movement.z
                        < 0.0
                {
                    let next_pixel = (slot_y - 1) * pixel_per_row + slot_x + 1;
                    if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                        if movement.z.abs() > movement.x.abs() {
                            movement.z = 0.0;
                        } else {
                            movement.x = 0.0;
                        }
                    }
                }
                let next_pixel = slot_y * pixel_per_row + slot_x + 1;
                if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                    if maze_level.exit[0] == slot_x + 1 && maze_level.exit[1] == slot_y {
                        leave_labyrinth_events.send(LeaveLabyrinthEvent);
                    }
                    movement.x = 0.0;
                }
            } else if movement.x < 0.
                && ((transform.translation.x + world_width) % PIXEL_WORLD_SIZE) - PLAYER_RADIUS
                    + movement.x
                    < 0.0
            {
                let slot_y = ((transform.translation.z + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                let slot_x = ((transform.translation.x + world_width / 2.) / PIXEL_WORLD_SIZE)
                    .round() as usize;
                // corners...
                if movement.z > 0.
                    && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE)
                        + PLAYER_RADIUS
                        + movement.z
                        > PIXEL_WORLD_SIZE
                {
                    let next_pixel = (slot_y + 1) * pixel_per_row + slot_x - 1;
                    if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                        if movement.z.abs() > movement.x.abs() {
                            movement.z = 0.0;
                        } else {
                            movement.x = 0.0;
                        }
                    }
                } else if movement.z < 0.
                    && ((transform.translation.z + world_width) % PIXEL_WORLD_SIZE) - PLAYER_RADIUS
                        + movement.z
                        < 0.0
                {
                    let next_pixel = (slot_y - 1) * pixel_per_row + slot_x - 1;
                    if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                        if movement.z.abs() > movement.x.abs() {
                            movement.z = 0.0;
                        } else {
                            movement.x = 0.0;
                        }
                    }
                }
                let next_pixel = slot_y * pixel_per_row + slot_x - 1;
                if maze_image.data.get(next_pixel * 4).unwrap() < &50 {
                    if maze_level.exit[0] == slot_x - 1 && maze_level.exit[1] == slot_y {
                        leave_labyrinth_events.send(LeaveLabyrinthEvent);
                    }
                    movement.x = 0.0;
                }
            }

            transform.translation += movement;
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn leave_labyrinth(
    mut events: EventReader<LeaveLabyrinthEvent>,
    controlled_character: Query<&Character, With<Controlled>>,
    mut notification: ResMut<Notification>,
    time: Res<Time>,
) {
    if let Some(_event) = events.iter().last() {
        if controlled_character.single().numbers.len() == 3 {
            notification.text = Some("You won!".to_string());
            notification.remove_when = None;
        } else {
            notification.text =
                Some("You need to combine all parts before you can leave".to_string());
            notification.remove_when = Some(time.seconds_since_startup() + 5.);
        }
    }
}

/// Modified from bevy_flycam (see credits directory for copyright notice and license file)
/// Handles looking around if cursor is locked
pub fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<CamInputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor_locked() {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
