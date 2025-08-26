use crate::entities::*;
use macroquad::prelude::*;

pub fn handle_player_input(player_tank: &mut Tank, _dt: f32) -> Vec<Bullet> {
    let mut bullets = Vec::new();
    
    // 移动控制
    let mut move_x = 0.0;
    let mut move_y = 0.0;
    
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
    
    // 标准化移动向量
    let move_length = ((move_x * move_x + move_y * move_y) as f32).sqrt();
    if move_length > 0.0 {
        player_tank.velocity.x = (move_x / move_length) * player_tank.speed;
        player_tank.velocity.y = (move_y / move_length) * player_tank.speed;
    } else {
        player_tank.velocity.x = 0.0;
        player_tank.velocity.y = 0.0;
    }
    
    // 瞄准控制（鼠标）
    let (mouse_x, mouse_y) = mouse_position();
    let dx = mouse_x - player_tank.position.x;
    let dy = mouse_y - player_tank.position.y;
    player_tank.angle = dy.atan2(dx);
    
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