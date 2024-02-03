use std::sync::mpsc::Receiver;

use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::player::Player;
use crate::math::{Vec2, vec2};
use crate::food::Food;


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
    }

    pub fn handle_player_food(&mut self) {
        let mut eaten_food_indices = vec![];
        for player in &mut self.players {
            for i in 0..self.food.len() {
                let food = &self.food[i];
                if food.collides_with(player.get_head_position()) {
                    player.eat(food);
                    eaten_food_indices.push(i);
                }
            }
        }
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
