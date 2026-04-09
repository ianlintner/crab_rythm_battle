pub const BPM: f64 = 90.0;
pub const BEAT_INTERVAL: f64 = 60.0 / BPM; // seconds per beat
pub const SIXTEENTH_NOTE: f64 = BEAT_INTERVAL / 4.0; // seconds per 16th note

pub const APPROACH_TIME: f64 = 2.0; // seconds for note to travel full lane
pub const NOTE_SPEED: f32 = 10.0; // units per second
pub const LANE_LENGTH: f32 = 20.0; // total lane length in units

pub const PERFECT_WINDOW: f32 = 0.5; // z distance for perfect hit
pub const GOOD_WINDOW: f32 = 1.5; // z distance for good hit

pub const LANE_X: [f32; 4] = [-4.5, -1.5, 1.5, 4.5];

// Damage values
pub const PERFECT_OPPONENT_DAMAGE: f32 = 12.0;
pub const GOOD_OPPONENT_DAMAGE: f32 = 6.0;
pub const MISS_PLAYER_DAMAGE: f32 = 15.0;

// HP values
pub const STARTING_HP: f32 = 100.0;

// Note visual sizes
pub const NOTE_WIDTH: f32 = 0.8;
pub const NOTE_HEIGHT: f32 = 0.2;
pub const NOTE_DEPTH: f32 = 0.4;

// Scoring
pub const PERFECT_SCORE: u32 = 100;
pub const GOOD_SCORE: u32 = 50;
