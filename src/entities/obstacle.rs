use super::Position;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Obstacle {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub health: i32,
    pub max_health: i32,
    pub destructible: bool,
}

impl Obstacle {
    pub fn new_wall(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 100,
            max_health: 100,
            destructible: true,
        }
    }
    
    pub fn new_steel(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1000,
            max_health: 1000,
            destructible: false,
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if !self.destructible {
            return false;
        }
        
        self.health -= damage;
        self.health <= 0
    }
    
    #[allow(dead_code)]
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x
            && x <= self.position.x + self.width
            && y >= self.position.y
            && y <= self.position.y + self.height
    }
    
    pub fn collides_with_circle(&self, pos: &Position, radius: f32) -> bool {
        let closest_x = pos.x.clamp(self.position.x, self.position.x + self.width);
        let closest_y = pos.y.clamp(self.position.y, self.position.y + self.height);
        
        let distance = ((pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2)).sqrt();
        distance < radius
    }
    
    pub fn draw(&self) {
        let color = if self.destructible {
            let health_ratio = self.health as f32 / self.max_health as f32;
            Color::new(0.6 * health_ratio, 0.3, 0.1, 1.0)
        } else {
            GRAY
        };
        
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            color,
        );
        
        // 绘制边框
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            2.0,
            DARKGRAY,
        );
    }
}