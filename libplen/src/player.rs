use serde_derive::{Serialize, Deserialize};

use crate::math::{Vec2, vec2};

use crate::snake::{Snake, SnakeSegment};


const PLAYER_ANGLE_SPEED: f32 = 5.0;


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,

    pub input_x: f32,
    pub input_y: f32,

    pub snake: Snake,
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
        }
    }

    pub fn set_input(&mut self, input_x: f32, input_y: f32) {
        self.input_x = input_x;
        self.input_y = input_y;
    }

    pub fn update(&mut self, delta_time: f32) {
        let delta_angle = self.input_x * PLAYER_ANGLE_SPEED * delta_time;
        self.snake.update(delta_angle, delta_time);
    }
}
