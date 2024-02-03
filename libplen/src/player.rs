use serde_derive::{Serialize, Deserialize};

use crate::math::{Vec2, vec2};
use crate::snake::{Snake, SnakeSegment};
use crate::food::Food;


const PLAYER_ANGLE_SPEED: f32 = 5.0;
const NUMBER_OF_NON_COLLIDABLE_SEGMENTS: usize = 30;
const EAT_GRACE_PERIOD: i32 = 10;


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,

    pub input_x: f32,
    pub input_y: f32,

    pub snake: Snake,

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

            input_x: 0.,
            input_y: 0.,

            snake: Snake::new(),
            eat_grace_timer: 0,
        }
    }

    pub fn cut(&mut self, index: usize) -> Vec<Vec2> {
        let mut cut_segment_positions = vec![];
        for segment in self.snake.segments.drain(index..) {
            cut_segment_positions.push(segment.position);
        }
        self.eat_grace_timer = EAT_GRACE_PERIOD;
        cut_segment_positions
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
        for _ in 0..food.energy {
            self.snake.segments.push(SnakeSegment {
                position: self.snake.segments.last().unwrap().position,
                angle: self.snake.segments.last().unwrap().angle,
            });
        }
        true
    }

    pub fn get_head_position(&self) -> Vec2 {
        self.snake.segments[0].position
    }

    pub fn set_input(&mut self, input_x: f32, input_y: f32) {
        self.input_x = input_x;
        self.input_y = input_y;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.eat_grace_timer = i32::max(0, self.eat_grace_timer - 1);
        let delta_angle = self.input_x * PLAYER_ANGLE_SPEED * delta_time;
        self.snake.update(delta_angle, delta_time);
    }
}
