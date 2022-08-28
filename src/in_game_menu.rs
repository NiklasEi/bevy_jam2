use crate::loading::FontAssets;
use crate::menu::ButtonColors;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(open_menu))
            .add_system_set(SystemSet::on_enter(GameState::InGameMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::InGameMenu)
                    .with_system(click_continue_button)
                    .with_system(click_quit_button)
                    .with_system(close_menu),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGameMenu).with_system(cleanup_menu));
    }
}

#[derive(Component)]
struct InGameMenuElement;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct QuitButton;

fn open_menu(
    mut states: ResMut<State<GameState>>,
    mut input: ResMut<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {
    if input.just_pressed(KeyCode::Escape) {
        input.clear();
        if let Some(window) = windows.get_primary_mut() {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }
        states.push(GameState::InGameMenu).unwrap();
    }
}

fn close_menu(mut states: ResMut<State<GameState>>, mut input: ResMut<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        states.pop().unwrap();
        input.clear();
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .insert(QuitButton)
        .insert(InGameMenuElement)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Quit".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .insert(ContinueButton)
        .insert(InGameMenuElement)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Continue".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn click_quit_button(
    button_colors: Res<ButtonColors>,
    mut exit_events: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                exit_events.send(AppExit);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn click_continue_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ContinueButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.pop().unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_elements: Query<Entity, With<InGameMenuElement>>) {
    for entity in &menu_elements {
        commands.entity(entity).despawn_recursive();
    }
}
