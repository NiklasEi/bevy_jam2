use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Notification>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_timer)
                    .with_system(spawn_notification),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_timer)
                    .with_system(update_notification),
            );
    }
}

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct NotificationText;

#[derive(Component)]
struct NotificationTextBox;

struct GameStopWatch(Stopwatch);

impl Default for GameStopWatch {
    fn default() -> Self {
        GameStopWatch(Stopwatch::new())
    }
}

fn spawn_timer(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(GameStopWatch::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
                alpha: 0.7,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "00:00".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb_u8(34, 32, 52),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(TimerText);
        });
}

fn spawn_notification(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(97.), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(NotificationTextBox)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb_u8(34, 32, 52),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(NotificationText);
        });
}

#[derive(Default)]
pub struct Notification {
    pub remove_when: Option<f64>,
    pub text: Option<String>,
}

fn update_notification(
    mut notification: ResMut<Notification>,
    time: Res<Time>,
    mut notification_text: Query<&mut Text, With<NotificationText>>,
    mut notification_text_box: Query<
        &mut UiColor,
        (With<NotificationTextBox>, Without<NotificationText>),
    >,
) {
    if let Some(remove_when) = notification.remove_when {
        if remove_when < time.seconds_since_startup() {
            notification.text = None;
            notification.remove_when = None;
            notification_text_box.single_mut().0 = Color::NONE;
        }
    }
    if !notification.is_changed() {
        return;
    }
    if let Some(text) = &notification.text {
        notification_text.single_mut().sections[0].value = text.clone();
        notification_text_box.single_mut().0 = Color::Rgba {
            red: 0.7,
            green: 0.7,
            blue: 0.7,
            alpha: 0.7,
        };
    } else {
        notification_text.single_mut().sections[0].value = "".to_string();
        notification_text_box.single_mut().0 = Color::NONE;
    }
}

fn update_timer(
    time: Res<Time>,
    mut game_stop_watch: ResMut<GameStopWatch>,
    mut timer_text: Query<&mut Text, With<TimerText>>,
) {
    game_stop_watch.0.tick(time.delta());
    let score = game_stop_watch.0.elapsed_secs();
    let minutes = (score / 60.).floor();
    let seconds = (score % 60.).floor();
    timer_text.single_mut().sections[0].value = format!("{:0>2}:{:0>2}", minutes, seconds);
}
