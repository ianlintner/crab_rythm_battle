use bevy::prelude::*;
use crate::states::GameState;
use crate::constants::STARTING_HP;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleResource>()
            .add_systems(Update, (
                check_win_lose_conditions,
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
}

impl Default for BattleResource {
    fn default() -> Self {
        BattleResource {
            player_hp: STARTING_HP,
            opponent_hp: STARTING_HP,
            last_hit: None,
            hit_flash_timer: 0.0,
            game_over: false,
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
