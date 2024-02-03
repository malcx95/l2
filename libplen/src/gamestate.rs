use std::sync::mpsc::Receiver;

use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::player::Player;
use crate::math::{Vec2, vec2};
use crate::food::Food;


const FOOD_CUT_STRIDE: usize = 4;
const MAX_AMOUNT_OF_FOOD: usize = 1000;


#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub food: Vec<Food>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            food: Vec::new(),
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
    pub fn update(&mut self, delta: f32) {
        for player in &mut self.players {
            player.update(delta);
        }
        self.update_food(delta);
        self.handle_player_food();
        self.handle_player_collisions();
    }

    fn handle_player_collisions(&mut self) {
        let mut cut_player_indices = vec![];
        for i in 0..self.players.len() {
            for j in 0..self.players.len() {
                let player1 = &self.players[i];
                let player2 = &self.players[j];
                match player1.collides_with(&player2.snake) {
                    Some(index) => {
                        cut_player_indices.push((j, index));
                    }
                    None => {}
                }
            }
        }
        for (i, index) in cut_player_indices {
            let other_id = self.players[i].id;
            match self.players[i].try_cut(index, other_id) {
                None => {},
                Some(cut_segment_positions) => {
                    for position in cut_segment_positions
                        .iter().step_by(FOOD_CUT_STRIDE) {

                        if self.food.len() < MAX_AMOUNT_OF_FOOD {
                            self.food.push(Food::new(*position));
                        }
                    }
                }
            }
        }
    }

    fn handle_player_food(&mut self) {
        let mut eaten_food_indices = vec![];
        for player in &mut self.players {
            for i in 0..self.food.len() {
                let food = &self.food[i];
                if food.collides_with(player.get_head_position()) {
                    if player.try_eat(food){
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

    fn update_food(&mut self, delta: f32) {
        self.maybe_spawn_food();
        for food in &mut self.food {
            food.update(delta);
        }
    }
}
