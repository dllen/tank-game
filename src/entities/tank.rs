use super::{Position, Velocity};
use macroquad::prelude::*;
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Tank {
    pub position: Position,
    pub velocity: Velocity,
    pub angle: f32,
    pub health: i32,
    pub max_health: i32,
    pub size: f32,
    pub speed: f32,
    pub base_speed: f32, // 原始速度，用于特效结束后恢复
    pub color: Color,
    pub base_color: Color, // 原始颜色
    pub shield: Option<Shield>,
    pub last_shot: f64,
    pub shot_cooldown: f64,
    pub base_shot_cooldown: f64, // 原始射击冷却
    pub is_player: bool,
    pub scatter_shot: bool,
    pub power_effects: Vec<PowerEffect>,
}

#[derive(Clone)]
pub struct Shield {
    #[allow(dead_code)]
    pub duration: f64,
    #[allow(dead_code)]
    pub start_time: f64,
}

#[derive(Clone)]
pub struct PowerEffect {
    pub effect_type: PowerEffectType,
    pub start_time: f64,
    pub duration: f64,
}

#[derive(Clone, PartialEq)]
pub enum PowerEffectType {
    SpeedBoost,   // 速度提升50%
    RapidFire,    // 射击速度提升50%
    DoubleDamage, // 双倍伤害
    Stealth,      // 隐身
    Invincible,   // 无敌
}

impl Tank {
    pub fn new_player(x: f32, y: f32) -> Self {
        let base_speed = 250.0;
        let base_color = BLUE;
        let base_shot_cooldown = 0.25;
        Self {
            position: Position::new(x, y),
            velocity: Velocity::new(0.0, 0.0),
            angle: 0.0,
            health: 200,
            max_health: 200,
            size: 20.0,
            speed: base_speed,
            base_speed,
            color: base_color,
            base_color,
            shield: None,
            last_shot: 0.0,
            shot_cooldown: base_shot_cooldown,
            base_shot_cooldown,
            is_player: true,
            scatter_shot: false,
            power_effects: Vec::new(),
        }
    }

