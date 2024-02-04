use serde_derive::{Serialize, Deserialize};
use crate::math::{Vec2, vec2, vec_add_wrap_around};
use crate::constants;
use crate::snake::Snake;
use rand::{self, Rng};


const FOOD_SPEED: f32 = 100.0;
const FOOD_SIZE: f32 = 10.0;
const FOOD_ENERGY: u32 = 10;

const ARMOR_ENERGY: usize = 10;
const ARMOR_PROBABILITY: f32 = 0.5;


#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum FoodType {
    Normal(u32),
    Armor(usize),
}


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Food {
    pub position: Vec2,
    pub velocity: Vec2,
    pub food_type: FoodType,
}


impl Food {
    pub fn new(position: Vec2) -> Food {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-1.0, 1.0) * FOOD_SPEED;
        let y: f32 = rng.gen_range(-1.0, 1.0) * FOOD_SPEED;

        let r = rand::random::<f32>();
        let food_type = if r < ARMOR_PROBABILITY {
            FoodType::Armor(ARMOR_ENERGY)
        } else {
            FoodType::Normal(FOOD_ENERGY)
        };

        Food {
            position,
            velocity: vec2(x, y),
            food_type,
        }
    }

    pub fn update(&mut self, delta_time: f32, snakes: &Vec<&Snake>) {
        for snake in snakes {
            match snake.get_collision_reflection(self.position, self.velocity) {
                Some(reflection) => {
                    self.velocity = reflection;
                }
                None => {}
            }

        }
        self.position = vec_add_wrap_around(self.position, self.velocity * delta_time, constants::WINDOW_SIZE);
    }

    pub fn collides_with(&self, position: Vec2) -> bool {
        (self.position - position).norm() < FOOD_SIZE
    }
}
