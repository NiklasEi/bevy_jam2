use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<MazeAssets>()
                .with_collection::<LabyrinthTextures>()
                .init_resource::<LabyrinthMaterials>()
                .continue_to_state(GameState::Menu),
        )
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(make_markers_opaque))
        .add_plugin(RonAssetPlugin::<LabyrinthLevel>::new(&["ron.level"]));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/grass.jpg", standard_material)]
    pub grass: Handle<StandardMaterial>,
    pub green: ColorStandardMaterial<80, 125, 80, { u8::MAX }>,
    pub red: ColorStandardMaterial<{ u8::MAX }, 0, 0, { u8::MAX }>,
    pub blue: ColorStandardMaterial<0, 0, { u8::MAX }, { u8::MAX }>,
    #[asset(path = "textures/green_marker_mask.png", standard_material)]
    pub green_marker_mask: Handle<StandardMaterial>,
    #[asset(path = "textures/green_marker.png", standard_material)]
    pub green_marker: Handle<StandardMaterial>,
    #[asset(path = "textures/red_marker_mask.png", standard_material)]
    pub red_marker_mask: Handle<StandardMaterial>,
    #[asset(path = "textures/red_marker.png", standard_material)]
    pub red_marker: Handle<StandardMaterial>,
    #[asset(path = "textures/blue_marker_mask.png", standard_material)]
    pub blue_marker_mask: Handle<StandardMaterial>,
    #[asset(path = "textures/blue_marker.png", standard_material)]
    pub blue_marker: Handle<StandardMaterial>,
}

fn make_markers_opaque(
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<TextureAssets>,
) {
    materials
        .get_mut(&textures.green_marker_mask)
        .unwrap()
        .alpha_mode = AlphaMode::Opaque;
    materials
        .get_mut(&textures.green_marker)
        .unwrap()
        .alpha_mode = AlphaMode::Opaque;
    materials
        .get_mut(&textures.red_marker_mask)
        .unwrap()
        .alpha_mode = AlphaMode::Opaque;
    materials.get_mut(&textures.red_marker).unwrap().alpha_mode = AlphaMode::Opaque;
    materials
        .get_mut(&textures.blue_marker_mask)
        .unwrap()
        .alpha_mode = AlphaMode::Opaque;
    materials.get_mut(&textures.blue_marker).unwrap().alpha_mode = AlphaMode::Opaque;
}

impl TextureAssets {
    pub fn get_character_texture(&self, character: u8) -> Handle<StandardMaterial> {
        match character {
            1 => self.green.handle.clone(),
            2 => self.red.handle.clone(),
            _ => self.blue.handle.clone(),
        }
    }
}

#[derive(AssetCollection)]
pub struct LabyrinthTextures {
    #[asset(path = "textures/wall/ambientOcclusion.jpg")]
    pub wall_ambient_occlusion: Handle<Image>,
    #[asset(path = "textures/wall/baseColor.jpg")]
    pub wall_base_color: Handle<Image>,
    #[asset(path = "textures/wall/normal.jpg")]
    pub wall_normal: Handle<Image>,
    #[asset(path = "textures/wall/roughness.jpg")]
    pub wall_roughness: Handle<Image>,
    #[asset(path = "textures/ground/ao.jpg")]
    pub ground_ambient_occlusion: Handle<Image>,
    #[asset(path = "textures/ground/diffuse.jpg")]
    pub ground_base_color: Handle<Image>,
    #[asset(path = "textures/ground/normal.jpg")]
    pub ground_normal: Handle<Image>,
    #[asset(path = "textures/ground/specular.jpg")]
    pub ground_roughness: Handle<Image>,
}

pub struct LabyrinthMaterials {
    pub wall: Handle<StandardMaterial>,
    pub ground: Handle<StandardMaterial>,
}

impl FromWorld for LabyrinthMaterials {
    fn from_world(world: &mut World) -> Self {
        let handles = world.remove_resource::<LabyrinthTextures>().unwrap();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        LabyrinthMaterials {
            wall: materials.add(StandardMaterial {
                base_color_texture: Some(handles.wall_base_color),
                normal_map_texture: Some(handles.wall_normal),
                metallic_roughness_texture: Some(handles.wall_roughness),
                occlusion_texture: Some(handles.wall_ambient_occlusion),
                ..default()
            }),
            ground: materials.add(StandardMaterial {
                base_color_texture: Some(handles.ground_base_color),
                normal_map_texture: Some(handles.ground_normal),
                metallic_roughness_texture: Some(handles.ground_roughness),
                occlusion_texture: Some(handles.ground_ambient_occlusion),
                ..default()
            }),
        }
    }
}

#[derive(AssetCollection)]
pub struct MazeAssets {
    #[asset(path = "mazes/1.png")]
    pub one_data: Handle<Image>,
    #[asset(path = "mazes/1.ron.level")]
    pub one_level: Handle<LabyrinthLevel>,
    #[asset(path = "mazes/2.png")]
    pub two_data: Handle<Image>,
}

pub struct ColorStandardMaterial<const R: u8, const G: u8, const B: u8, const A: u8> {
    pub handle: Handle<StandardMaterial>,
}

impl<const R: u8, const G: u8, const B: u8, const A: u8> FromWorld
    for ColorStandardMaterial<R, G, B, A>
{
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        ColorStandardMaterial {
            handle: materials.add(StandardMaterial::from(Color::rgba_u8(R, G, B, A))),
        }
    }
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "84f362c3-62e0-cac3-73c8-7e013e8049f5"]
pub struct LabyrinthLevel {
    pub spawns: Vec<[f32; 2]>,
    pub exit: [usize; 2],
}
