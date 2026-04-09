use bevy::prelude::*;
use crate::states::GameState;
use crate::constants::*;
use crate::chart::{NoteEvent, build_chart};
use crate::beat::BeatResource;
use crate::battle::BattleResource;
use crate::scoring::ScoreResource;

pub struct NotesPlugin;

impl Plugin for NotesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoteChart>()
            .add_systems(OnEnter(GameState::Playing), setup_note_resources)
            .add_systems(Update, (
                spawn_notes,
                move_notes,
                mark_missed_notes,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct NoteChart {
    pub events: Vec<NoteEvent>,
    pub next_idx: usize,
}

impl NoteChart {
    pub fn reset(&mut self) {
        self.events = build_chart();
        self.next_idx = 0;
    }
}

#[derive(Component)]
pub struct Note {
    pub lane: usize,
    pub hit_time: f64,
    pub missed: bool,
}

#[derive(Resource)]
pub struct NoteAssets {
    pub mesh: Handle<Mesh>,
    pub materials: [Handle<StandardMaterial>; 4],
    pub missed_material: Handle<StandardMaterial>,
}

pub fn setup_note_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut note_chart: ResMut<NoteChart>,
) {
    let mesh = meshes.add(Cuboid::new(NOTE_WIDTH, NOTE_HEIGHT, NOTE_DEPTH));

    let lane_colors = [
        Color::rgb(0.2, 0.4, 1.0),  // Lane 0: blue
        Color::rgb(0.2, 0.9, 0.2),  // Lane 1: green
        Color::rgb(1.0, 0.6, 0.1),  // Lane 2: orange
        Color::rgb(1.0, 0.2, 0.2),  // Lane 3: red
    ];

    let materials_arr = lane_colors.map(|color| {
        materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })
    });

    let missed_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.4),
        ..default()
    });

    commands.insert_resource(NoteAssets {
        mesh,
        materials: materials_arr,
        missed_material,
    });

    note_chart.reset();
}

pub fn spawn_notes(
    mut commands: Commands,
    mut chart: ResMut<NoteChart>,
    beat: Res<BeatResource>,
    note_assets: Option<Res<NoteAssets>>,
) {
    let Some(assets) = note_assets else { return };

    while chart.next_idx < chart.events.len() {
        let event = &chart.events[chart.next_idx];
        let spawn_time = event.hit_time - APPROACH_TIME;
        if beat.song_time >= spawn_time {
            let lane = event.lane;
            let hit_time = event.hit_time;
            let x = LANE_X[lane];
            // Spawn at the far end (opponent's position, z=20)
            let z = LANE_LENGTH;

            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh.clone(),
                    material: assets.materials[lane].clone(),
                    transform: Transform::from_xyz(x, 0.3, z),
                    ..default()
                },
                Note {
                    lane,
                    hit_time,
                    missed: false,
                },
            ));

            chart.next_idx += 1;
        } else {
            break;
        }
    }
}

pub fn move_notes(
    beat: Res<BeatResource>,
    mut query: Query<(&Note, &mut Transform)>,
) {
    for (note, mut transform) in query.iter_mut() {
        if !note.missed {
            // z goes from LANE_LENGTH (far) toward 0 (player)
            let time_to_hit = (note.hit_time - beat.song_time) as f32;
            transform.translation.z = time_to_hit * NOTE_SPEED;
        }
    }
}

pub fn mark_missed_notes(
    mut commands: Commands,
    beat: Res<BeatResource>,
    mut battle: ResMut<BattleResource>,
    mut score: ResMut<ScoreResource>,
    note_assets: Option<Res<NoteAssets>>,
    mut query: Query<(Entity, &mut Note, &mut Handle<StandardMaterial>)>,
) {
    let Some(assets) = note_assets else { return };

    for (entity, mut note, mut material) in query.iter_mut() {
        if note.missed {
            continue;
        }
        // Check if note has passed the hit zone by more than GOOD_WINDOW + 0.5
        let time_to_hit = (note.hit_time - beat.song_time) as f32;
        let z_pos = time_to_hit * NOTE_SPEED;

        if z_pos < -(GOOD_WINDOW + 0.5) {
            note.missed = true;
            *material = assets.missed_material.clone();
            battle.damage_player(MISS_PLAYER_DAMAGE);
            score.record_miss();
            // Despawn missed note
            commands.entity(entity).despawn_recursive();
        }
    }
}
