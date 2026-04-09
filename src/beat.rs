use bevy::prelude::*;
use crate::states::GameState;
use crate::constants::BEAT_INTERVAL;

pub struct BeatPlugin;

impl Plugin for BeatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BeatResource>()
            .add_systems(Update, update_beat.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct BeatResource {
    pub song_time: f64,
    pub is_playing: bool,
    pub beat_pulse: f32,
}

impl BeatResource {
    pub fn reset(&mut self) {
        self.song_time = 0.0;
        self.is_playing = true;
        self.beat_pulse = 0.0;
    }
}

fn update_beat(
    time: Res<Time>,
    mut beat: ResMut<BeatResource>,
) {
    if beat.is_playing {
        beat.song_time += time.delta_seconds_f64();

        // Sawtooth wave synced to beat: goes from 1.0 at beat start to 0.0 at next beat
        let beat_phase = (beat.song_time / BEAT_INTERVAL).fract() as f32;
        beat.beat_pulse = 1.0 - beat_phase;
    }
}
