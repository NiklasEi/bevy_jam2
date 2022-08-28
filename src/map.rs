use crate::loading::{LabyrinthLevel, LabyrinthMaterials, MazeAssets, TextureAssets};
use crate::shape::Plane;
use crate::GameState;
use bevy::prelude::*;

pub const PIXEL_WORLD_SIZE: f32 = 0.7;
pub const WALL_HEIGHT: f32 = 0.3;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map));
    }
}

fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    labyrinth_materials: Res<LabyrinthMaterials>,
    maze_assets: Res<MazeAssets>,
    maze_levels: Res<Assets<LabyrinthLevel>>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let plane = meshes.add(Plane::default().into());
    let maze_image = images.get(&maze_assets.one_data).unwrap();
    let pixel_per_row = maze_image.texture_descriptor.size.width as usize;
    let world_width = pixel_per_row as f32 * PIXEL_WORLD_SIZE;
    let data = &maze_image.data;
    let maze_level = maze_levels.get(&maze_assets.one_level).unwrap();
    for pixel_x in 0..pixel_per_row {
        for pixel_y in 0..pixel_per_row {
            let pixel = pixel_y * pixel_per_row + pixel_x;
            if data.get(pixel * 4).unwrap() > &50 {
                let mut transform = Transform::from_translation(Vec3::new(
                    pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                    -WALL_HEIGHT,
                    pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                ));
                transform = transform.with_scale(Vec3::splat(PIXEL_WORLD_SIZE));
                commands.spawn_bundle(PbrBundle {
                    mesh: plane.clone(),
                    material: labyrinth_materials.ground.clone(),
                    transform,
                    ..default()
                });
                // +x
                if pixel_x < pixel_per_row - 1 && data.get((pixel + 1) * 4).unwrap() < &50 {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.
                            + PIXEL_WORLD_SIZE / 2.,
                        -WALL_HEIGHT / 2.,
                        pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                    ));
                    transform = transform.with_scale(Vec3::new(PIXEL_WORLD_SIZE, 1.0, WALL_HEIGHT));
                    transform = transform.looking_at(transform.translation - Vec3::Y, -Vec3::X);
                    if maze_level.exit[0] == pixel_x + 1 && maze_level.exit[1] == pixel_y {
                        transform.translation.y = -3. * (WALL_HEIGHT / 4.);
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                    }
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: labyrinth_materials.wall.clone(),
                        transform,
                        ..default()
                    });
                }
                // -x
                if pixel_x > 0 && data.get((pixel - 1) * 4).unwrap() < &50 {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE
                            - world_width / 2.
                            - PIXEL_WORLD_SIZE / 2.,
                        -WALL_HEIGHT / 2.,
                        pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                    ));
                    transform = transform.with_scale(Vec3::new(PIXEL_WORLD_SIZE, 1.0, WALL_HEIGHT));
                    transform = transform.looking_at(transform.translation + Vec3::Y, Vec3::X);
                    if maze_level.exit[0] == pixel_x - 1 && maze_level.exit[1] == pixel_y {
                        transform.translation.y = -3. * (WALL_HEIGHT / 4.);
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                    }
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: labyrinth_materials.wall.clone(),
                        transform,
                        ..default()
                    });
                }
                // +y
                if pixel_y < pixel_per_row - 1
                    && data.get((pixel + pixel_per_row) * 4).unwrap() < &50
                {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        -WALL_HEIGHT / 2.,
                        pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.
                            + PIXEL_WORLD_SIZE / 2.,
                    ));
                    transform = transform.with_scale(Vec3::new(PIXEL_WORLD_SIZE, 1.0, WALL_HEIGHT));
                    transform = transform.looking_at(transform.translation - Vec3::Y, -Vec3::Z);
                    if maze_level.exit[0] == pixel_x && maze_level.exit[1] == pixel_y + 1 {
                        transform.translation.y = -3. * (WALL_HEIGHT / 4.);
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                    }
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: labyrinth_materials.wall.clone(),
                        transform,
                        ..default()
                    });
                }
                // -y
                if pixel_y > 0 && data.get((pixel - pixel_per_row) * 4).unwrap() < &50 {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        -WALL_HEIGHT / 2.,
                        pixel_y as f32 * PIXEL_WORLD_SIZE
                            - world_width / 2.
                            - PIXEL_WORLD_SIZE / 2.,
                    ));
                    transform = transform.with_scale(Vec3::new(PIXEL_WORLD_SIZE, 1.0, WALL_HEIGHT));
                    transform = transform.looking_at(transform.translation + Vec3::Y, Vec3::Z);
                    if maze_level.exit[0] == pixel_x && maze_level.exit[1] == pixel_y - 1 {
                        transform.translation.y = -3. * (WALL_HEIGHT / 4.);
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                    }
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: labyrinth_materials.wall.clone(),
                        transform,
                        ..default()
                    });
                }
            } else {
                if maze_level.exit[0] == pixel_x && maze_level.exit[1] == pixel_y {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        -WALL_HEIGHT / 2.,
                        pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                    ));
                    transform = transform.with_scale(Vec3::splat(PIXEL_WORLD_SIZE));
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: textures.grass.clone().into(),
                        transform,
                        ..default()
                    });
                    // +x
                    if pixel_x < pixel_per_row - 1 && data.get((pixel + 1) * 4).unwrap() < &50 {
                        let mut transform = Transform::from_translation(Vec3::new(
                            pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.
                                + PIXEL_WORLD_SIZE / 2.,
                            -WALL_HEIGHT / 4.,
                            pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        ));
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                        transform = transform.looking_at(transform.translation - Vec3::Y, -Vec3::X);
                        commands.spawn_bundle(PbrBundle {
                            mesh: plane.clone(),
                            material: labyrinth_materials.wall.clone(),
                            transform,
                            ..default()
                        });
                    }
                    // -x
                    if pixel_x > 0 && data.get((pixel - 1) * 4).unwrap() < &50 {
                        let mut transform = Transform::from_translation(Vec3::new(
                            pixel_x as f32 * PIXEL_WORLD_SIZE
                                - world_width / 2.
                                - PIXEL_WORLD_SIZE / 2.,
                            -WALL_HEIGHT / 4.,
                            pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        ));
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                        transform = transform.looking_at(transform.translation + Vec3::Y, Vec3::X);
                        commands.spawn_bundle(PbrBundle {
                            mesh: plane.clone(),
                            material: labyrinth_materials.wall.clone(),
                            transform,
                            ..default()
                        });
                    }
                    // +y
                    if pixel_y < pixel_per_row - 1
                        && data.get((pixel + pixel_per_row) * 4).unwrap() < &50
                    {
                        let mut transform = Transform::from_translation(Vec3::new(
                            pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                            -WALL_HEIGHT / 4.,
                            pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.
                                + PIXEL_WORLD_SIZE / 2.,
                        ));
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                        transform = transform.looking_at(transform.translation - Vec3::Y, -Vec3::Z);
                        commands.spawn_bundle(PbrBundle {
                            mesh: plane.clone(),
                            material: labyrinth_materials.wall.clone(),
                            transform,
                            ..default()
                        });
                    }
                    // -y
                    if pixel_y > 0 && data.get((pixel - pixel_per_row) * 4).unwrap() < &50 {
                        let mut transform = Transform::from_translation(Vec3::new(
                            pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                            -WALL_HEIGHT / 4.,
                            pixel_y as f32 * PIXEL_WORLD_SIZE
                                - world_width / 2.
                                - PIXEL_WORLD_SIZE / 2.,
                        ));
                        transform = transform.with_scale(Vec3::new(
                            PIXEL_WORLD_SIZE,
                            1.0,
                            WALL_HEIGHT / 2.,
                        ));
                        transform = transform.looking_at(transform.translation + Vec3::Y, Vec3::Z);
                        commands.spawn_bundle(PbrBundle {
                            mesh: plane.clone(),
                            material: labyrinth_materials.wall.clone(),
                            transform,
                            ..default()
                        });
                    }
                } else {
                    let mut transform = Transform::from_translation(Vec3::new(
                        pixel_x as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                        0.0,
                        pixel_y as f32 * PIXEL_WORLD_SIZE - world_width / 2.,
                    ));
                    transform = transform.with_scale(Vec3::splat(PIXEL_WORLD_SIZE));
                    commands.spawn_bundle(PbrBundle {
                        mesh: plane.clone(),
                        material: textures.grass.clone().into(),
                        transform,
                        ..default()
                    });
                }
            }
        }
    }

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });
}

