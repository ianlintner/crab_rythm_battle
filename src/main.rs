use bevy::prelude::*;

mod battle;
mod beat;
mod chart;
mod constants;
mod crab;
mod input;
mod notes;
mod scoring;
mod states;
mod ui;

use states::GameState;
use beat::BeatResource;
use battle::BattleResource;
use notes::NoteChart;
use scoring::ScoreResource;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crab Rhythm Battle".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // State
        .init_state::<GameState>()
        // Game resources
        .init_resource::<BeatResource>()
        .init_resource::<BattleResource>()
        .init_resource::<NoteChart>()
        .init_resource::<ScoreResource>()
        // Plugins
        .add_plugins((
            beat::BeatPlugin,
            battle::BattlePlugin,
            notes::NotesPlugin,
            input::InputPlugin,
            crab::CrabPlugin,
            ui::UIPlugin,
        ))
        // Scene setup on enter Playing
        .add_systems(OnEnter(GameState::Playing), (setup_scene, reset_game_state))
        .add_systems(OnExit(GameState::Playing), cleanup_scene)
        // Menu navigation
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        // Result screen navigation
        .add_systems(
            Update,
            result_input.run_if(in_state(GameState::Victory).or_else(in_state(GameState::Defeat))),
        )
        .run();
}

// ─── Scene cleanup marker ─────────────────────────────────────────────────────

#[derive(Component)]
struct SceneEntity;

// ─── Scene setup ──────────────────────────────────────────────────────────────

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera – angled down from behind player, looking toward opponent
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, -8.0)
                .looking_at(Vec3::new(0.0, 0.0, 10.0), Vec3::Y),
            ..default()
        },
        SceneEntity,
    ));

    // Directional light (sun from above)
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8000.0,
                color: Color::rgb(1.0, 0.95, 0.8),
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::FRAC_PI_4,
                std::f32::consts::FRAC_PI_4,
                0.0,
            )),
            ..default()
        },
        SceneEntity,
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.4, 0.6, 0.8),
        brightness: 150.0,
    });

    // Ocean floor plane
    let floor_mesh = meshes.add(Cuboid::new(30.0, 0.2, 30.0));
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.25, 0.45),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: floor_mesh,
            material: floor_material,
            transform: Transform::from_xyz(0.0, -0.5, 10.0),
            ..default()
        },
        SceneEntity,
    ));

    // Lane colors
    let lane_colors = [
        Color::rgba(0.2, 0.4, 1.0, 0.4),
        Color::rgba(0.2, 0.9, 0.2, 0.4),
        Color::rgba(1.0, 0.6, 0.1, 0.4),
        Color::rgba(1.0, 0.2, 0.2, 0.4),
    ];
    let lane_x = constants::LANE_X;

    // Lane rail meshes
    let rail_mesh = meshes.add(Cuboid::new(0.6, 0.05, constants::LANE_LENGTH));

    for (i, &x) in lane_x.iter().enumerate() {
        let mat = materials.add(StandardMaterial {
            base_color: lane_colors[i],
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: rail_mesh.clone(),
                material: mat,
                transform: Transform::from_xyz(x, 0.0, constants::LANE_LENGTH / 2.0),
                ..default()
            },
            SceneEntity,
        ));
    }

    // Hit zone indicators – glowing disc at z=0 for each lane
    let hit_zone_mesh = meshes.add(Cuboid::new(0.7, 0.1, 0.3));
    for (i, &x) in lane_x.iter().enumerate() {
        let glow_color = match i {
            0 => Color::rgb(0.3, 0.5, 1.0),
            1 => Color::rgb(0.3, 1.0, 0.3),
            2 => Color::rgb(1.0, 0.7, 0.2),
            _ => Color::rgb(1.0, 0.3, 0.3),
        };
        let mat = materials.add(StandardMaterial {
            base_color: glow_color,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: hit_zone_mesh.clone(),
                material: mat,
                transform: Transform::from_xyz(x, 0.1, 0.0),
                ..default()
            },
            SceneEntity,
        ));
    }

    // Point light near hit zone for glow effect
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                color: Color::rgb(0.5, 0.8, 1.0),
                range: 8.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        SceneEntity,
    ));

    // Try loading music (gracefully ignore if missing)
    let audio_path = "audio/beat.wav";
    if std::path::Path::new("assets/audio/beat.wav").exists() {
        commands.spawn((
            AudioBundle {
                source: asset_server.load(audio_path),
                settings: PlaybackSettings::LOOP,
            },
            SceneEntity,
        ));
    }
}

fn cleanup_scene(mut commands: Commands, query: Query<Entity, With<SceneEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Reset ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 80.0,
    });
}

// ─── Reset game state when entering Playing ───────────────────────────────────

fn reset_game_state(
    mut beat: ResMut<BeatResource>,
    mut battle: ResMut<BattleResource>,
    mut score: ResMut<ScoreResource>,
) {
    beat.reset();
    battle.reset();
    score.reset();
}

// ─── Menu input ───────────────────────────────────────────────────────────────

fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

// ─── Result input ─────────────────────────────────────────────────────────────

fn result_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
