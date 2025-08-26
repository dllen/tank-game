use super::{Position, Velocity};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Tank {
    pub position: Position,
    pub velocity: Velocity,
    pub angle: f32,
    pub health: i32,
    pub max_health: i32,
    pub size: f32,
    pub speed: f32,
    pub color: Color,
    pub shield: Option<Shield>,
    pub last_shot: f64,
    pub shot_cooldown: f64,
    pub is_player: bool,
    pub scatter_shot: bool,
}

#[derive(Clone)]
pub struct Shield {
    pub duration: f64,
    pub start_time: f64,
}

impl Tank {
    pub fn new_player(x: f32, y: f32) -> Self {
        Self {
            position: Position::new(x, y),
            velocity: Velocity::new(0.0, 0.0),
            angle: 0.0,
            health: 100,
            max_health: 100,
            size: 20.0,
            speed: 150.0,
            color: BLUE,
            shield: None,
            last_shot: 0.0,
            shot_cooldown: 0.3,
            is_player: true,
            scatter_shot: false,
        }
    }
    
    pub fn new_enemy(x: f32, y: f32) -> Self {
        Self {
            position: Position::new(x, y),
            velocity: Velocity::new(0.0, 0.0),
            angle: 0.0,
            health: 50,
            max_health: 50,
            size: 18.0,
            speed: 80.0,
            color: RED,
            shield: None,
            last_shot: 0.0,
            shot_cooldown: 1.0,
            is_player: false,
            scatter_shot: false,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        // 更新位置
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        
        // 边界检查
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        if self.position.x < self.size {
            self.position.x = self.size;
        } else if self.position.x > screen_width - self.size {
            self.position.x = screen_width - self.size;
        }
        
        if self.position.y < self.size {
            self.position.y = self.size;
        } else if self.position.y > screen_height - self.size {
            self.position.y = screen_height - self.size;
        }
        
        // 更新护盾
        if let Some(shield) = &self.shield {
            if get_time() - shield.start_time > shield.duration {
                self.shield = None;
            }
        }
    }
    
    pub fn can_shoot(&self) -> bool {
        get_time() - self.last_shot > self.shot_cooldown
    }
    
    pub fn shoot(&mut self) {
        self.last_shot = get_time();
    }
    
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if self.shield.is_some() {
            return false; // 护盾保护
        }
        
        self.health -= damage;
        self.health <= 0
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }
    
    pub fn add_shield(&mut self, duration: f64) {
        self.shield = Some(Shield {
            duration,
            start_time: get_time(),
        });
    }
    
    pub fn draw(&self) {
        let color = if self.shield.is_some() {
            Color::new(self.color.r, self.color.g, self.color.b, 0.7)
        } else {
            self.color
        };
        
        // 计算坦克主体的尺寸
        let body_width = self.size * 2.0;
        let body_height = self.size * 1.4;
        let turret_radius = self.size * 0.5;
        
        // 绘制坦克主体（矩形）
        let body_x = self.position.x - body_width / 2.0;
        let body_y = self.position.y - body_height / 2.0;
        
        // 主体阴影效果
        draw_rectangle(
            body_x + 2.0,
            body_y + 2.0,
            body_width,
            body_height,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
        
        // 主体
        draw_rectangle(body_x, body_y, body_width, body_height, color);
        
        // 主体边框
        draw_rectangle_lines(body_x, body_y, body_width, body_height, 2.0, DARKGRAY);
        
        // 绘制履带细节
        let track_color = Color::new(0.2, 0.2, 0.2, 1.0);
        // 左履带
        draw_rectangle(
            body_x - 3.0,
            body_y,
            4.0,
            body_height,
            track_color,
        );
        // 右履带
        draw_rectangle(
            body_x + body_width - 1.0,
            body_y,
            4.0,
            body_height,
            track_color,
        );
        
        // 绘制炮管（黑色宽线条）
        let barrel_length = self.size * 2.2;
        let barrel_start_x = self.position.x + self.angle.cos() * turret_radius * 0.8;
        let barrel_start_y = self.position.y + self.angle.sin() * turret_radius * 0.8;
        let barrel_end_x = self.position.x + self.angle.cos() * barrel_length;
        let barrel_end_y = self.position.y + self.angle.sin() * barrel_length;
        
        // 炮管阴影
        draw_line(
            barrel_start_x + 1.0,
            barrel_start_y + 1.0,
            barrel_end_x + 1.0,
            barrel_end_y + 1.0,
            6.0,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
        
        // 炮管主体
        draw_line(
            barrel_start_x,
            barrel_start_y,
            barrel_end_x,
            barrel_end_y,
            6.0,
            BLACK,
        );
        
        // 炮管高光
        draw_line(
            barrel_start_x,
            barrel_start_y,
            barrel_end_x,
            barrel_end_y,
            2.0,
            DARKGRAY,
        );
        
        // 绘制炮塔（圆形）
        // 炮塔阴影
        draw_circle(
            self.position.x + 1.0,
            self.position.y + 1.0,
            turret_radius,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
        
        // 炮塔主体
        let turret_color = Color::new(
            color.r * 0.9,
            color.g * 0.9,
            color.b * 0.9,
            color.a,
        );
        draw_circle(self.position.x, self.position.y, turret_radius, turret_color);
        
        // 炮塔边框
        draw_circle_lines(self.position.x, self.position.y, turret_radius, 2.0, DARKGRAY);
        
        // 炮塔中心点
        draw_circle(self.position.x, self.position.y, 3.0, DARKGRAY);
        
        // 绘制护盾效果
        if self.shield.is_some() {
            let shield_radius = (body_width.max(body_height) / 2.0) + 8.0;
            draw_circle_lines(
                self.position.x,
                self.position.y,
                shield_radius,
                3.0,
                YELLOW,
            );
            
            // 护盾闪烁效果
            let time = macroquad::prelude::get_time();
            if (time * 8.0).sin() > 0.0 {
                draw_circle_lines(
                    self.position.x,
                    self.position.y,
                    shield_radius - 3.0,
                    2.0,
                    Color::new(1.0, 1.0, 0.0, 0.5),
                );
            }
        }
        
        // 绘制血条（不旋转）
        if !self.is_player {
            let bar_width = body_width;
            let bar_height = 4.0;
            let bar_x = self.position.x - bar_width / 2.0;
            let bar_y = self.position.y - body_height / 2.0 - 12.0;
            
            // 血条背景
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
            
            // 血量
            let health_ratio = self.health as f32 / self.max_health as f32;
            let health_color = if health_ratio > 0.6 {
                GREEN
            } else if health_ratio > 0.3 {
                YELLOW
            } else {
                RED
            };
            
            draw_rectangle(
                bar_x,
                bar_y,
                bar_width * health_ratio,
                bar_height,
                health_color,
            );
            
            // 血条边框
            draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 1.0, WHITE);
        }
    }
}