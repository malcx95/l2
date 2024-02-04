use enum_map::{enum_map, EnumMap};

use macroquad::{audio::Sound, texture::*};
use pollster::block_on;

pub struct Assets {
    pub malcolm: Texture2D,
    pub welcome: Sound,
    pub eat: Sound,
    pub cut: Sound,
    pub food_bounce: Sound,
    pub start: Sound,
    pub end: Sound,
}

impl Assets {
    pub fn new() -> Assets {
        let assets = Assets {
            malcolm: Texture2D::from_file_with_format(include_bytes!("../resources/malcolm.png"), None),
            welcome: block_on(macroquad::audio::load_sound("resources/audio/welcome.ogg")).unwrap(),
            eat: block_on(macroquad::audio::load_sound("resources/audio/plopp.ogg")).unwrap(),
            cut: block_on(macroquad::audio::load_sound("resources/audio/splat.ogg")).unwrap(),
            food_bounce: block_on(macroquad::audio::load_sound("resources/audio/tick.ogg")).unwrap(),
            start: block_on(macroquad::audio::load_sound("resources/audio/goodluck.ogg")).unwrap(),
            end: block_on(macroquad::audio::load_sound("resources/audio/end.ogg")).unwrap(),
        };
        assets
    }
}
