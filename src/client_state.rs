use libplen::constants;
use libplen::gamestate::GameState;
use libplen::math::{self, vec2, Vec2};
use macroquad::prelude::*;
use macroquad::texture;
use libplen::food::Food;

use crate::assets::Assets;

pub struct ClientState {
    // add client side state
}

impl ClientState {
    pub fn new() -> ClientState {
        ClientState {
            // init client stuff
        }
    }

    pub fn update(&mut self, delta_time: f32, game_state: &GameState, my_id: u64) {
        // update client side stuff
    }

    pub fn draw(
        &self,
        my_id: u64,
        game_state: &GameState,
        assets: &mut Assets,
    ) -> Result<(), String> {

        clear_background(BLACK);
        self.draw_bounds();

        for player in &game_state.players {
            let head_px = player.snake.segments[0].position.x;
            let head_py = player.snake.segments[0].position.y;

            draw_circle(head_px, head_py, 5.0, RED);

            for i in 0..(player.snake.segments.len() - 1) {
                let curr = &player.snake.segments[i];
                let next = &player.snake.segments[i + 1];
                
                draw_line(
                    curr.position.x,
                    curr.position.y,
                    next.position.x,
                    next.position.y,
                    5.0,
                    RED,
                );

            }
        }
        self.draw_food(&game_state.food);

        Ok(())
    }

    fn draw_bounds(&self) {
        draw_rectangle_lines(
            0.0,
            0.0,
            constants::WINDOW_SIZE,
            constants::WINDOW_SIZE,
            5.0,
            WHITE,
        );
    }

    fn draw_food(&self, food: &Vec<Food>) {
        for f in food {
            draw_circle(f.position.x, f.position.y, 5.0, YELLOW);
        }
    }
}
