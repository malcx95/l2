use libplen::constants;
use libplen::gamestate::GameState;
use libplen::math::{self, vec2, Vec2};
use libplen::player::Player;
use macroquad::math::vec2 as macroquad_vec2;
use macroquad::prelude::*;
use macroquad::texture;
use libplen::food::{Food, FoodType};

use crate::assets::Assets;


const MAX_SEGMENT_DISTANCE: f32 = 30.0;
const FOOD_SIZE: f32 = 5.0;
const PLAYER_MENU_SPACING: f32 = 80.0;
const PLAYER_MENU_Y: f32 = constants::WINDOW_SIZE - 100.0;

const COLORS: [macroquad::color::Color; 11] = [
    RED, GREEN, PURPLE, ORANGE, PINK, VIOLET, MAGENTA, LIME, BROWN, GOLD, WHITE
];


pub struct ClientState {
    screen_scale: f32,
}

impl ClientState {
    pub fn new() -> ClientState {
        let screen_scale = match std::env::var("SCREEN_SCALE") {
            Ok(val) => val.parse::<f32>().unwrap(),
            Err(_) => 1.0,
        };
        ClientState {
            screen_scale
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

        match game_state.stage {
            libplen::gamestate::GameStage::Lobby => {
                self.draw_menu(game_state, assets);
            }
            _ => {
                self.draw_players(&game_state.players, my_id);
                self.draw_food(&game_state.food);
            }
        }

        Ok(())
    }

    fn draw_menu(&self, game_state: &GameState, assets: &mut Assets) {
        let text1 = "välkommen till L2";
        let text2 = "tryck C för att byta färg, space för att starta";
        draw_text(
            text1,
            (constants::WINDOW_SIZE / 10.0) * self.screen_scale,
            (constants::WINDOW_SIZE / 2.0) * self.screen_scale,
            40.0 * self.screen_scale,
            WHITE,
        );
        draw_text(
            text2,
            (constants::WINDOW_SIZE / 10.0) * self.screen_scale,
            (constants::WINDOW_SIZE / 2.0 + 20.0) * self.screen_scale,
            24.0 * self.screen_scale,
            WHITE,
        );

        for (i, player) in game_state.players.iter().enumerate() {
            let color = COLORS[player.color % COLORS.len()];
            let text = &player.name;
            let text_size = measure_text(text, None, 32, 1.0);
            let x = (PLAYER_MENU_SPACING + text_size.width / 2.0) * (i as f32 + 1.0);
            draw_text(
                text,
                (x - text_size.width / 2.0) * self.screen_scale,
                (PLAYER_MENU_Y - text_size.height / 2.0) * self.screen_scale,
                32.0 * self.screen_scale,
                color
            );

            draw_circle(x * self.screen_scale, (PLAYER_MENU_Y + 30.0) * self.screen_scale,
                10.0 * self.screen_scale, color);
        }
    }

    fn draw_players(&self, players: &Vec<Player>, my_id: u64) {
        for player in players {
            let color = COLORS[player.color % COLORS.len()];

            let head_px = player.snake.segments[0].position.x;
            let head_py = player.snake.segments[0].position.y;

            draw_circle(head_px * self.screen_scale, head_py * self.screen_scale,
                5.0 * self.screen_scale, color);

            let body_color = Color::new(color.r, color.g, color.b, 0.9);

            for i in 0..(player.snake.segments.len() - 1) {
                let curr = &player.snake.segments[i];
                let next = &player.snake.segments[i + 1];

                if (curr.position - next.position).norm() > MAX_SEGMENT_DISTANCE {
                    continue;
                }
                
                draw_line(
                    curr.position.x * self.screen_scale,
                    curr.position.y * self.screen_scale,
                    next.position.x * self.screen_scale,
                    next.position.y * self.screen_scale,
                    5.0 * self.screen_scale,
                    body_color,
                );

                if !next.cuttable {
                    let uncuttable_color = Color::new(1.0, 1.0, 1.0, 0.4);
                    
                    draw_line(
                        curr.position.x * self.screen_scale,
                        curr.position.y * self.screen_scale,
                        next.position.x * self.screen_scale,
                        next.position.y * self.screen_scale,
                        10.0 * self.screen_scale,
                        uncuttable_color,
                    );
                    
                }

            }
        }
    }

    fn draw_bounds(&self) {
        draw_rectangle_lines(
            0.0,
            0.0,
            constants::WINDOW_SIZE * self.screen_scale,
            constants::WINDOW_SIZE * self.screen_scale,
            5.0 * self.screen_scale,
            WHITE,
        );
    }

    fn draw_food(&self, food: &Vec<Food>) {
        for f in food {
            match f.food_type {
                FoodType::Normal(_) => draw_circle(
                    f.position.x * self.screen_scale, 
                    f.position.y * self.screen_scale,
                    FOOD_SIZE * self.screen_scale, 
                    YELLOW),
                FoodType::Armor(_) => {
                    let v1: Vec2 = f.position+vec2( 0.00, -1.00);
                    let v2: Vec2 = f.position+vec2( 0.87,  0.50);
                    let v3: Vec2 = f.position+vec2(-0.87,  0.50);
                    draw_triangle(
                        FOOD_SIZE * macroquad_vec2(v1.x, v1.y) * self.screen_scale, 
                        FOOD_SIZE * macroquad_vec2(v2.x, v2.y) * self.screen_scale, 
                        FOOD_SIZE * macroquad_vec2(v3.x, v3.y) * self.screen_scale,
                        BLUE)
                },
            };
        }
    }
}
