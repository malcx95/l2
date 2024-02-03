use serde_derive::{Serialize, Deserialize};

use crate::math::{Vec2, vec2};
use crate::snake::{Snake, SnakeSegment};
use crate::food::{Food, FoodType::*};


const PLAYER_ANGLE_SPEED: f32 = 5.0;
const PLAYER_MIN_SPEED: f32 = 100.0;
const PLAYER_MAX_SPEED: f32 = 300.0;
const PLAYER_ACCELERATION: f32 = 10.0;
const NUMBER_OF_NON_COLLIDABLE_SEGMENTS: usize = 6;
const EAT_GRACE_PERIOD: i32 = 10;


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub color: usize,
    pub name: String,

    pub input_x: f32,
    pub input_y: f32,
    pub input_start_game: bool,
    pub input_change_color: bool,

    pub snake: Snake,
    pub player_speed: f32,

    pub eat_grace_timer: i32,
}


impl Player {
    pub fn new(
        id: u64,
        name: String
    ) -> Player {
        Player {
            id,
            name,
            color: id as usize,

            input_x: 0.,
            input_y: 0.,
            input_start_game: false,
            input_change_color: false,

            snake: Snake::new(),
            player_speed: PLAYER_MIN_SPEED,

            eat_grace_timer: 0,
        }
    }

    pub fn try_cut(&mut self, index: usize, other_id: u64) -> Option<Vec<Vec2>> {
        let segment = &self.snake.segments[index];
        if !segment.cuttable && self.id != other_id {
            return None;
        }

        let mut cut_segment_positions = vec![];
        for segment in self.snake.segments.drain(index..) {
            cut_segment_positions.push(segment.position);
        }
        self.eat_grace_timer = EAT_GRACE_PERIOD;
        Some(cut_segment_positions)
    }

    pub fn collides_with(&self, other: &Snake) -> Option<usize> {
        for (i, other_segment) in other.segments.iter().enumerate().skip(NUMBER_OF_NON_COLLIDABLE_SEGMENTS) {
            if (self.get_head_position() - other_segment.position).norm() < 5.0 {
                return Some(i);
            }
        }

        None
    }

    pub fn try_eat(&mut self, food: &Food) -> bool {
        if self.eat_grace_timer > 0 {
            return false;
        }
        match food.food_type {
            Normal(energy) => {
                for _ in 0..energy {
                    self.snake.segments.push(SnakeSegment {
                        position: self.snake.segments.last().unwrap().position,
                        angle: self.snake.segments.last().unwrap().angle,
                        cuttable: true,
                    });
                }
            },
            Armor(energy) => {
                match self.snake.get_first_cuttable_index() {
                    Some(first_cuttable_index) => {
                        for i in first_cuttable_index..(first_cuttable_index + energy).min(self.snake.segments.len()) {
                            self.snake.segments[i].cuttable = false;
                        }
                    }
                    None => {}
                }
            },
        }
        true
    }

    pub fn get_head_position(&self) -> Vec2 {
        self.snake.segments[0].position
    }

    pub fn set_input(&mut self, input_x: f32, input_y: f32, input_start_game: bool, input_change_color: bool) {
        self.input_x = input_x;
        self.input_y = input_y;
        self.input_start_game = input_start_game;
        self.input_change_color = input_change_color;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.eat_grace_timer = i32::max(0, self.eat_grace_timer - 1);
        let delta_angle = self.input_x * PLAYER_ANGLE_SPEED * delta_time;
        
        self.player_speed = f32::clamp(
            self.player_speed + self.input_y * PLAYER_ACCELERATION,
            PLAYER_MIN_SPEED, PLAYER_MAX_SPEED);

        self.snake.update(delta_angle, delta_time, self.player_speed);
    }
}