    pub fn new_enemy(x: f32, y: f32) -> Self {
        let base_speed = 80.0;
        let base_color = RED;
        let base_shot_cooldown = 1.0;
        Self {
            position: Position::new(x, y),
            velocity: Velocity::new(0.0, 0.0),
            angle: 0.0,
            health: 50,
            max_health: 50,
            size: 18.0,
            speed: base_speed,
            base_speed,
            color: base_color,
            base_color,
            shield: None,
            last_shot: 0.0,
            shot_cooldown: base_shot_cooldown,
            base_shot_cooldown,
            is_player: false,
            scatter_shot: false,
            power_effects: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, dt: f32) {
        // 更新特效
        self.update_power_effects();

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

    // 检查是否会与障碍物碰撞的预测函数
    pub fn would_collide_with_obstacles(
        &self,
        new_x: f32,
        new_y: f32,
        obstacles: &[&crate::entities::Obstacle],
    ) -> bool {
        for obstacle in obstacles {
            // 计算障碍物的边界
            let obstacle_left = obstacle.position.x;
            let obstacle_right = obstacle.position.x + obstacle.width;
            let obstacle_top = obstacle.position.y;
            let obstacle_bottom = obstacle.position.y + obstacle.height;

            // 计算坦克的边界
            let tank_left = new_x - self.size;
            let tank_right = new_x + self.size;
            let tank_top = new_y - self.size;
            let tank_bottom = new_y + self.size;

            // 检查是否重叠
            if tank_right > obstacle_left
                && tank_left < obstacle_right
                && tank_bottom > obstacle_top
                && tank_top < obstacle_bottom
            {
                return true;
            }
        }
        false
    }

    // 安全移动函数，考虑障碍物碰撞和地形效果
    pub fn safe_move(&mut self, dt: f32, obstacles: &[crate::entities::Obstacle]) {
        let original_x = self.position.x;
        let original_y = self.position.y;

        // 计算地形修正的速度
        let terrain_modifier = self.get_terrain_modifier(obstacles);
        let modified_velocity_x = self.velocity.x * terrain_modifier;
        let modified_velocity_y = self.velocity.y * terrain_modifier;

        // 计算预期的新位置
        let target_x = self.position.x + modified_velocity_x * dt;
        let target_y = self.position.y + modified_velocity_y * dt;

        // 边界检查
        let screen_width = screen_width();
        let screen_height = screen_height();

        let mut new_x = target_x;
        let mut new_y = target_y;

        // 边界限制和反弹
        if new_x - self.size < 0.0 {
            new_x = self.size;
            self.velocity.x = 0.0; // 停止向边界移动
        } else if new_x + self.size > screen_width {
            new_x = screen_width - self.size;
            self.velocity.x = 0.0; // 停止向边界移动
        }

        if new_y - self.size < 0.0 {
            new_y = self.size;
            self.velocity.y = 0.0; // 停止向边界移动
        } else if new_y + self.size > screen_height {
            new_y = screen_height - self.size;
            self.velocity.y = 0.0; // 停止向边界移动
        }

        // 只检查阻挡移动的障碍物（墙体、钢墙）
        let blocking_obstacles: Vec<_> = obstacles
            .iter()
            .filter(|obs| obs.blocks_movement())
            .collect();

        // 创建实际的障碍物切片
        let blocking_refs: Vec<&crate::entities::Obstacle> =
            blocking_obstacles.iter().map(|&obs| obs).collect();

        // 尝试X轴移动
        let test_x_pos = new_x;
        if !self.would_collide_with_obstacles(test_x_pos, original_y, &blocking_refs) {
            self.position.x = test_x_pos;
        } else {
            self.velocity.x = 0.0; // 停止X轴移动
                                   // 添加小幅度的反向推力防止抖动
            if target_x > original_x {
                self.velocity.x = -10.0; // 向左推
            } else if target_x < original_x {
                self.velocity.x = 10.0; // 向右推
            }
        }

        // 尝试Y轴移动
        let test_y_pos = new_y;
        if !self.would_collide_with_obstacles(self.position.x, test_y_pos, &blocking_refs) {
            self.position.y = test_y_pos;
        } else {
            self.velocity.y = 0.0; // 停止Y轴移动
                                   // 添加小幅度的反向推力防止抖动
            if target_y > original_y {
                self.velocity.y = -10.0; // 向上推
            } else if target_y < original_y {
                self.velocity.y = 10.0; // 向下推
            }
        }

        // 如果两个轴都不能移动，尝试对角线移动
        if self.position.x == original_x && self.position.y == original_y {
            if !self.would_collide_with_obstacles(new_x, new_y, &blocking_refs) {
                self.position.x = new_x;
                self.position.y = new_y;
            } else {
                // 如果完全卡住，尝试小幅度随机移动来脱困
                let mut rng = thread_rng();

                for _ in 0..8 {
                    // 尝试8个方向
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let escape_distance = self.size * 0.5;
                    let escape_x = original_x + angle.cos() * escape_distance;
                    let escape_y = original_y + angle.sin() * escape_distance;

                    // 检查边界
                    if escape_x - self.size >= 0.0
                        && escape_x + self.size <= screen_width
                        && escape_y - self.size >= 0.0
                        && escape_y + self.size <= screen_height
                    {
                        if !self.would_collide_with_obstacles(escape_x, escape_y, &blocking_refs) {
                            self.position.x = escape_x;
                            self.position.y = escape_y;
                            self.velocity.x *= 0.5; // 减少速度避免再次卡住
                            self.velocity.y *= 0.5;
                            break;
                        }
                    }
                }
            }
        }

        // 尝试Y轴移动
        let test_y_pos = new_y;
        if !self.would_collide_with_obstacles(self.position.x, test_y_pos, &blocking_obstacles) {
            self.position.y = test_y_pos;
        } else {
            self.velocity.y = 0.0; // 停止Y轴移动
                                   // 添加小幅度的反向推力防止抖动
            if target_y > original_y {
                self.velocity.y = -10.0; // 向上推
            } else if target_y < original_y {
                self.velocity.y = 10.0; // 向下推
            }
        }

        // 如果两个轴都不能移动，尝试对角线移动
        if self.position.x == original_x && self.position.y == original_y {
            if !self.would_collide_with_obstacles(new_x, new_y, &blocking_obstacles) {
                self.position.x = new_x;
                self.position.y = new_y;
            } else {
                // 如果完全卡住，尝试小幅度随机移动来脱困
                let mut rng = ::rand::thread_rng();
                use rand::Rng;

                for _ in 0..8 {
                    // 尝试8个方向
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let escape_distance = self.size * 0.5;
                    let escape_x = original_x + angle.cos() * escape_distance;
                    let escape_y = original_y + angle.sin() * escape_distance;

                    // 检查边界
                    if escape_x - self.size >= 0.0
                        && escape_x + self.size <= screen_width
                        && escape_y - self.size >= 0.0
                        && escape_y + self.size <= screen_height
                    {
                        if !self.would_collide_with_obstacles(
                            escape_x,
                            escape_y,
                            &blocking_obstacles,
                        ) {
                            self.position.x = escape_x;
                            self.position.y = escape_y;
                            self.velocity.x *= 0.5; // 减少速度避免再次卡住
                            self.velocity.y *= 0.5;
                            break;
                        }
                    }
                }
            }
        }
    }

    // 获取地形修正系数
    fn get_terrain_modifier(&self, obstacles: &[crate::entities::Obstacle]) -> f32 {
        let mut modifier = 1.0;

        for obstacle in obstacles {
            if obstacle.contains_point(self.position.x, self.position.y) {
                modifier *= obstacle.get_speed_modifier();
            }
        }

        modifier
    }

    // 检查是否在隐蔽地形中
    pub fn is_in_cover(&self, obstacles: &[crate::entities::Obstacle]) -> bool {
        obstacles
            .iter()
            .any(|obs| obs.contains_point(self.position.x, self.position.y) && obs.provides_cover())
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

        if self.is_invincible() {
            return false; // 无敌状态
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

    pub fn add_power_effect(&mut self, effect_type: PowerEffectType) {
        self.power_effects.push(PowerEffect {
            effect_type,
            start_time: get_time(),
            duration: 10.0, // 10秒时效
        });
        self.apply_power_effects();
    }

    fn update_power_effects(&mut self) {
        let current_time = get_time();

        // 移除过期的特效
        self.power_effects
            .retain(|effect| current_time - effect.start_time < effect.duration);

        // 重新应用剩余特效
        self.apply_power_effects();
    }

    fn apply_power_effects(&mut self) {
        // 重置为基础值
        self.speed = self.base_speed;
        self.shot_cooldown = self.base_shot_cooldown;
        self.color = self.base_color;

        let current_time = get_time();

        for effect in &self.power_effects {
            let remaining_time = effect.duration - (current_time - effect.start_time);

            match effect.effect_type {
                PowerEffectType::SpeedBoost => {
                    self.speed = self.base_speed * 1.5;
                    // 速度提升时显示青色
                    self.color = Color::new(0.0, 1.0, 1.0, 1.0);
                }
                PowerEffectType::RapidFire => {
                    self.shot_cooldown = self.base_shot_cooldown * 0.5;
                    // 快速射击时显示橙色
                    self.color = Color::new(1.0, 0.5, 0.0, 1.0);
                }
                PowerEffectType::DoubleDamage => {
                    // 双倍伤害时显示紫色
                    self.color = Color::new(0.8, 0.0, 1.0, 1.0);
                }
                PowerEffectType::Stealth => {
                    // 隐身时显示半透明
                    self.color.a = 0.3;
                }
                PowerEffectType::Invincible => {
                    // 无敌时显示金色
                    self.color = Color::new(1.0, 0.8, 0.0, 1.0);
                }
            }

            // 时效快结束时的闪烁效果
            if remaining_time < 2.0 {
                let flash = (remaining_time * 10.0).sin() > 0.0;
                if !flash {
                    self.color.a *= 0.5;
                }
            }
        }
    }

    pub fn has_effect(&self, effect_type: PowerEffectType) -> bool {
        self.power_effects
            .iter()
            .any(|e| e.effect_type == effect_type)
    }

    pub fn get_damage_multiplier(&self) -> f32 {
        if self.has_effect(PowerEffectType::DoubleDamage) {
            2.0
        } else {
            1.0
        }
    }

    pub fn is_invincible(&self) -> bool {
        self.has_effect(PowerEffectType::Invincible)
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
            10.0,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );

        // 炮管主体
        draw_line(
            barrel_start_x,
            barrel_start_y,
            barrel_end_x,
            barrel_end_y,
            10.0,
            BLACK,
        );

        // 炮管高光
        draw_line(
            barrel_start_x,
            barrel_start_y,
            barrel_end_x,
            barrel_end_y,
            4.0,
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
        let turret_color = Color::new(color.r * 0.9, color.g * 0.9, color.b * 0.9, color.a);
        draw_circle(
            self.position.x,
            self.position.y,
            turret_radius,
            turret_color,
        );

        // 炮塔边框
        draw_circle_lines(
            self.position.x,
            self.position.y,
            turret_radius,
            2.0,
            DARKGRAY,
        );

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
            draw_circle_lines(self.position.x, self.position.y, shield_radius, 3.0, YELLOW);

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
    fn draw_rotated_rectangle(
        &self,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
        angle: f32,
        color: Color,
    ) {
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
            shadow_corners.push(Vec2::new(
                corner.x + shadow_offset,
                corner.y + shadow_offset,
            ));
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