// #[derive(Debug, Copy, Clone)]
// struct MazePlane<'a> {
//     extent: f32,
//     num_vertices: u32,
//     maze_pixel_per_row: usize,
//     maze_handle: &'a Handle<Image>,
//     image_assets: &'a Assets<Image>,
// }
//
// impl<'a> From<MazePlane<'a>> for Mesh {
//     fn from(plane: MazePlane) -> Self {
//         let diff = plane.extent / plane.num_vertices as f32;
//
//         let vertices = (0..=plane.num_vertices)
//             .cartesian_product(0..=plane.num_vertices)
//             .map(|(y, x)| {
//                 // println!("{}/{}", x, y);
//                 let uv_x = x as f32 / (plane.num_vertices / 2) as f32;
//                 let uv_y = y as f32 / (plane.num_vertices / 2) as f32;
//                 (
//                     [
//                         x as f32 * diff - 0.5 * plane.extent,
//                         0.0,
//                         y as f32 * diff - 0.5 * plane.extent,
//                     ],
//                     [0.0, 1.0, 0.0],
//                     [
//                         if uv_x > 1. { 2. - uv_x } else { uv_x },
//                         if uv_y > 1. { 2. - uv_y } else { uv_y },
//                     ],
//                 )
//             })
//             .collect::<Vec<_>>();
//
//         let indices = Indices::U32(
//             (0..=plane.num_vertices)
//                 .cartesian_product(0..=plane.num_vertices)
//                 .enumerate()
//                 .filter_map(|(index, (x, y))| {
//                     if y >= plane.num_vertices {
//                         None
//                     } else if x >= plane.num_vertices {
//                         None
//                     } else {
//                         Some([
//                             [
//                                 index as u32,
//                                 index as u32 + 1 + 1 + plane.num_vertices,
//                                 index as u32 + 1,
//                             ],
//                             [
//                                 index as u32,
//                                 index as u32 + 1 + plane.num_vertices,
//                                 index as u32 + plane.num_vertices + 1 + 1,
//                             ],
//                         ])
//                     }
//                 })
//                 .flatten()
//                 .flatten()
//                 .collect::<Vec<_>>(),
//         );
//
//         let mut positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
//         let mut uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();
//         let colors = plane.carve_maze(&mut positions, &mut uvs);
//         let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
//
//         let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//         mesh.set_indices(Some(indices));
//         mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
//         mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//         mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//         mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//         mesh
//     }
// }
//
// impl<'a> MazePlane<'a> {
//     fn carve_maze(&self, positions: &mut Vec<[f32; 3]>, _uvs: &mut Vec<[f32; 2]>) -> Vec<[f32; 4]> {
//         let maze_texture = self.image_assets.get(self.maze_handle).unwrap();
//         let colors: Vec<[f32; 4]> = [[0.3, 0.5, 0.3, 1.0]].repeat(positions.len());
//         positions
//             .iter_mut()
//             .enumerate()
//             .for_each(|(_index, [x, y, z])| {
//                 let x = (*x + self.extent / 2.) / self.extent;
//                 let z = (*z + self.extent / 2.) / self.extent;
//
//                 let x_index = (self.maze_pixel_per_row as f32 * x).floor() as u32 as usize;
//                 let z_index = (self.maze_pixel_per_row as f32 * z).floor() as u32 as usize;
//
//                 // colors.remove(index);
//                 // colors.insert(index, [x_index as f32 / self.maze_pixel_per_row as f32, z_index as f32 / self.maze_pixel_per_row as f32, 0.0, 1.0]);
//
//                 let pixel = z_index * self.maze_pixel_per_row + x_index;
//
//                 if let Some(data) = maze_texture.data.get(pixel * 4) {
//                     if data > &50 && *y > -0.1 {
//                         *y = *y - 0.3;
//                     }
//                 }
//             });
//
//         colors
//     }
// }
