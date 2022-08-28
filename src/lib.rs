mod actions;
mod audio;
mod character;
mod in_game_menu;
mod loading;
mod map;
mod menu;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use crate::actions::ActionPlugin;
use crate::character::CharacterPlugin;
use crate::in_game_menu::InGameMenuPlugin;
use crate::map::MapPlugin;
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
    InGameMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugins(DefaultPickingPlugins)
            .add_plugin(AtmospherePlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(InGameMenuPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(ActionPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(WorldInspectorPlugin::new())
                .add_plugin(WireframePlugin);
        }
    }
}
