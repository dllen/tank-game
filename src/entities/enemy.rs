use super::{Position, Tank};
use macroquad::prelude::*;
extern crate rand;
use rand::{thread_rng, Rng};

pub struct EnemyAI {
    #[allow(dead_code)]
    pub target_position: Position,
    pub last_direction_change: f64,
    pub direction_change_interval: f64,
    pub aggression_level: f32,
    pub difficulty: f32,
    pub last_player_position: Position,
    pub player_velocity_estimate: (f32, f32),
    pub state: EnemyState,
    pub state_change_time: f64,
    pub last_shot_time: f64,
    pub suppressed_until: f64, // 被火力压制时间
    pub patrol_target: Position,
    pub last_seen_player_time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Patrol,
    Chase,
    Attack,
    Evade,
    Suppress,
    Flank,
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

        let current_time = get_time();
        Self {
            target_position: Position::new(0.0, 0.0),
            last_direction_change: current_time,
            direction_change_interval: rng.gen_range(2.0..5.0),
            aggression_level: base_aggression,
            difficulty,
            last_player_position: Position::new(0.0, 0.0),
            player_velocity_estimate: (0.0, 0.0),
            state: EnemyState::Patrol,
            state_change_time: current_time,
            last_shot_time: current_time,
            suppressed_until: current_time,
            patrol_target: Position::new(rng.gen_range(100.0..900.0), rng.gen_range(100.0..600.0)),
            last_seen_player_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
    ) {
        let current_time = get_time();

        // 更新玩家速度估计（用于预测瞄准）
        self.update_player_velocity_estimate(player_tank);

        // 计算到玩家的距离
        let distance_to_player = enemy_tank.position.distance_to(&player_tank.position);

        // 检查玩家是否在视线范围内
        let has_line_of_sight = self.check_line_of_sight(enemy_tank, player_tank, obstacles);
        if has_line_of_sight {
            self.last_seen_player_time = current_time;
        }

        // 检查是否被火力压制
        let under_fire = self.check_if_under_fire(enemy_tank);
        if under_fire {
            self.suppressed_until = current_time + 2.0; // 压制2秒
        }

        // 智能状态机
        self.update_state(
            enemy_tank,
            player_tank,
            distance_to_player,
            has_line_of_sight,
            current_time,
        );

        // 根据状态执行行为
        match self.state {
            EnemyState::Patrol => self.execute_patrol(enemy_tank, obstacles, current_time),
            EnemyState::Chase => {
                self.execute_chase(enemy_tank, player_tank, obstacles, distance_to_player)
            }
            EnemyState::Attack => {
                self.execute_attack(enemy_tank, player_tank, obstacles, distance_to_player)
            }
            EnemyState::Evade => self.execute_evade(enemy_tank, player_tank, obstacles),
            EnemyState::Suppress => self.execute_suppress(enemy_tank, player_tank, obstacles),
            EnemyState::Flank => self.execute_flank(enemy_tank, player_tank, obstacles),
        }

        // 首先检查边界避让
        if self.check_and_avoid_boundaries(enemy_tank) {
            // 如果正在避开边界，不执行其他移动逻辑
        } else if self.check_and_avoid_obstacles(enemy_tank, obstacles) {
            // 如果正在避开障碍物，不执行其他移动逻辑
        }

        // 根据难度调整瞄准行为
        let mut rng = thread_rng();
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
                self.last_shot_time = current_time;
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
        let predicted_x =
            player_tank.position.x + self.player_velocity_estimate.0 * flight_time * 60.0; // 60fps
        let predicted_y =
            player_tank.position.y + self.player_velocity_estimate.1 * flight_time * 60.0;

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
            let distance_factor =
                (enemy_tank.position.x - (screen_width - boundary_margin)) / boundary_margin;
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
            let distance_factor =
                (enemy_tank.position.y - (screen_height - boundary_margin)) / boundary_margin;
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

    fn check_and_avoid_obstacles(
        &mut self,
        enemy_tank: &mut Tank,
        obstacles: &[super::Obstacle],
    ) -> bool {
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
                let distance_factor =
                    1.0 - ((distance - obstacle_radius) / detection_distance).max(0.0);

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

    // 新增智能AI方法
    fn update_state(
        &mut self,
        enemy_tank: &Tank,
        player_tank: &Tank,
        distance: f32,
        has_los: bool,
        current_time: f64,
    ) {
        let time_since_state_change = current_time - self.state_change_time;
        let min_state_duration = if self.difficulty <= 1.0 { 3.0 } else { 2.0 };

        // 如果刚切换状态，等待一段时间
        if time_since_state_change < min_state_duration {
            return;
        }

        let new_state = if current_time < self.suppressed_until {
            // 被火力压制，优先躲避
            EnemyState::Evade
        } else if !has_los && current_time - self.last_seen_player_time > 5.0 {
            // 长时间没看到玩家，回到巡逻状态
            EnemyState::Patrol
        } else if distance < 100.0 {
            // 玩家太近，判断是否需要躲避
            let mut rng = thread_rng();
            if rng.gen::<f32>() < self.aggression_level {
                EnemyState::Attack
            } else {
                EnemyState::Evade
            }
        } else if has_los && distance < 300.0 {
            // 在视线范围内且距离适中
            let mut rng = thread_rng();
            if self.difficulty > 1.5 && rng.gen::<f32>() < 0.3 {
                // 高难度时有概率侧翼包抄
                EnemyState::Flank
            } else if rng.gen::<f32>() < self.aggression_level {
                EnemyState::Attack
            } else {
                EnemyState::Chase
            }
        } else if has_los {
            // 能看到但距离较远
            EnemyState::Chase
        } else {
            // 看不到玩家
            EnemyState::Patrol
        };

        if new_state != self.state {
            self.state = new_state;
            self.state_change_time = current_time;
        }
    }

    fn check_line_of_sight(
        &self,
        enemy_tank: &Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
    ) -> bool {
        // 检查玩家是否在草地的隐蔽中
        if player_tank.is_in_cover(obstacles) {
            // 如果玩家在隐蔽中，需要更近的距离才能看到
            let distance = enemy_tank.position.distance_to(&player_tank.position);
            if distance > 150.0 {
                return false;
            }
        }

        // 简化的视线检测：检查两点之间是否有阻挡视线的障碍物
        let steps = 20;
        for i in 1..steps {
            let t = i as f32 / steps as f32;
            let check_x =
                enemy_tank.position.x + (player_tank.position.x - enemy_tank.position.x) * t;
            let check_y =
                enemy_tank.position.y + (player_tank.position.y - enemy_tank.position.y) * t;
            let check_pos = Position::new(check_x, check_y);

            for obstacle in obstacles {
                // 只有墙体和钢墙会阻挡视线
                if obstacle.blocks_movement() && obstacle.collides_with_circle(&check_pos, 5.0) {
                    return false;
                }
            }
        }
        true
    }

    fn check_if_under_fire(&self, enemy_tank: &Tank) -> bool {
        // 简化检测：检查附近是否有子弹
        // 这里需要外部传入子弹信息，暂时返回false
        false
    }

    fn execute_patrol(
        &mut self,
        enemy_tank: &mut Tank,
        obstacles: &[super::Obstacle],
        current_time: f64,
    ) {
        let distance_to_target = enemy_tank.position.distance_to(&self.patrol_target);

        if distance_to_target < 50.0 || current_time - self.last_direction_change > 5.0 {
            // 到达目标点或时间太长，生成新的巡逻点
            let mut rng = thread_rng();
            self.patrol_target =
                Position::new(rng.gen_range(100.0..900.0), rng.gen_range(100.0..600.0));
            self.last_direction_change = current_time;
        }

        // 向巡逻点移动
        let dx = self.patrol_target.x - enemy_tank.position.x;
        let dy = self.patrol_target.y - enemy_tank.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            let speed_factor = 0.5; // 巡逻时移动较慢
            enemy_tank.velocity.x = (dx / distance) * enemy_tank.speed * speed_factor;
            enemy_tank.velocity.y = (dy / distance) * enemy_tank.speed * speed_factor;
        }
    }

    fn execute_chase(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
        distance: f32,
    ) {
        if distance > 200.0 {
            // 追击玩家
            self.approach_player(enemy_tank, player_tank);
        } else {
            // 距离合适，准备攻击
            self.execute_attack(enemy_tank, player_tank, obstacles, distance);
        }
    }

    fn execute_attack(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
        distance: f32,
    ) {
        let optimal_distance = if self.difficulty <= 1.0 { 200.0 } else { 150.0 };

        if distance > optimal_distance + 50.0 {
            // 太远了，接近
            self.approach_player(enemy_tank, player_tank);
        } else if distance < optimal_distance - 50.0 {
            // 太近了，后退
            self.retreat_from_player(enemy_tank, player_tank);
        } else {
            // 在最佳攻击距离，进行战术移动
            self.tactical_movement(enemy_tank, player_tank);
        }
    }

    fn execute_evade(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
    ) {
        // 远离玩家，同时寻找掩护
        self.retreat_from_player(enemy_tank, player_tank);

        // 寻找最近的障碍物作为掩护
        let mut best_cover: Option<&super::Obstacle> = None;
        let mut best_distance = f32::MAX;

        for obstacle in obstacles {
            let obs_center_x = obstacle.position.x + obstacle.width / 2.0;
            let obs_center_y = obstacle.position.y + obstacle.height / 2.0;
            let obs_distance = enemy_tank
                .position
                .distance_to(&Position::new(obs_center_x, obs_center_y));

            if obs_distance < best_distance && obs_distance > enemy_tank.size + 20.0 {
                best_distance = obs_distance;
                best_cover = Some(obstacle);
            }
        }

        if let Some(cover) = best_cover {
            // 向掩体移动
            let cover_x = cover.position.x + cover.width / 2.0;
            let cover_y = cover.position.y + cover.height / 2.0;
            let dx = cover_x - enemy_tank.position.x;
            let dy = cover_y - enemy_tank.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 0.0 {
                let cover_factor = 0.3; // 混合躲避和寻找掩护
                enemy_tank.velocity.x = enemy_tank.velocity.x * (1.0 - cover_factor)
                    + (dx / distance) * enemy_tank.speed * cover_factor;
                enemy_tank.velocity.y = enemy_tank.velocity.y * (1.0 - cover_factor)
                    + (dy / distance) * enemy_tank.speed * cover_factor;
            }
        }
    }

    fn execute_suppress(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
    ) {
        // 压制状态：快速移动并寻找掩护
        self.execute_evade(enemy_tank, player_tank, obstacles);
    }

    fn execute_flank(
        &mut self,
        enemy_tank: &mut Tank,
        player_tank: &Tank,
        obstacles: &[super::Obstacle],
    ) {
        // 侧翼包抄：绕到玩家侧面
        let dx = player_tank.position.x - enemy_tank.position.x;
        let dy = player_tank.position.y - enemy_tank.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            // 计算侧向移动向量
            let flank_x = -dy / distance;
            let flank_y = dx / distance;

            // 选择左侧或右侧
            let mut rng = thread_rng();
            let side_factor = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            // 混合侧向移动和接近
            let approach_factor = 0.3;
            let flank_factor = 0.7;

            enemy_tank.velocity.x = (dx / distance) * approach_factor * enemy_tank.speed
                + flank_x * side_factor * flank_factor * enemy_tank.speed;
            enemy_tank.velocity.y = (dy / distance) * approach_factor * enemy_tank.speed
                + flank_y * side_factor * flank_factor * enemy_tank.speed;
        }
    }
}
