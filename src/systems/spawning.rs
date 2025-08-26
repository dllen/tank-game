use crate::entities::*;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

pub struct SpawnSystem {
    pub last_enemy_spawn: f64,
    pub enemy_spawn_interval: f64,
    pub last_powerup_spawn: f64,
    pub powerup_spawn_interval: f64,
    pub max_enemies: usize,
    pub difficulty_multiplier: f32,
}

impl SpawnSystem {
    pub fn new(difficulty: f32) -> Self {
        Self {
            last_enemy_spawn: 0.0,
            enemy_spawn_interval: 3.0 / difficulty as f64,
            last_powerup_spawn: 0.0,
            powerup_spawn_interval: 8.0,
            max_enemies: 4, // 固定最大敌方坦克数量为4辆
            difficulty_multiplier: difficulty,
        }
    }
    
    pub fn update(&mut self, enemies: &mut Vec<Tank>, powerups: &mut Vec<PowerUp>, obstacles: &[Obstacle]) {
        let current_time = get_time();
        
        // 生成敌人
        if current_time - self.last_enemy_spawn > self.enemy_spawn_interval && enemies.len() < self.max_enemies {
            if let Some(spawn_pos) = self.find_safe_spawn_position(obstacles) {
                let mut enemy = Tank::new_enemy(spawn_pos.x, spawn_pos.y);
                // 根据难度调整敌人属性
                enemy.health = (enemy.health as f32 * self.difficulty_multiplier) as i32;
                // 调整速度：容易模式稍微快一些，其他模式按原来的逻辑
                if self.difficulty_multiplier <= 1.0 {
                    // 容易模式：基础速度提升100%，射击更快
                    enemy.speed *= 2.0;
                    enemy.shot_cooldown = 0.6; // 减少射击冷却时间
                } else {
                    // 其他模式：按原来的公式
                    enemy.speed *= 1.0 + (self.difficulty_multiplier - 1.0) * 0.5;
                }
                enemies.push(enemy);
                self.last_enemy_spawn = current_time;
            }
        }
        
        // 生成道具
        if current_time - self.last_powerup_spawn > self.powerup_spawn_interval {
            if let Some(spawn_pos) = self.find_safe_spawn_position(obstacles) {
                powerups.push(PowerUp::new_random(spawn_pos.x, spawn_pos.y));
                self.last_powerup_spawn = current_time;
            }
        }
    }
    
    fn find_safe_spawn_position(&self, obstacles: &[Obstacle]) -> Option<Position> {
        let mut rng = thread_rng();
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        for _ in 0..20 {  // 最多尝试20次
            let x = rng.gen_range(50.0..screen_width - 50.0);
            let y = rng.gen_range(50.0..screen_height - 50.0);
            let pos = Position::new(x, y);
            
            // 检查是否与障碍物重叠
            let mut safe = true;
            for obstacle in obstacles {
                if obstacle.collides_with_circle(&pos, 30.0) {
                    safe = false;
                    break;
                }
            }
            
            if safe {
                return Some(pos);
            }
        }
        
        None
    }
    
    pub fn increase_difficulty(&mut self) {
        self.difficulty_multiplier += 0.1;
        self.enemy_spawn_interval = (3.0 / self.difficulty_multiplier as f64).max(0.5);
        // 保持最大敌方坦克数量为4辆，不随难度增加
        self.max_enemies = 4;
    }
}