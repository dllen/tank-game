use crate::entities::*;

pub fn check_bullet_tank_collisions(
    bullets: &mut Vec<Bullet>,
    tanks: &mut Vec<Tank>,
    player_tank: &mut Tank,
) -> Vec<usize> {
    let mut tanks_to_remove = Vec::new();
    let mut bullets_to_remove = Vec::new();
    
    for (bullet_idx, bullet) in bullets.iter().enumerate() {
        // 检查子弹与玩家坦克的碰撞
        if !bullet.from_player && bullet.collides_with_circle(&player_tank.position, player_tank.size) {
            if player_tank.take_damage(bullet.damage) {
                // 玩家死亡将在游戏主循环中处理
            }
            bullets_to_remove.push(bullet_idx);
            continue;
        }
        
        // 检查子弹与敌方坦克的碰撞
        for (tank_idx, tank) in tanks.iter_mut().enumerate() {
            if bullet.from_player && bullet.collides_with_circle(&tank.position, tank.size) {
                if tank.take_damage(bullet.damage) {
                    tanks_to_remove.push(tank_idx);
                }
                bullets_to_remove.push(bullet_idx);
                break;
            }
        }
    }
    
    // 移除被击中的子弹（从后往前移除以避免索引问题）
    bullets_to_remove.sort_unstable();
    bullets_to_remove.reverse();
    for idx in bullets_to_remove {
        if idx < bullets.len() {
            bullets.remove(idx);
        }
    }
    
    tanks_to_remove
}

pub fn check_bullet_obstacle_collisions(bullets: &mut Vec<Bullet>, obstacles: &mut Vec<Obstacle>) {
    let mut bullets_to_remove = Vec::new();
    let mut obstacles_to_remove = Vec::new();
    
    for (bullet_idx, bullet) in bullets.iter().enumerate() {
        for (obstacle_idx, obstacle) in obstacles.iter_mut().enumerate() {
            if obstacle.collides_with_circle(&bullet.position, bullet.size) {
                if obstacle.take_damage(bullet.damage) {
                    obstacles_to_remove.push(obstacle_idx);
                }
                bullets_to_remove.push(bullet_idx);
                break;
            }
        }
    }
    
    // 移除被击中的子弹和障碍物
    bullets_to_remove.sort_unstable();
    bullets_to_remove.reverse();
    for idx in bullets_to_remove {
        if idx < bullets.len() {
            bullets.remove(idx);
        }
    }
    
    obstacles_to_remove.sort_unstable();
    obstacles_to_remove.reverse();
    for idx in obstacles_to_remove {
        if idx < obstacles.len() {
            obstacles.remove(idx);
        }
    }
}

pub fn check_tank_obstacle_collisions(tank: &mut Tank, obstacles: &[Obstacle]) {
    for obstacle in obstacles {
        if obstacle.collides_with_circle(&tank.position, tank.size) {
            // 简单的碰撞响应：停止移动
            tank.velocity.x = 0.0;
            tank.velocity.y = 0.0;
            
            // 将坦克推出障碍物
            let center_x = obstacle.position.x + obstacle.width / 2.0;
            let center_y = obstacle.position.y + obstacle.height / 2.0;
            
            let dx = tank.position.x - center_x;
            let dy = tank.position.y - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 0.0 {
                let push_distance = tank.size + 5.0;
                tank.position.x = center_x + (dx / distance) * push_distance;
                tank.position.y = center_y + (dy / distance) * push_distance;
            }
        }
    }
}

pub fn check_powerup_collisions(tank: &mut Tank, powerups: &mut Vec<PowerUp>) -> Vec<PowerUpType> {
    let mut collected_powerups = Vec::new();
    let mut powerups_to_remove = Vec::new();
    
    for (idx, powerup) in powerups.iter_mut().enumerate() {
        if powerup.collides_with_circle(&tank.position, tank.size) {
            powerup.collect();
            collected_powerups.push(powerup.power_type.clone());
            powerups_to_remove.push(idx);
        }
    }
    
    // 移除被收集的道具
    powerups_to_remove.reverse();
    for idx in powerups_to_remove {
        if idx < powerups.len() {
            powerups.remove(idx);
        }
    }
    
    collected_powerups
}