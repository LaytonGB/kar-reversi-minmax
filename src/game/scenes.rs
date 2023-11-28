use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    highlight::PickHighlight,
    prelude::On,
};
use strum::IntoEnumIterator;

use crate::{bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty, player::Player};

use crate::game::{
    interactions::click_grid_square,
    structs::{
        BevyCurrentPlayer, BevyMenuContent, BevyPlayerScore, BevyReversi, BevySquare, PieceCounts,
    },
};

use super::{
    highlight_constants::{BUTTON_DEFAULT, DANGER_DEFAULT, GRID_HIGHLIGHT},
    structs::{
        BevyBotAlgorithm, BevyBotDifficulty, BevyGameConfig, BevyMetricsDisplay, BevyPlayButton,
    },
};

pub fn menu_setup(mut commands: Commands) {
    // camera
    let camera_entity = commands
        .spawn(Camera2dBundle {
            ..Default::default()
        })
        .id();
    // content
    let menu_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    padding: UiRect {
                        left: Val::ZERO,
                        right: Val::ZERO,
                        top: Val::ZERO,
                        bottom: Val::Px(150.0),
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Reversi",
                    TextStyle {
                        font: default(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });

            // difficulty
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Difficulty",
                    TextStyle {
                        font: default(),
                        font_size: 26.0,
                        color: Color::Hsla {
                            hue: 0.0,
                            saturation: 0.0,
                            lightness: 0.85,
                            alpha: 1.0,
                        },
                    },
                ),
                ..Default::default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let mut it = BotDifficulty::iter();
                    let mut next = it.next();
                    while let Some(difficulty) = next {
                        next = it.next();
                        parent
                            .spawn((
                                ButtonBundle {
                                    background_color: if next.is_some() {
                                        BackgroundColor(BUTTON_DEFAULT)
                                    } else {
                                        BackgroundColor(DANGER_DEFAULT)
                                    },
                                    style: Style {
                                        padding: UiRect::all(Val::Px(6.0)),
                                        margin: UiRect::all(Val::Px(6.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                BevyBotDifficulty(difficulty),
                            ))
                            .with_children(|btn| {
                                btn.spawn(TextBundle {
                                    text: Text::from_section(
                                        difficulty.to_string(),
                                        TextStyle {
                                            font: default(),
                                            font_size: 16.0,
                                            color: Color::Hsla {
                                                hue: 0.0,
                                                saturation: 0.0,
                                                lightness: 0.7,
                                                alpha: 1.0,
                                            },
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                    }
                });

            // algorithm
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Algorithm",
                        TextStyle {
                            font: default(),
                            font_size: 26.0,
                            color: Color::Hsla {
                                hue: 0.0,
                                saturation: 0.0,
                                lightness: 0.85,
                                alpha: 1.0,
                            },
                        },
                    ),
                    ..Default::default()
                }
                .with_style(Style {
                    margin: UiRect::top(Val::Px(36.0)),
                    ..Default::default()
                }),
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let mut it = BotAlgorithm::iter();
                    let mut next = it.next();
                    while let Some(algorithm) = next {
                        next = it.next();
                        parent
                            .spawn((
                                ButtonBundle {
                                    background_color: if next.is_some() {
                                        BackgroundColor(BUTTON_DEFAULT)
                                    } else {
                                        BackgroundColor(DANGER_DEFAULT)
                                    },
                                    style: Style {
                                        padding: UiRect::all(Val::Px(6.0)),
                                        margin: UiRect::all(Val::Px(6.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                BevyBotAlgorithm(algorithm),
                            ))
                            .with_children(|btn| {
                                btn.spawn(TextBundle {
                                    text: Text::from_section(
                                        algorithm.to_string(),
                                        TextStyle {
                                            font: default(),
                                            font_size: 16.0,
                                            color: Color::Hsla {
                                                hue: 0.0,
                                                saturation: 0.0,
                                                lightness: 0.7,
                                                alpha: 1.0,
                                            },
                                        },
                                    ),
                                    ..Default::default()
                                });
                            });
                    }
                });

            // confirm button
            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::BLACK),
                        style: Style {
                            padding: UiRect::all(Val::Px(6.0)),
                            margin: UiRect::top(Val::Px(36.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    BevyPlayButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "Play",
                                TextStyle {
                                    font: default(),
                                    font_size: 32.0,
                                    color: Color::Hsla {
                                        hue: 0.0,
                                        saturation: 0.0,
                                        lightness: 0.3,
                                        alpha: 1.0,
                                    },
                                },
                            ),
                            ..Default::default()
                        },
                        BevyPlayButton,
                    ));
                });
        })
        .id();
    commands.insert_resource(BevyMenuContent {
        camera: camera_entity,
        menu: menu_entity,

        config: BevyGameConfig::default(),
    })
}

pub fn menu_teardown(mut commands: Commands, menu_data: Res<BevyMenuContent>) {
    commands.entity(menu_data.camera).despawn_recursive();
    commands.entity(menu_data.menu).despawn_recursive();
    commands.remove_resource::<BevyMenuContent>();
}

pub fn board_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game: Res<BevyReversi>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // game board base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 2.0, 10.0).into()),
        material: materials.add(Color::DARK_GRAY.into()),
        ..default()
    });
    // game board grid
    for x in -4..4 {
        for z in -4..4 {
            let color = if (x % 2_i32).abs() == (z % 2_i32).abs() {
                Color::WHITE
            } else {
                Color::BLACK
            };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Box::new(1.0, 2.0, 1.0).into()),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.2, z as f32 + 0.5),
                    ..default()
                },
                BevySquare,
                On::<Pointer<Click>>::target_commands_mut(click_grid_square),
                PickHighlight,
                GRID_HIGHLIGHT,
            ));
        }
    }
    // background
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(30.0, 1.0, 30.0).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, -4.0),
        ..default()
    });
    // scores and current player
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                padding: UiRect {
                    left: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                    top: Val::Px(16.0),
                    bottom: Val::ZERO,
                },
                margin: UiRect::all(Val::Px(6.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Auto,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
            let text_style = TextStyle {
                font: default(),
                font_size: 24.0,
                color: Color::WHITE,
            };
            let mut piece_counts = PieceCounts { green: 0, red: 0 };
            for player in Player::iter() {
                piece_counts.set(player, game.0.board().pieces_for_player(player).count());
            }
            let parent_spawn_score_text =
                |parent: &mut ChildBuilder<'_, '_, '_>, player: Player| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{}: {}", player, piece_counts.get(player)),
                            text_style.clone(),
                        )
                        .with_text_alignment(TextAlignment::Center),
                        BevyPlayerScore {
                            player,
                            text_style: text_style.clone(),
                            piece_counts,
                        },
                    ));
                };
            parent_spawn_score_text(parent, Player::Green);
            parent.spawn((
                TextBundle::from_section(
                    format!("Turn: {}", game.0.current_player()),
                    text_style.clone(),
                )
                .with_text_alignment(TextAlignment::Center),
                BevyCurrentPlayer,
            ));
            parent_spawn_score_text(parent, Player::Red);
        });
    // algorithm metrics
    commands.spawn((
        NodeBundle {
            style: Style {
                left: Val::Px(36.0),
                height: Val::Percent(80.0),
                width: Val::Percent(20.0),
                padding: UiRect::all(Val::Px(12.0)),
                margin: UiRect::vertical(Val::Percent(5.0)),
                overflow: Overflow::clip(),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(16.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Hsla {
                hue: 0.0,
                saturation: 0.0,
                lightness: 0.0,
                alpha: 0.3,
            }),
            ..Default::default()
        },
        BevyMetricsDisplay::default(),
    ));
}
