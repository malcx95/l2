use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use serde_derive::{Serialize, Deserialize};
use crate::constants;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Self {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        vec * self
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f32) -> Self {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Vec2 {
    pub fn from_direction(angle: f32, length: f32) -> Self {
        Vec2 {
            x: angle.cos() * length,
            y: angle.sin() * length,
        }
    }

    pub fn norm(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vec2 {
        self / self.norm()
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn distance_to(self, other: Self) -> f32 {
        (self - other).norm()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn get_normal(self) -> Self {
        vec2(-self.y, self.x)
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}


pub fn modulo(x: f32, div: f32) -> f32 {
    (x % div + div) % div
}


pub fn vec_add_wrap_around(a: Vec2, b: Vec2, div: f32) -> Vec2 {
    let mut result = a + b;
    result.x = modulo(result.x, div);
    result.y = modulo(result.y, div);
    result
}


pub fn angle_diff(source_angle: f32, target_angle: f32) -> f32 {
    // From https://stackoverflow.com/a/7869457
    use std::f32::consts::PI;
    modulo(target_angle - source_angle + PI, 2. * PI) - PI
}
