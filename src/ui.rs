use bevy::prelude::*;
use crate::states::GameState;
use crate::battle::{BattleResource, HitResult};
use crate::scoring::ScoreResource;
use crate::constants::STARTING_HP;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Menu
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            // Playing HUD
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_hud)
            // Victory
            .add_systems(OnEnter(GameState::Victory), setup_result_screen)
            .add_systems(OnExit(GameState::Victory), cleanup_result_screen)
            // Defeat
            .add_systems(OnEnter(GameState::Defeat), setup_result_screen)
            .add_systems(OnExit(GameState::Defeat), cleanup_result_screen);
    }
}

// ─── Marker components ───────────────────────────────────────────────────────

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct HudUI;

#[derive(Component)]
struct ResultUI;

#[derive(Component)]
struct PlayerHpBar;

#[derive(Component)]
struct OpponentHpBar;

#[derive(Component)]
struct ComboText;

#[derive(Component)]
struct HitFeedbackText;

#[derive(Component)]
struct ScoreText;

// ─── Menu ─────────────────────────────────────────────────────────────────────

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.05, 0.15, 0.95)),
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "CRAB RHYTHM BATTLE",
                TextStyle {
                    font_size: 72.0,
                    color: Color::rgb(0.0, 0.9, 1.0),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }));

            // Subtitle / crab emoji text
            parent.spawn(TextBundle::from_section(
                "~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~",
                TextStyle {
                    font_size: 28.0,
                    color: Color::rgb(0.4, 0.8, 1.0),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }));

            // Press Enter
            parent.spawn(TextBundle::from_section(
                "Press ENTER or SPACE to Start",
                TextStyle {
                    font_size: 36.0,
                    color: Color::rgb(1.0, 1.0, 0.3),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }));

            // Controls header
            parent.spawn(TextBundle::from_section(
                "CONTROLS",
                TextStyle {
                    font_size: 28.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            }));

            // Controls row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    let keys = [
                        ("A", Color::rgb(0.2, 0.4, 1.0), "Left Claw"),
                        ("S", Color::rgb(0.2, 0.9, 0.2), "Left Leg"),
                        ("D", Color::rgb(1.0, 0.6, 0.1), "Right Leg"),
                        ("F", Color::rgb(1.0, 0.2, 0.2), "Right Claw"),
                    ];
                    for (key, color, label) in keys {
                        row.spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        }).with_children(|col| {
                            // Colored dot
                            col.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(color),
                                ..default()
                            });
                            // Key
                            col.spawn(TextBundle::from_section(
                                key,
                                TextStyle {
                                    font_size: 26.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                            // Label
                            col.spawn(TextBundle::from_section(
                                label,
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::rgb(0.7, 0.7, 0.7),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ─── HUD ──────────────────────────────────────────────────────────────────────

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            HudUI,
        ))
        .with_children(|root| {
            // ── Top bar: HP + combo ─────────────────────────────────────────
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            }).with_children(|top| {
                // Player HP (left)
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(220.0),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|col| {
                    col.spawn(TextBundle::from_section(
                        "PLAYER",
                        TextStyle { font_size: 18.0, color: Color::rgb(0.0, 0.8, 1.0), ..default() },
                    ));
                    // HP bar background
                    col.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                        ..default()
                    }).with_children(|bar_bg| {
                        // HP bar fill
                        bar_bg.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.0, 0.6, 1.0)),
                                ..default()
                            },
                            PlayerHpBar,
                        ));
                    });
                });

                // Center: combo + hit feedback
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|center| {
                    center.spawn((
                        TextBundle::from_section(
                            "COMBO x0",
                            TextStyle { font_size: 28.0, color: Color::rgb(1.0, 1.0, 0.3), ..default() },
                        ),
                        ComboText,
                    ));
                    center.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle { font_size: 36.0, color: Color::WHITE, ..default() },
                        ),
                        HitFeedbackText,
                    ));
                });

                // Opponent HP (right)
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        width: Val::Px(220.0),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|col| {
                    col.spawn(TextBundle::from_section(
                        "OPPONENT",
                        TextStyle { font_size: 18.0, color: Color::rgb(1.0, 0.4, 0.1), ..default() },
                    ));
                    col.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                        ..default()
                    }).with_children(|bar_bg| {
                        bar_bg.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(1.0, 0.3, 0.1)),
                                ..default()
                            },
                            OpponentHpBar,
                        ));
                    });
                });
            });

            // ── Spacer ──────────────────────────────────────────────────────
            root.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            });

            // ── Bottom: score ───────────────────────────────────────────────
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            }).with_children(|bottom| {
                bottom.spawn((
                    TextBundle::from_section(
                        "SCORE: 0",
                        TextStyle { font_size: 24.0, color: Color::rgb(0.9, 0.9, 0.9), ..default() },
                    ),
                    ScoreText,
                ));
            });
        });
}

