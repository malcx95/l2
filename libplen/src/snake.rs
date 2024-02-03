use serde_derive::{Serialize, Deserialize};
use crate::{constants, math::{self, vec2, Vec2}};

const FOOD_HIT_BOX: f32 = 5.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct SnakeSegment {
    pub position: Vec2,
    pub angle: f32,
    pub cuttable: bool,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Snake {
    pub segments: Vec<SnakeSegment>,
}


impl Snake {
    pub fn new() -> Snake {
        Snake {
            segments: vec![
                SnakeSegment {
                    position: vec2(0., 0.),
                    angle: 0.,
                    cuttable: true,
                }
            ],
        }
    }

    pub fn update(&mut self, delta_angle: f32, delta_time: f32, speed: f32) {
        let head = &mut self.segments[0];
        let delta_position = Vec2::from_direction(head.angle, speed * delta_time);
        let mut old_angle = head.angle;
        let mut old_position = head.position;

        head.angle += delta_angle;
        head.position += delta_position;
        head.position = math::vec_add_wrap_around(head.position, delta_position, constants::WINDOW_SIZE);

        for i in 1..self.segments.len() {
            let current = &mut self.segments[i];

            let current_angle = current.angle;
            let current_position = current.position.clone();

            current.angle = old_angle;
            current.position = old_position;

            old_angle = current_angle;
            old_position = current_position;
        }
    }

    pub fn get_collision_reflection(&self, pos: Vec2, velocity: Vec2) -> Option<Vec2> {
        for i in 0..(self.segments.len() - 1) {
            let curr_pos = self.segments[i].position;
            let next_pos = self.segments[i + 1].position;
            if (curr_pos - pos).norm() < FOOD_HIT_BOX {
                let tangent = (next_pos - curr_pos).normalize();
                let normal = tangent.get_normal();

                return Some(velocity.reflect(normal));
            }
        }
        None
    }

    pub fn get_first_cuttable_index(&self) -> Option<usize> {
        for (i, segment) in self.segments.iter().enumerate() {
            if segment.cuttable {
                return Some(i);
            }
        }
        None
    }
}
