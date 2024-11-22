use crate::game::internal::brighten;
use crate::game::internal::DefaultColor;
use crate::game::textcolor_smoothing::{TargetTextColor, TextColorSmoothing};
use crate::utils::buttoning::{ButtonClickEvent, ButtonClickType, Clickable, Hovered, TextButton};
use crate::utils::position_smoothing::{
    PSmoothing, PositionSmoothing, TargetPosition, DEFAULT_END_THRESHOLD,
};
use crate::utils::responsive::{ResponsivePosition, ResponsiveValue};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::sprite::Anchor;

///
/// Requires [utils::position_smoothing::PositionSmoothingPlugin]
///
#[derive(Default)]
pub struct GameLogicPlugin;

#[derive(Default, Resource)]
pub struct GameData {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Exiting,
}

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.init_state::<GameState>();
        app.enable_state_scoped_entities::<GameState>();
        main_menu::configure_app(app);
    }
}

mod main_menu {
    use super::*;
    use std::time::Instant;
    use crate::game::internal;
    use crate::game::internal::ScriptControlled;
    use crate::utils::buttoning::GenericBoundingBox;

    pub fn configure_app(app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu::on_enter);
        app.add_systems(Update, (hovered_texts, menu_button_clicks));
        app.observe(unhovered_texts);
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Component)]
    pub enum MenuButton {
        Play,
        Quit,
        Settings,
    }

    #[derive(Component)]
    pub struct MenuController {
        pub title: Entity,
        pub play_button: Entity,
        pub quit_button: Entity,
    }

    pub fn spawn_camera(commands: &mut Commands) {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                projection: OrthographicProjection {
                    far: 1000.,
                    near: -1000.,
                    viewport_origin: Vec2::new(0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            BloomSettings::default(),
            StateScoped(GameState::MainMenu),
        ));
    }

    pub fn spawn_title(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                StateScoped(GameState::MainMenu),
                Text2dBundle {
                    text: Text::from_section(
                        "Secrets of the Path",
                        TextStyle {
                            color: Color::srgb(1.4, 2.1, 2.4),
                            font_size: 60.0,
                            font: Handle::default(),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(640.0, 1000.0, 1.0),
                    text_anchor: Anchor::Center,
                    ..default()
                },
                PositionSmoothing::P(PSmoothing {
                    modifier: 2.5,
                    end_threshold: DEFAULT_END_THRESHOLD,
                }),
                TargetPosition::default(),
                ResponsivePosition::new(
                    ResponsiveValue::Percentage(50.0),
                    ResponsiveValue::Percentage(90.0),
                    ResponsiveValue::Absolute(0.0),
                ),
            ))
            .id()
    }

    pub fn spawn_buttons(commands: &mut Commands) -> (Entity, Entity) {
        (
            commands
                .spawn((
                    StateScoped(GameState::MainMenu),
                    Text2dBundle {
                        text: Text::from_section(
                            "Play",
                            TextStyle {
                                color: Color::srgb(1.4, 2.1, 1.4),
                                font_size: 30.0,
                                font: Handle::default(),
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(640.0, -250.0, 1.0),
                        text_anchor: Anchor::Center,
                        ..default()
                    },
                    PositionSmoothing::P(PSmoothing {
                        modifier: 2.3,
                        end_threshold: DEFAULT_END_THRESHOLD,
                    }),
                    TargetPosition(Vec3::new(640.0, 0.0, 0.0)),
                    ResponsivePosition::new(
                        ResponsiveValue::Percentage(50.0),
                        ResponsiveValue::Percentage(50.0),
                        ResponsiveValue::Absolute(0.0),
                    ),
                    DefaultColor(Color::srgb(1.4, 2.1, 1.4)),
                    TargetTextColor(Color::srgb(1.4, 2.1, 1.4)),
                    TextColorSmoothing::default(),
                    TextButton,
                    MenuButton::Play,
                ))
                .id(),
            commands
                .spawn((
                    StateScoped(GameState::MainMenu),
                    Text2dBundle {
                        text: Text::from_section(
                            "Quit",
                            TextStyle {
                                color: Color::srgb(2.1, 1.4, 1.4),
                                font_size: 30.0,
                                font: Handle::default(),
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(640.0, -250.0, 1.0),
                        text_anchor: Anchor::Center,
                        ..default()
                    },
                    PositionSmoothing::P(PSmoothing {
                        modifier: 2.1,
                        end_threshold: DEFAULT_END_THRESHOLD,
                    }),
                    TargetPosition(Vec3::new(640.0, 0.0, 0.0)),
                    ResponsivePosition::new(
                        ResponsiveValue::Percentage(50.0),
                        ResponsiveValue::Percentage(40.0),
                        ResponsiveValue::Absolute(0.0),
                    ),
                    DefaultColor(Color::srgb(2.1, 1.4, 1.4)),
                    TargetTextColor(Color::srgb(2.1, 1.4, 1.4)),
                    TextColorSmoothing::default(),
                    TextButton,
                    MenuButton::Quit,
                ))
                .id(),
        )
    }

    pub fn on_enter(mut commands: Commands) {
        spawn_camera(&mut commands);
        let title_e = spawn_title(&mut commands);
        let (play_btn_e, quit_btn_e) = spawn_buttons(&mut commands);

        commands.spawn(MenuController {
            title: title_e,
            play_button: play_btn_e,
            quit_button: quit_btn_e,
        });
    }

    fn menu_button_clicks(
        query: Query<(&MenuButton), (With<Hovered>, With<Clickable>, Without<ScriptControlled>)>,
        mut event_reader: EventReader<ButtonClickEvent>,
        menu_controller_q: Query<&MenuController>,
        mut commands: Commands,
    ) {
        if menu_controller_q.is_empty() { return; }
        let menu_controller = menu_controller_q.single();

        for event in event_reader.read() {
            if event.mouse_button == MouseButton::Left
                && event.click_type == ButtonClickType::Pressed
            {
                if let Ok(menu_button) = query.get(event.entity) {
                    match menu_button {
                        MenuButton::Play => info!("Play Button Clicked"),
                        MenuButton::Quit => {
                            info!("Quit Button Clicked");
                            commands.entity(menu_controller.quit_button).add(|mut e: EntityWorldMut| {
                                e.insert(ScriptControlled::default());
                                let newcolor = e.get::<DefaultColor>().map(|dc| dc.0).unwrap_or(Color::srgb(2.1, 1.4, 1.4));
                                if let Some(mut ttc) = e.get_mut::<TargetTextColor>() {
                                    ttc.0 = brighten(newcolor, 64.0);
                                }
                                if let Some(mut ts) = e.get_mut::<TextColorSmoothing>() {
                                    ts.proportional = 0.35;
                                }
                            });
                        }
                        MenuButton::Settings => info!("Settings Button Clicked"),
                    }
                }
            }
        }
    }

    fn hovered_texts(
        mut query: Query<(&mut TargetTextColor, &DefaultColor), (Added<Hovered>, (With<MenuButton>, Without<ScriptControlled>))>,
    ) {
        for (mut target_color, default_color) in query.iter_mut() {
            target_color.0 = brighten(default_color.0, 2.0);
        }
    }

    fn unhovered_texts(
        trigger: Trigger<OnRemove, Hovered>,
        mut query: Query<(&mut TargetTextColor, &DefaultColor), (With<MenuButton>, Without<ScriptControlled>)>,
    ) {
        if let Ok((mut target_color, default_color)) = query.get_mut(trigger.entity()) {
            target_color.0 = default_color.0;
        }
    }
}
