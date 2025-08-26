use super::{Position, Tank};
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

pub struct EnemyAI {
    #[allow(dead_code)]
    pub target_position: Position,
    pub last_direction_change: f64,
    pub direction_change_interval: f64,
    pub aggression_level: f32,
    pub difficulty: f32,
    pub last_player_position: Position,
    pub player_velocity_estimate: (f32, f32),
}

impl EnemyAI {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::new_with_difficulty(1.0)
    }
    
    pub fn new_with_difficulty(difficulty: f32) -> Self {
        let mut rng = thread_rng();
        // 根据难度调整攻击性
        let base_aggression = if difficulty <= 1.0 {
            // 容易模式：低攻击性
            rng.gen_range(0.1..0.3)
        } else if difficulty <= 2.0 {
            // 普通模式：中等攻击性
            rng.gen_range(0.3..0.6)
        } else {
            // 困难模式：高攻击性
            rng.gen_range(0.6..0.9)
        };
        
        Self {
            target_position: Position::new(0.0, 0.0),
            last_direction_change: get_time(),
            direction_change_interval: rng.gen_range(2.0..5.0), // 增加方向改变间隔
            aggression_level: base_aggression,
            difficulty,
            last_player_position: Position::new(0.0, 0.0),
            player_velocity_estimate: (0.0, 0.0),
        }
    }
    
    pub fn update(&mut self, enemy_tank: &mut Tank, player_tank: &Tank, obstacles: &[super::Obstacle]) {
        let current_time = get_time();
        
        // 更新玩家速度估计（用于预测瞄准）
        self.update_player_velocity_estimate(player_tank);
        
        // 计算到玩家的距离
        let distance_to_player = enemy_tank.position.distance_to(&player_tank.position);
        
        // 根据难度调整行为距离
        let (retreat_distance, approach_distance) = if self.difficulty <= 1.0 {
            // 容易模式：保持更远距离，不那么激进
            (150.0, 300.0)
        } else if self.difficulty <= 2.0 {
            // 普通模式：中等距离
            (120.0, 250.0)
        } else {
            // 困难模式：更近距离，更激进
            (100.0, 200.0)
        };
        
        // 容易模式下，提高追击概率，让AI更智能
        let mut rng = thread_rng();
        let should_pursue = if self.difficulty <= 1.0 {
            rng.gen::<f32>() < 0.8 // 容易模式提高到80%概率追击
        } else if self.difficulty <= 2.0 {
            rng.gen::<f32>() < 0.7 // 普通模式70%概率追击
        } else {
            rng.gen::<f32>() < 0.9 // 困难模式90%概率追击
        };
        
        // 首先检查边界避让
        if self.check_and_avoid_boundaries(enemy_tank) {
            // 如果正在避开边界，不执行其他移动逻辑
        } else if self.check_and_avoid_obstacles(enemy_tank, obstacles) {
            // 如果正在避开障碍物，不执行其他移动逻辑
        } else if should_pursue {
            // 如果太近，后退
            if distance_to_player < retreat_distance {
                self.retreat_from_player(enemy_tank, player_tank);
            } else if distance_to_player > approach_distance {
                // 如果太远，接近玩家
                self.approach_player(enemy_tank, player_tank);
            } else {
                // 在合适距离内，使用智能战术移动
                if current_time - self.last_direction_change > self.direction_change_interval {
                    if self.difficulty <= 1.0 {
                        // 容易模式：使用侧向移动战术
                        self.tactical_movement(enemy_tank, player_tank);
                    } else {
                        // 其他模式：随机移动
                        self.random_movement(enemy_tank);
                    }
                    self.last_direction_change = current_time;
                    let interval_range = if self.difficulty <= 1.0 {
                        2.0..4.0 // 容易模式：更频繁的战术调整
                    } else {
                        1.0..3.0 // 其他模式：较短间隔
                    };
                    self.direction_change_interval = rng.gen_range(interval_range);
                }
            }
        } else {
            // 不追击时进行随机移动
            if current_time - self.last_direction_change > self.direction_change_interval {
                self.random_movement(enemy_tank);
                self.last_direction_change = current_time;
                self.direction_change_interval = rng.gen_range(2.0..5.0);
            }
        }
        
        // 根据难度调整瞄准行为
        if self.difficulty <= 1.0 {
            // 容易模式：使用预测瞄准，让AI更智能
            if rng.gen::<f32>() < 0.8 {
                self.aim_at_player_predictive(enemy_tank, player_tank);
            }
        } else {
            // 其他模式：总是瞄准玩家当前位置
            self.aim_at_player(enemy_tank, player_tank);
        }
        
        // 决定是否射击
        if self.should_shoot(enemy_tank, player_tank, distance_to_player) {
            if enemy_tank.can_shoot() {
                enemy_tank.shoot();
            }
        }
    }
    
    fn retreat_from_player(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        let dx = enemy_tank.position.x - player_tank.position.x;
        let dy = enemy_tank.position.y - player_tank.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            enemy_tank.velocity.x = (dx / distance) * enemy_tank.speed;
            enemy_tank.velocity.y = (dy / distance) * enemy_tank.speed;
        }
    }
    
    fn approach_player(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        let dx = player_tank.position.x - enemy_tank.position.x;
        let dy = player_tank.position.y - enemy_tank.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            enemy_tank.velocity.x = (dx / distance) * enemy_tank.speed * 0.7;
            enemy_tank.velocity.y = (dy / distance) * enemy_tank.speed * 0.7;
        }
    }
    
    fn random_movement(&mut self, enemy_tank: &mut Tank) {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed_factor = rng.gen_range(0.3..1.0);
        
        enemy_tank.velocity.x = angle.cos() * enemy_tank.speed * speed_factor;
        enemy_tank.velocity.y = angle.sin() * enemy_tank.speed * speed_factor;
    }
    
    fn tactical_movement(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        let mut rng = thread_rng();
        
        // 计算到玩家的向量
        let dx = player_tank.position.x - enemy_tank.position.x;
        let dy = player_tank.position.y - enemy_tank.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            // 计算垂直于玩家方向的向量（侧向移动）
            let perpendicular_x = -dy / distance;
            let perpendicular_y = dx / distance;
            
            // 随机选择左侧或右侧移动
            let side_factor = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            
            // 混合侧向移动和轻微的接近/远离
            let approach_factor = rng.gen_range(-0.3..0.3);
            let approach_x = dx / distance * approach_factor;
            let approach_y = dy / distance * approach_factor;
            
            // 最终移动向量
            let final_x = perpendicular_x * side_factor * 0.8 + approach_x;
            let final_y = perpendicular_y * side_factor * 0.8 + approach_y;
            
            enemy_tank.velocity.x = final_x * enemy_tank.speed;
            enemy_tank.velocity.y = final_y * enemy_tank.speed;
        }
    }
    
    fn aim_at_player(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        let dx = player_tank.position.x - enemy_tank.position.x;
        let dy = player_tank.position.y - enemy_tank.position.y;
        enemy_tank.angle = dy.atan2(dx);
    }
    
    fn update_player_velocity_estimate(&mut self, player_tank: &Tank) {
        // 计算玩家速度
        let dx = player_tank.position.x - self.last_player_position.x;
        let dy = player_tank.position.y - self.last_player_position.y;
        
        // 平滑速度估计
        self.player_velocity_estimate.0 = self.player_velocity_estimate.0 * 0.7 + dx * 0.3;
        self.player_velocity_estimate.1 = self.player_velocity_estimate.1 * 0.7 + dy * 0.3;
        
        // 更新上一帧玩家位置
        self.last_player_position = player_tank.position.clone();
    }
    
    fn aim_at_player_predictive(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        // 计算子弹飞行时间
        let distance = enemy_tank.position.distance_to(&player_tank.position);
        let bullet_speed = 300.0; // 假设子弹速度
        let flight_time = distance / bullet_speed;
        
        // 预测玩家位置
        let predicted_x = player_tank.position.x + self.player_velocity_estimate.0 * flight_time * 60.0; // 60fps
        let predicted_y = player_tank.position.y + self.player_velocity_estimate.1 * flight_time * 60.0;
        
        // 瞄准预测位置
        let dx = predicted_x - enemy_tank.position.x;
        let dy = predicted_y - enemy_tank.position.y;
        enemy_tank.angle = dy.atan2(dx);
    }
    
    fn check_and_avoid_boundaries(&mut self, enemy_tank: &mut Tank) -> bool {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let boundary_margin = enemy_tank.size + 50.0; // 增加边界检测距离
        
        let mut avoid_x: f32 = 0.0;
        let mut avoid_y: f32 = 0.0;
        let mut needs_avoidance = false;
        
        // 检查左边界 - 使用渐进式避让
        if enemy_tank.position.x < boundary_margin {
            let distance_factor = (boundary_margin - enemy_tank.position.x) / boundary_margin;
            avoid_x += distance_factor;
            needs_avoidance = true;
        }
        // 检查右边界
        if enemy_tank.position.x > screen_width - boundary_margin {
            let distance_factor = (enemy_tank.position.x - (screen_width - boundary_margin)) / boundary_margin;
            avoid_x -= distance_factor;
            needs_avoidance = true;
        }
        // 检查上边界
        if enemy_tank.position.y < boundary_margin {
            let distance_factor = (boundary_margin - enemy_tank.position.y) / boundary_margin;
            avoid_y += distance_factor;
            needs_avoidance = true;
        }
        // 检查下边界
        if enemy_tank.position.y > screen_height - boundary_margin {
            let distance_factor = (enemy_tank.position.y - (screen_height - boundary_margin)) / boundary_margin;
            avoid_y -= distance_factor;
            needs_avoidance = true;
        }
        
        if needs_avoidance {
            // 标准化避让向量
            let length = (avoid_x * avoid_x + avoid_y * avoid_y).sqrt();
            if length > 0.0 {
                avoid_x /= length;
                avoid_y /= length;
            }
            
            // 平滑的避让移动，不要太激进
            enemy_tank.velocity.x = avoid_x * enemy_tank.speed * 0.8;
            enemy_tank.velocity.y = avoid_y * enemy_tank.speed * 0.8;
            return true;
        }
        
        false
    }
    
    fn check_and_avoid_obstacles(&mut self, enemy_tank: &mut Tank, obstacles: &[super::Obstacle]) -> bool {
        let detection_distance = enemy_tank.size + 50.0; // 检测距离
        let mut closest_obstacle: Option<&super::Obstacle> = None;
        let mut closest_distance = f32::MAX;
        
        // 找到最近的障碍物
        for obstacle in obstacles {
            let obstacle_center_x = obstacle.position.x + obstacle.width / 2.0;
            let obstacle_center_y = obstacle.position.y + obstacle.height / 2.0;
            
            let dx = obstacle_center_x - enemy_tank.position.x;
            let dy = obstacle_center_y - enemy_tank.position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // 检查是否在检测范围内
            let obstacle_radius = (obstacle.width + obstacle.height) / 4.0;
            if distance - obstacle_radius < detection_distance && distance < closest_distance {
                closest_distance = distance;
                closest_obstacle = Some(obstacle);
            }
        }
        
        if let Some(obstacle) = closest_obstacle {
            // 计算避让方向
            let obstacle_center_x = obstacle.position.x + obstacle.width / 2.0;
            let obstacle_center_y = obstacle.position.y + obstacle.height / 2.0;
            
            let dx = enemy_tank.position.x - obstacle_center_x;
            let dy = enemy_tank.position.y - obstacle_center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 0.0 {
                // 计算距离因子，越近避让力度越大
                let obstacle_radius = (obstacle.width + obstacle.height) / 4.0;
                let distance_factor = 1.0 - ((distance - obstacle_radius) / detection_distance).max(0.0);
                
                // 计算避让向量（远离障碍物）
                let avoid_x = dx / distance;
                let avoid_y = dy / distance;
                
                // 添加切线方向的移动来绕过障碍物
                let tangent_x = -dy / distance;
                let tangent_y = dx / distance;
                
                // 混合避让和切线移动
                let final_x = avoid_x * 0.7 + tangent_x * 0.3;
                let final_y = avoid_y * 0.7 + tangent_y * 0.3;
                
                // 应用距离因子和平滑移动
                enemy_tank.velocity.x = final_x * enemy_tank.speed * distance_factor * 0.6;
                enemy_tank.velocity.y = final_y * enemy_tank.speed * distance_factor * 0.6;
                
                return true; // 正在避让障碍物
            }
        }
        
        false // 没有需要避让的障碍物
    }
    
    pub fn should_shoot(&self, _enemy_tank: &Tank, _player_tank: &Tank, distance: f32) -> bool {
        // 根据难度调整射击频率
        let base_shoot_chance = if self.difficulty <= 1.0 {
            0.02 // 容易模式：提高射击频率，让AI更智能
        } else if self.difficulty <= 2.0 {
            0.015 // 普通模式：中等射击频率
        } else {
            0.025 // 困难模式：高射击频率
        };
        
        // 基于距离调整射击概率
        let distance_factor = if distance < 150.0 { 
            1.0 
        } else if distance < 250.0 { 
            0.7 
        } else { 
            0.3 
        };
        
        let mut rng = thread_rng();
        rng.gen::<f32>() < self.aggression_level * distance_factor * base_shoot_chance
    }
}