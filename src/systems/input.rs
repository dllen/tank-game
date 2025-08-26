use crate::entities::*;
use macroquad::prelude::*;

pub fn handle_player_input(player_tank: &mut Tank, dt: f32) -> Vec<Bullet> {
    let mut bullets = Vec::new();
    
    // 移动控制 - 支持长按方向键，增加加速度效果
    let mut move_x = 0.0;
    let mut move_y = 0.0;
    
    // 检查所有方向键的长按状态
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        move_y -= 1.0;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        move_y += 1.0;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        move_x -= 1.0;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        move_x += 1.0;
    }
    
    // 计算目标速度
    let move_length = ((move_x * move_x + move_y * move_y) as f32).sqrt();
    let target_velocity_x;
    let target_velocity_y;
    
    if move_length > 0.0 {
        // 标准化方向向量，确保对角线移动不会更快
        let normalized_x = move_x / move_length;
        let normalized_y = move_y / move_length;
        
        // 计算目标速度
        target_velocity_x = normalized_x * player_tank.speed;
        target_velocity_y = normalized_y * player_tank.speed;
    } else {
        // 没有按键时目标速度为0
        target_velocity_x = 0.0;
        target_velocity_y = 0.0;
    }
    
    // 使用插值实现更平滑的加速和减速
    let acceleration = 1200.0; // 加速度
    let deceleration = 1500.0; // 减速度
    
    // X轴速度调整
    let vel_diff_x = target_velocity_x - player_tank.velocity.x;
    if vel_diff_x.abs() > 0.1 {
        let accel_rate = if target_velocity_x.abs() > player_tank.velocity.x.abs() {
            acceleration
        } else {
            deceleration
        };
        let max_change = accel_rate * dt;
        let change = vel_diff_x.signum() * max_change.min(vel_diff_x.abs());
        player_tank.velocity.x += change;
    } else {
        player_tank.velocity.x = target_velocity_x;
    }
    
    // Y轴速度调整
    let vel_diff_y = target_velocity_y - player_tank.velocity.y;
    if vel_diff_y.abs() > 0.1 {
        let accel_rate = if target_velocity_y.abs() > player_tank.velocity.y.abs() {
            acceleration
        } else {
            deceleration
        };
        let max_change = accel_rate * dt;
        let change = vel_diff_y.signum() * max_change.min(vel_diff_y.abs());
        player_tank.velocity.y += change;
    } else {
        player_tank.velocity.y = target_velocity_y;
    }
    
    // 转向控制 - 坦克整体转向
    if move_length > 0.0 {
        // 根据移动方向设置坦克角度
        player_tank.angle = move_y.atan2(move_x);
    }
    
    // 射击控制
    if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) {
        if player_tank.can_shoot() {
            player_tank.shoot();
            
            if player_tank.scatter_shot {
                // 散弹射击
                let spread_angles = [-0.3, -0.15, 0.0, 0.15, 0.3];
                for &spread in &spread_angles {
                    let bullet_x = player_tank.position.x + player_tank.angle.cos() * (player_tank.size + 5.0);
                    let bullet_y = player_tank.position.y + player_tank.angle.sin() * (player_tank.size + 5.0);
                    bullets.push(Bullet::new_scatter(bullet_x, bullet_y, player_tank.angle, spread, true));
                }
            } else {
                // 普通射击
                let bullet_x = player_tank.position.x + player_tank.angle.cos() * (player_tank.size + 5.0);
                let bullet_y = player_tank.position.y + player_tank.angle.sin() * (player_tank.size + 5.0);
                bullets.push(Bullet::new(bullet_x, bullet_y, player_tank.angle, true));
            }
        }
    }
    
    bullets
}