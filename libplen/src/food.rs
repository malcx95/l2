use serde_derive::{Serialize, Deserialize};
use crate::math::{Vec2, vec2, vec_add_wrap_around};
use crate::constants;
use rand::{self, Rng};


const FOOD_SPEED: f32 = 100.0;
const FOOD_SIZE: f32 = 10.0;
const FOOD_ENERGY: u32 = 10;


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Food {
    pub position: Vec2,
    pub velocity: Vec2,
    pub energy: u32,
}


impl Food {
    pub fn new(position: Vec2) -> Food {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0, 1.0) * FOOD_SPEED;
        let y: f32 = rng.gen_range(-1.0, 1.0) * FOOD_SPEED;
        Food {
            position,
            velocity: vec2(x, y),
            energy: FOOD_ENERGY,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position = vec_add_wrap_around(self.position, self.velocity * delta_time, constants::WINDOW_SIZE);
    }

    pub fn collides_with(&self, position: Vec2) -> bool {
        (self.position - position).norm() < FOOD_SIZE
    }
}
