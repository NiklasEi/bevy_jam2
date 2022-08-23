use crate::loading::{MazeAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_mod_picking::{Highlighting, PickableBundle};
use itertools::Itertools;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<MazeMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map));
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "641bb9cf-2a23-46f8-aa66-91dd79655018"]
pub struct MazeMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material for MazeMaterial {
    fn vertex_shader() -> ShaderRef {
        "mazes/maze_shader.wgsl".into()
    }
}

#[derive(Component)]
pub struct Map;

fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<MazeMaterial>>,
    maze_assets: Res<MazeAssets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(
                MazePlane {
                    extent: 2.5,
                    num_vertices: 200,
                    maze_pixel_per_row: 19,
                    maze_handle: &maze_assets.one,
                    image_assets: &mut images,
                }
                .into(),
            ),
            material: materials.add(MazeMaterial { time: 0. }),
            transform: Transform::from_scale(Vec3::splat(2.5)),
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

#[derive(Debug, Copy, Clone)]
struct MazePlane<'a> {
    extent: f32,
    num_vertices: u32,
    maze_pixel_per_row: usize,
    maze_handle: &'a Handle<Image>,
    image_assets: &'a Assets<Image>,
}

impl<'a> From<MazePlane<'a>> for Mesh {
    fn from(plane: MazePlane) -> Self {
        let diff = plane.extent / plane.num_vertices as f32;

        let vertices = (0..=plane.num_vertices)
            .cartesian_product(0..=plane.num_vertices)
            .map(|(y, x)| {
                (
                    [
                        x as f32 * diff - 0.5 * plane.extent,
                        0.0,
                        y as f32 * diff - 0.5 * plane.extent,
                    ],
                    [0.0, 1.0, 0.0],
                    [
                        x as f32 / plane.num_vertices as f32,
                        y as f32 / plane.num_vertices as f32,
                    ],
                )
            })
            .collect::<Vec<_>>();

        let indices = Indices::U32(
            (0..=plane.num_vertices)
                .cartesian_product(0..=plane.num_vertices)
                .enumerate()
                .filter_map(|(index, (x, y))| {
                    if y >= plane.num_vertices {
                        None
                    } else if x >= plane.num_vertices {
                        None
                    } else {
                        Some([
                            [
                                index as u32,
                                index as u32 + 1 + 1 + plane.num_vertices,
                                index as u32 + 1,
                            ],
                            [
                                index as u32,
                                index as u32 + 1 + plane.num_vertices,
                                index as u32 + plane.num_vertices + 1 + 1,
                            ],
                        ])
                    }
                })
                .flatten()
                .flatten()
                .collect::<Vec<_>>(),
        );

        let mut positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let colors = plane.carve_maze(&mut positions);
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

impl<'a> MazePlane<'a> {
    fn carve_maze(&self, positions: &mut Vec<[f32; 3]>) -> Vec<[f32; 4]> {
        let maze_texture = self.image_assets.get(self.maze_handle).unwrap();
        let mut colors: Vec<[f32; 4]> = [[0.3, 0.5, 0.3, 1.0]].repeat(positions.len());
        positions
            .iter_mut()
            .enumerate()
            .for_each(|(index, [x, y, z])| {
                let x = (*x + self.extent / 2.) / self.extent;
                let z = (*z + self.extent / 2.) / self.extent;

                let x_index = (self.maze_pixel_per_row as f32 * x).floor() as u32 as usize;
                let z_index = (self.maze_pixel_per_row as f32 * z).floor() as u32 as usize;

                // colors.remove(index);
                // colors.insert(index, [x_index as f32 / self.maze_pixel_per_row as f32, z_index as f32 / self.maze_pixel_per_row as f32, 0.0, 1.0]);

                let pixel = z_index * self.maze_pixel_per_row + x_index;

                if let Some(data) = maze_texture.data.get(pixel * 4) {
                    if data > &50 && *y > -0.5 {
                        *y = *y - 1.;
                    }
                }
            });

        colors
    }
}
