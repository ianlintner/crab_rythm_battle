use bevy::prelude::*;
use crate::states::GameState;
use crate::constants::STARTING_HP;
use crate::beat::BeatResource;
use crate::notes::NoteChart;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleResource>()
            .add_systems(Update, (
                check_win_lose_conditions,
                check_song_complete,
                update_hit_flash_timer,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HitResult {
    Perfect,
    Good,
    Miss,
}

#[derive(Resource)]
pub struct BattleResource {
    pub player_hp: f32,
    pub opponent_hp: f32,
    pub last_hit: Option<HitResult>,
    pub hit_flash_timer: f32,
    pub game_over: bool,
    pub song_complete_timer: Option<f32>,
}

impl Default for BattleResource {
    fn default() -> Self {
        BattleResource {
            player_hp: STARTING_HP,
            opponent_hp: STARTING_HP,
            last_hit: None,
            hit_flash_timer: 0.0,
            game_over: false,
            song_complete_timer: None,
        }
    }
}

impl BattleResource {
    pub fn reset(&mut self) {
        self.player_hp = STARTING_HP;
        self.opponent_hp = STARTING_HP;
        self.last_hit = None;
        self.hit_flash_timer = 0.0;
        self.game_over = false;
        self.song_complete_timer = None;
    }

    pub fn damage_player(&mut self, amount: f32) {
        self.player_hp = (self.player_hp - amount).max(0.0);
        self.last_hit = Some(HitResult::Miss);
        self.hit_flash_timer = 0.5;
    }

    pub fn damage_opponent(&mut self, amount: f32, result: HitResult) {
        self.opponent_hp = (self.opponent_hp - amount).max(0.0);
        self.last_hit = Some(result);
        self.hit_flash_timer = 0.3;
    }
}

fn check_win_lose_conditions(
    mut battle: ResMut<BattleResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if battle.game_over {
        return;
    }
    if battle.player_hp <= 0.0 {
        battle.game_over = true;
        next_state.set(GameState::Defeat);
    } else if battle.opponent_hp <= 0.0 {
        battle.game_over = true;
        next_state.set(GameState::Victory);
    }
}

fn check_song_complete(
    chart: Res<NoteChart>,
    beat: Res<BeatResource>,
    time: Res<Time>,
    mut battle: ResMut<BattleResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if battle.game_over || chart.events.is_empty() {
        return;
    }

    let last_note_time = chart.events.last().unwrap().hit_time;
    // Wait 3 seconds after the last note's hit time for all notes to resolve
    if beat.song_time >= last_note_time + 3.0 {
        match &mut battle.song_complete_timer {
            Some(timer) => {
                *timer -= time.delta_seconds();
                if *timer <= 0.0 {
                    battle.game_over = true;
                    if battle.player_hp >= battle.opponent_hp {
                        next_state.set(GameState::Victory);
                    } else {
                        next_state.set(GameState::Defeat);
                    }
                }
            }
            None => {
                battle.song_complete_timer = Some(2.0);
            }
        }
    }
}

fn update_hit_flash_timer(
    time: Res<Time>,
    mut battle: ResMut<BattleResource>,
) {
    if battle.hit_flash_timer > 0.0 {
        battle.hit_flash_timer -= time.delta_seconds();
        if battle.hit_flash_timer <= 0.0 {
            battle.hit_flash_timer = 0.0;
            battle.last_hit = None;
        }
    }
}