fn update_hud(
    battle: Res<BattleResource>,
    score: Res<ScoreResource>,
    mut player_hp_query: Query<&mut Style, (With<PlayerHpBar>, Without<OpponentHpBar>)>,
    mut opponent_hp_query: Query<&mut Style, (With<OpponentHpBar>, Without<PlayerHpBar>)>,
    mut combo_query: Query<&mut Text, (With<ComboText>, Without<HitFeedbackText>, Without<ScoreText>)>,
    mut feedback_query: Query<&mut Text, (With<HitFeedbackText>, Without<ComboText>, Without<ScoreText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<ComboText>, Without<HitFeedbackText>)>,
) {
    // Player HP bar
    if let Ok(mut style) = player_hp_query.get_single_mut() {
        let pct = (battle.player_hp / STARTING_HP * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(pct);
    }

    // Opponent HP bar
    if let Ok(mut style) = opponent_hp_query.get_single_mut() {
        let pct = (battle.opponent_hp / STARTING_HP * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(pct);
    }

    // Combo text
    if let Ok(mut text) = combo_query.get_single_mut() {
        text.sections[0].value = if score.combo > 0 {
            format!("COMBO x{}", score.combo)
        } else {
            String::new()
        };
    }

    // Hit feedback
    if let Ok(mut text) = feedback_query.get_single_mut() {
        if battle.hit_flash_timer > 0.0 {
            let (label, color) = match battle.last_hit {
                Some(HitResult::Perfect) => ("PERFECT!", Color::rgb(1.0, 1.0, 0.2)),
                Some(HitResult::Good) => ("GOOD!", Color::rgb(0.3, 1.0, 0.3)),
                Some(HitResult::Miss) => ("MISS...", Color::rgb(1.0, 0.3, 0.3)),
                None => ("", Color::WHITE),
            };
            text.sections[0].value = label.to_string();
            text.sections[0].style.color = color.with_a(battle.hit_flash_timer.min(1.0));
        } else {
            text.sections[0].value = String::new();
        }
    }

    // Score
    if let Ok(mut text) = score_query.get_single_mut() {
        text.sections[0].value = format!("SCORE: {}", score.score);
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ─── Result screen ────────────────────────────────────────────────────────────

fn setup_result_screen(
    mut commands: Commands,
    state: Res<State<GameState>>,
    score: Res<ScoreResource>,
) {
    let (title, title_color) = match state.get() {
        GameState::Victory => ("VICTORY!", Color::rgb(1.0, 1.0, 0.2)),
        _ => ("DEFEAT...", Color::rgb(1.0, 0.3, 0.3)),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.1, 0.85)),
                ..default()
            },
            ResultUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle { font_size: 80.0, color: title_color, ..default() },
            ));

            parent.spawn(TextBundle::from_section(
                format!("Score: {}", score.score),
                TextStyle { font_size: 40.0, color: Color::WHITE, ..default() },
            ));

            parent.spawn(TextBundle::from_section(
                format!("Max Combo: {}", score.max_combo),
                TextStyle { font_size: 30.0, color: Color::rgb(0.9, 0.9, 0.9), ..default() },
            ));

            parent.spawn(TextBundle::from_section(
                format!(
                    "Perfects: {}   Goods: {}   Misses: {}",
                    score.perfects, score.goods, score.misses
                ),
                TextStyle { font_size: 24.0, color: Color::rgb(0.7, 0.7, 0.7), ..default() },
            ));

            parent.spawn(TextBundle::from_section(
                "Press ENTER to play again  |  ESC for menu",
                TextStyle { font_size: 28.0, color: Color::rgb(1.0, 1.0, 0.4), ..default() },
            ));
        });
}

fn cleanup_result_screen(mut commands: Commands, query: Query<Entity, With<ResultUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
