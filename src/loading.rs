use crate::GameState;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
                .init_resource::<MazeMesh>()
                .continue_to_state(GameState::Menu),
        );
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
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    pub green: ColorStandardMaterial<80, 125, 80, { u8::MAX }>,
    pub red: ColorStandardMaterial<{ u8::MAX }, 0, 0, { u8::MAX }>,
    pub blue: ColorStandardMaterial<0, 0, { u8::MAX }, { u8::MAX }>,
}

#[derive(AssetCollection)]
pub struct MazeAssets {
    #[asset(path = "plane.gltf#Scene0")]
    pub scene: Handle<Scene>,
    #[asset(path = "mazes/1.png")]
    pub one: Handle<Image>,
    #[asset(path = "mazes/2.png")]
    pub two: Handle<Image>,
}

pub struct MazeMesh {
    pub mesh: Handle<Mesh>,
}

impl FromWorld for MazeMesh {
    fn from_world(world: &mut World) -> Self {
        let scene_handle = world.resource::<MazeAssets>().scene.clone();
        let mut scene = world
            .resource_mut::<Assets<Scene>>()
            .remove(scene_handle)
            .unwrap();
        let mut system_state: SystemState<(Query<&Handle<Mesh>>,)> =
            SystemState::new(&mut scene.world);
        let mesh_handle_query = system_state.get(&mut scene.world);

        MazeMesh {
            mesh: mesh_handle_query.0.single().clone(),
        }
    }
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
