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
            health: 200,
            max_health: 200,
            size: 20.0,
            speed: 250.0, // 提高基础速度
            color: BLUE,
            shield: None,
            last_shot: 0.0,
            shot_cooldown: 0.25, // 稍微减少射击冷却时间
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
        // 预测新位置
        let new_x = self.position.x + self.velocity.x * dt;
        let new_y = self.position.y + self.velocity.y * dt;
        
        // 边界检查 - 预防性碰撞检测
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        let mut final_x = new_x;
        let mut final_y = new_y;
        
        // 检查X轴边界
        if new_x - self.size < 0.0 {
            final_x = self.size;
            self.velocity.x = 0.0; // 停止X轴移动
        } else if new_x + self.size > screen_width {
            final_x = screen_width - self.size;
            self.velocity.x = 0.0; // 停止X轴移动
        }
        
        // 检查Y轴边界
        if new_y - self.size < 0.0 {
            final_y = self.size;
            self.velocity.y = 0.0; // 停止Y轴移动
        } else if new_y + self.size > screen_height {
            final_y = screen_height - self.size;
            self.velocity.y = 0.0; // 停止Y轴移动
        }
        
        // 应用最终位置
        self.position.x = final_x;
        self.position.y = final_y;
        
        // 更新护盾
        if let Some(shield) = &self.shield {
            if get_time() - shield.start_time > shield.duration {
                self.shield = None;
            }
        }
    }
    
    // 新增：检查是否会与障碍物碰撞的预测函数
    pub fn would_collide_with_obstacles(&self, new_x: f32, new_y: f32, obstacles: &[crate::entities::Obstacle]) -> bool {
        for obstacle in obstacles {
            let distance_x = (new_x - (obstacle.position.x + obstacle.width / 2.0)).abs();
            let distance_y = (new_y - (obstacle.position.y + obstacle.height / 2.0)).abs();
            
            if distance_x < (self.size + obstacle.width / 2.0) && distance_y < (self.size + obstacle.height / 2.0) {
                return true;
            }
        }
        false
    }
    
    // 新增：安全移动函数，考虑障碍物碰撞
    pub fn safe_move(&mut self, dt: f32, obstacles: &[crate::entities::Obstacle]) {
        let original_x = self.position.x;
        let original_y = self.position.y;
        
        // 计算预期的新位置
        let target_x = self.position.x + self.velocity.x * dt;
        let target_y = self.position.y + self.velocity.y * dt;
        
        // 边界检查
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        let mut new_x = target_x;
        let mut new_y = target_y;
        
        // 边界限制
        if new_x - self.size < 0.0 {
            new_x = self.size;
        } else if new_x + self.size > screen_width {
            new_x = screen_width - self.size;
        }
        
        if new_y - self.size < 0.0 {
            new_y = self.size;
        } else if new_y + self.size > screen_height {
            new_y = screen_height - self.size;
        }
        
        // 尝试X轴移动
        let test_x_pos = new_x;
        if !self.would_collide_with_obstacles(test_x_pos, original_y, obstacles) {
            self.position.x = test_x_pos;
        } else {
            self.velocity.x = 0.0; // 停止X轴移动
        }
        
        // 尝试Y轴移动
        let test_y_pos = new_y;
        if !self.would_collide_with_obstacles(self.position.x, test_y_pos, obstacles) {
            self.position.y = test_y_pos;
        } else {
            self.velocity.y = 0.0; // 停止Y轴移动
        }
        
        // 如果两个轴都不能移动，尝试对角线移动
        if self.position.x == original_x && self.position.y == original_y {
            if !self.would_collide_with_obstacles(new_x, new_y, obstacles) {
                self.position.x = new_x;
                self.position.y = new_y;
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
        
        // 绘制旋转的坦克主体
        self.draw_rotated_rectangle(
            self.position.x,
            self.position.y,
            body_width,
            body_height,
            self.angle,
            color,
        );
        
        // 履带已移除，保持简洁的坦克外观
        
        // 绘制炮管（与坦克主体同方向）
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
        
        // 绘制炮塔（圆形，与主体同方向）
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
        
        // 炮塔方向指示器（小矩形）
        let indicator_length = turret_radius * 0.6;
        let indicator_x = self.position.x + self.angle.cos() * indicator_length;
        let indicator_y = self.position.y + self.angle.sin() * indicator_length;
        draw_circle(indicator_x, indicator_y, 2.0, DARKGRAY);
        
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
    
    // 辅助函数：绘制旋转的矩形
    fn draw_rotated_rectangle(&self, center_x: f32, center_y: f32, width: f32, height: f32, angle: f32, color: Color) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        // 计算矩形的四个角点（相对于中心）
        let half_w = width / 2.0;
        let half_h = height / 2.0;
        
        let corners = [
            (-half_w, -half_h),
            (half_w, -half_h),
            (half_w, half_h),
            (-half_w, half_h),
        ];
        
        // 旋转并转换到世界坐标
        let mut rotated_corners = Vec::new();
        for (x, y) in corners.iter() {
            let rotated_x = center_x + x * cos_a - y * sin_a;
            let rotated_y = center_y + x * sin_a + y * cos_a;
            rotated_corners.push(Vec2::new(rotated_x, rotated_y));
        }
        
        // 绘制阴影
        let shadow_offset = 2.0;
        let mut shadow_corners = Vec::new();
        for corner in &rotated_corners {
            shadow_corners.push(Vec2::new(corner.x + shadow_offset, corner.y + shadow_offset));
        }
        
        // 使用三角形绘制矩形阴影
        draw_triangle(
            shadow_corners[0],
            shadow_corners[1],
            shadow_corners[2],
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
        draw_triangle(
            shadow_corners[0],
            shadow_corners[2],
            shadow_corners[3],
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
        
        // 绘制主体
        draw_triangle(
            rotated_corners[0],
            rotated_corners[1],
            rotated_corners[2],
            color,
        );
        draw_triangle(
            rotated_corners[0],
            rotated_corners[2],
            rotated_corners[3],
            color,
        );
        
        // 绘制边框
        for i in 0..4 {
            let next_i = (i + 1) % 4;
            draw_line(
                rotated_corners[i].x,
                rotated_corners[i].y,
                rotated_corners[next_i].x,
                rotated_corners[next_i].y,
                2.0,
                DARKGRAY,
            );
        }
    }
}