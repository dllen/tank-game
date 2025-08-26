use super::Position;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub enum PowerUpType {
    Health,
    Shield,
    ScatterShot,
    SpeedBoost,
    Damage,
}

#[derive(Clone)]
pub struct PowerUp {
    pub position: Position,
    pub power_type: PowerUpType,
    pub size: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub collected: bool,
}

impl PowerUp {
    pub fn new_random(x: f32, y: f32) -> Self {
        let mut rng = thread_rng();
        let power_type = match rng.gen_range(0..5) {
            0 => PowerUpType::Health,
            1 => PowerUpType::Shield,
            2 => PowerUpType::ScatterShot,
            3 => PowerUpType::SpeedBoost,
            _ => PowerUpType::Damage,
        };
        
        Self {
            position: Position::new(x, y),
            power_type,
            size: 12.0,
            lifetime: 0.0,
            max_lifetime: 15.0,
            collected: false,
        }
    }
    
    pub fn update(&mut self, dt: f32) -> bool {
        self.lifetime += dt;
        self.lifetime < self.max_lifetime && !self.collected
    }
    
    pub fn collides_with_circle(&self, pos: &Position, radius: f32) -> bool {
        if self.collected {
            return false;
        }
        self.position.distance_to(pos) < self.size + radius
    }
    
    pub fn collect(&mut self) {
        self.collected = true;
    }
    
    pub fn get_color(&self) -> Color {
        match self.power_type {
            PowerUpType::Health => GREEN,
            PowerUpType::Shield => YELLOW,
            PowerUpType::ScatterShot => PURPLE,
            PowerUpType::SpeedBoost => SKYBLUE,
            PowerUpType::Damage => RED,
        }
    }
    
    pub fn get_symbol(&self) -> &str {
        match self.power_type {
            PowerUpType::Health => "+",
            PowerUpType::Shield => "S",
            PowerUpType::ScatterShot => "*",
            PowerUpType::SpeedBoost => ">",
            PowerUpType::Damage => "!",
        }
    }
    
    pub fn draw(&self) {
        if self.collected {
            return;
        }
        
        // 闪烁效果
        let alpha = if (self.lifetime * 4.0).sin() > 0.0 { 1.0 } else { 0.7 };
        let mut color = self.get_color();
        color.a = alpha;
        
        // 绘制道具
        draw_circle(self.position.x, self.position.y, self.size, color);
        draw_circle_lines(self.position.x, self.position.y, self.size, 2.0, WHITE);
        
        // 绘制符号
        let text = self.get_symbol();
        let text_size = 20.0;
        let text_dims = measure_text(text, None, text_size as u16, 1.0);
        draw_text(
            text,
            self.position.x - text_dims.width / 2.0,
            self.position.y + text_dims.height / 2.0,
            text_size,
            BLACK,
        );
    }
}