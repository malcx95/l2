use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::messages::SoundEffect;
use crate::player::Player;
use crate::math::{Vec2, vec2};
use crate::food::Food;


const FOOD_CUT_STRIDE: usize = 4;
const MAX_AMOUNT_OF_FOOD: usize = 1000;


#[derive(Serialize, Deserialize, Clone)]
pub enum GameStage {
    Lobby,
    Running,
    Ended,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub food: Vec<Food>,
    pub stage: GameStage,
    pub game_timer: f32,
    pub player_leaderboard: Vec<u64>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            food: Vec::new(),
            stage: GameStage::Lobby,
            game_timer: constants::GAME_DURATION,
            player_leaderboard: Vec::new(),
        }
    }
    

    fn reset(&mut self) {
        self.food = Vec::new();
        self.stage = GameStage::Lobby;
        self.game_timer = constants::GAME_DURATION;
        self.player_leaderboard = Vec::new();
        for player in &mut self.players {
            player.reset();
        }
    }

    /**
     *  Updates the gamestate and returns
     *  (
     *  vec with player ids that got hit with bullets,
     *  vec with positions where powerups where picked up,
     *  vec with positions where lasers are fired
     *  )
     */
    pub fn update(&mut self, sound_effects: &mut Vec<SoundEffect>, delta: f32) {
        match self.stage {
            GameStage::Running => {
                for player in &mut self.players {
                    player.update(delta);
                }
                self.update_food(delta, sound_effects);
                self.handle_player_food(sound_effects);
                self.handle_player_collisions(sound_effects);
                self.game_timer -= delta;
                if self.game_timer <= 0.0 {
                    self.stage = GameStage::Ended;
                    sound_effects.push(SoundEffect::End);
                }
                self.update_leaderboard();
            },
            GameStage::Lobby => {
                for player in &self.players {
                    if player.input_start_game {
                        self.stage = GameStage::Running;
                        sound_effects.push(SoundEffect::Start);
                    }
                }
            },
            GameStage::Ended => {
                let mut should_reset = false;
                for player in &self.players {
                    if player.input_start_game {
                        should_reset = true;
                    }
                }
                if should_reset {
                    self.reset();
                }
            }
        }
    }

    pub fn update_leaderboard(&mut self) {
        let mut player_lengths: Vec<(u64, usize)> = self.players.iter().map(|p| (p.id, p.snake.len())).collect();

        player_lengths.sort_by(|a, b| b.1.cmp(&a.1));
        self.player_leaderboard = player_lengths.iter().map(|(id, _)| *id).collect();
    }

    fn handle_player_collisions(&mut self, sound_effects: &mut Vec<SoundEffect>) {
        let mut cut_player_indices = vec![];
        for i in 0..self.players.len() {
            for j in 0..self.players.len() {
                let player1 = &self.players[i];
                let player2 = &self.players[j];
                match player1.collides_with(&player2.snake) {
                    Some(index) => {
                        cut_player_indices.push((i, j, index));
                    }
                    None => {}
                }
            }
        }
        for (i, j, index) in cut_player_indices {
            let other_id = self.players[i].id;
            match self.players[j].try_cut(index, other_id) {
                None => {},
                Some(cut_segment_positions) => {
                    for position in cut_segment_positions
                        .iter().step_by(FOOD_CUT_STRIDE) {
                        if self.food.len() < MAX_AMOUNT_OF_FOOD {
                            self.food.push(Food::new(*position));
                        }
                    }
                    sound_effects.push(SoundEffect::Cut);
                }
            }
        }
    }

    fn handle_player_food(&mut self, sound_effects: &mut Vec<SoundEffect>) {
        let mut eaten_food_indices = vec![];
        for player in &mut self.players {
            for i in 0..self.food.len() {
                let food = &self.food[i];
                if food.collides_with(player.get_head_position()) {
                    if player.try_eat(food){
                        sound_effects.push(SoundEffect::Eat);
                        eaten_food_indices.push(i);
                    }
                }
            }
        }
        eaten_food_indices.sort();
        eaten_food_indices.dedup();
        for i in eaten_food_indices.iter().rev() {
            self.food.remove(*i);
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.clone());
    }

    pub fn get_player_by_id(&self, id: u64) -> Option<&Player> {
        for player in &self.players {
            if player.id == id {
                return Some(player);
            }
        }
        None
    }

    fn maybe_spawn_food(&mut self) {
        if self.food.len() < 10 {
            let x = rand::random::<f32>() * constants::WINDOW_SIZE;
            let y = rand::random::<f32>() * constants::WINDOW_SIZE;
            self.food.push(Food::new(vec2(x, y)));
        }
    }

    fn update_food(&mut self, delta: f32, sound_effects: &mut Vec<SoundEffect>) {
        self.maybe_spawn_food();
        for food in &mut self.food {
            food.update(delta, &self.players.iter().map(|p| &p.snake).collect(), sound_effects);
        }
    }
}
