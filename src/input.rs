use bevy::prelude::*;
use crate::states::GameState;
use crate::constants::*;
use crate::notes::Note;
use crate::battle::{BattleResource, HitResult};
use crate::scoring::{ScoreResource, HitTiming};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input.run_if(in_state(GameState::Playing)));
    }
}

fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut battle: ResMut<BattleResource>,
    mut score: ResMut<ScoreResource>,
    notes: Query<(Entity, &Note, &Transform)>,
) {
    let lane_keys: [(usize, &[KeyCode]); 4] = [
        (0, &[KeyCode::KeyA, KeyCode::ArrowLeft]),
        (1, &[KeyCode::KeyS, KeyCode::ArrowDown]),
        (2, &[KeyCode::KeyD, KeyCode::ArrowUp]),
        (3, &[KeyCode::KeyF, KeyCode::ArrowRight]),
    ];

    for (lane, keys) in lane_keys {
        let pressed = keys.iter().any(|k| keyboard.just_pressed(*k));
        if pressed {
            // Find the closest note in this lane within GOOD_WINDOW
            let mut best_entity: Option<Entity> = None;
            let mut best_dist = f32::MAX;

            for (entity, note, transform) in notes.iter() {
                if note.lane != lane || note.missed {
                    continue;
                }
                // z position indicates distance to hit zone (z=0 is perfect)
                let z = transform.translation.z;
                let dist = z.abs();

                if dist <= GOOD_WINDOW && dist < best_dist {
                    best_dist = dist;
                    best_entity = Some(entity);
                }
            }

            if let Some(entity) = best_entity {
                // Determine hit quality
                let timing = if best_dist <= PERFECT_WINDOW {
                    HitTiming::Perfect
                } else {
                    HitTiming::Good
                };

                let (opponent_damage, hit_result) = match timing {
                    HitTiming::Perfect => (PERFECT_OPPONENT_DAMAGE, HitResult::Perfect),
                    HitTiming::Good => (GOOD_OPPONENT_DAMAGE, HitResult::Good),
                };

                score.record_hit(timing);
                battle.damage_opponent(opponent_damage, hit_result);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
