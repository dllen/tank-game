pub mod tank;
pub mod bullet;
pub mod obstacle;
pub mod powerup;
pub mod enemy;

pub use tank::*;
pub use bullet::*;
pub use obstacle::*;
pub use powerup::*;
pub use enemy::*;

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn from_angle(angle: f32, speed: f32) -> Self {
        Self {
            x: angle.cos() * speed,
            y: angle.sin() * speed,
        }
    }
}