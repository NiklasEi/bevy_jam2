use crate::loading::{MazeAssets, MazeMesh, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_mod_picking::{Highlighting, PickableBundle};

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
    maze: Res<MazeMesh>,
    maze_assets: Res<MazeAssets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let maze_texture = images.remove(&maze_assets.two).unwrap();
    const MAZE_PIXEL_PER_ROW: usize = 5;
    let maze_mash = meshes.get_mut(&maze.mesh).unwrap();
    let mut len = 0;
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        maze_mash.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        len = positions.len();
        positions.iter_mut().for_each(|[x, y, z]| {
            println!("x {}, z {}, y{}", x, z, y);
            let x = (*x + 1.) / 2.;
            let z = (*z + 1.) / 2.;

            let x_index = ((MAZE_PIXEL_PER_ROW - 1) as f32 * x) as u32 as usize;
            let z_index = ((MAZE_PIXEL_PER_ROW - 1) as f32 * z) as u32 as usize;

            let pixel = z_index * MAZE_PIXEL_PER_ROW + x_index;

            println!("x {}, z {}, y{} , pixel {}", x_index, z_index, y, pixel);
            if maze_texture.data.get(pixel * 4).unwrap() > &50 {
                if *y > -0.5 {
                    *y = *y - 1.;
                }
            }
        });
    }
    maze_mash.insert_attribute(Mesh::ATTRIBUTE_COLOR, [[0.3, 0.5, 0.3, 1.0]].repeat(len));
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: maze.mesh.clone(),
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
