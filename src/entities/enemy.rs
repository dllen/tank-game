use super::{Position, Tank};
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

pub struct EnemyAI {
    #[allow(dead_code)]
    pub target_position: Position,
    pub last_direction_change: f64,
    pub direction_change_interval: f64,
    pub aggression_level: f32,
}

impl EnemyAI {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            target_position: Position::new(0.0, 0.0),
            last_direction_change: get_time(),
            direction_change_interval: rng.gen_range(1.0..3.0),
            aggression_level: rng.gen_range(0.3..0.8),
        }
    }
    
    pub fn update(&mut self, enemy_tank: &mut Tank, player_tank: &Tank, _obstacles: &[super::Obstacle]) {
        let current_time = get_time();
        
        // 计算到玩家的距离
        let distance_to_player = enemy_tank.position.distance_to(&player_tank.position);
        
        // 如果太近，后退
        if distance_to_player < 100.0 {
            self.retreat_from_player(enemy_tank, player_tank);
        } else if distance_to_player > 200.0 {
            // 如果太远，接近玩家
            self.approach_player(enemy_tank, player_tank);
        } else {
            // 在合适距离内，随机移动
            if current_time - self.last_direction_change > self.direction_change_interval {
                self.random_movement(enemy_tank);
                self.last_direction_change = current_time;
                let mut rng = thread_rng();
                self.direction_change_interval = rng.gen_range(1.0..3.0);
            }
        }
        
        // 瞄准玩家
        self.aim_at_player(enemy_tank, player_tank);
        
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
    
    fn aim_at_player(&mut self, enemy_tank: &mut Tank, player_tank: &Tank) {
        let dx = player_tank.position.x - enemy_tank.position.x;
        let dy = player_tank.position.y - enemy_tank.position.y;
        enemy_tank.angle = dy.atan2(dx);
    }
    
    pub fn should_shoot(&self, _enemy_tank: &Tank, _player_tank: &Tank, distance: f32) -> bool {
        // 基于距离和攻击性决定是否射击
        let distance_factor = if distance < 150.0 { 1.0 } else { 0.5 };
        let mut rng = thread_rng();
        rng.gen::<f32>() < self.aggression_level * distance_factor * 0.02
    }
}