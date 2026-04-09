use bevy::prelude::*;
use crate::constants::{PERFECT_SCORE, GOOD_SCORE};

#[derive(Resource, Default)]
pub struct ScoreResource {
    pub score: u32,
    pub combo: u32,
    pub max_combo: u32,
    pub perfects: u32,
    pub goods: u32,
    pub misses: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HitTiming {
    Perfect,
    Good,
}

impl ScoreResource {
    pub fn reset(&mut self) {
        self.score = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.perfects = 0;
        self.goods = 0;
        self.misses = 0;
    }

    pub fn record_hit(&mut self, timing: HitTiming) {
        self.combo += 1;
        if self.combo > self.max_combo {
            self.max_combo = self.combo;
        }

        let base_score = match timing {
            HitTiming::Perfect => {
                self.perfects += 1;
                PERFECT_SCORE
            }
            HitTiming::Good => {
                self.goods += 1;
                GOOD_SCORE
            }
        };

        // Combo multiplier (every 10 combo adds 0.5x, capped at 4x)
        let multiplier = (1.0 + (self.combo / 10) as f32 * 0.5).min(4.0);
        self.score += (base_score as f32 * multiplier) as u32;
    }

    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.combo = 0;
    }
}
