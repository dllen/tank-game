use super::{Position, Velocity};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Bullet {
    pub position: Position,
    pub velocity: Velocity,
    pub damage: i32,
    pub size: f32,
    pub color: Color,
    pub from_player: bool,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32, from_player: bool) -> Self {
        let speed = 300.0;
        Self {
            position: Position::new(x, y),
            velocity: Velocity::from_angle(angle, speed),
            damage: 25,
            size: 3.0,
            color: if from_player { YELLOW } else { ORANGE },
            from_player,
            lifetime: 0.0,
            max_lifetime: 3.0,
        }
    }
    
    pub fn new_scatter(x: f32, y: f32, angle: f32, spread: f32, from_player: bool) -> Self {
        let speed = 250.0;
        let actual_angle = angle + spread;
        Self {
            position: Position::new(x, y),
            velocity: Velocity::from_angle(actual_angle, speed),
            damage: 15,
            size: 2.5,
            color: if from_player { GOLD } else { ORANGE },
            from_player,
            lifetime: 0.0,
            max_lifetime: 2.5,
        }
    }
    
    pub fn update(&mut self, dt: f32) -> bool {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.lifetime += dt;
        
        // 检查是否超出屏幕边界或生命周期结束
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        self.position.x >= 0.0
            && self.position.x <= screen_width
            && self.position.y >= 0.0
            && self.position.y <= screen_height
            && self.lifetime < self.max_lifetime
    }
    
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
    }
    
    pub fn collides_with_circle(&self, pos: &Position, radius: f32) -> bool {
        self.position.distance_to(pos) < self.size + radius
    }
}