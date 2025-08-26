use crate::entities::*;
use crate::systems::*;
use crate::ui::GameUI;
use crate::math_challenge::MathChallenge;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
    MathChallenge,
}

pub struct Game {
    pub state: GameState,
    pub player_tank: Tank,
    pub enemy_tanks: Vec<Tank>,
    pub enemy_ais: Vec<EnemyAI>,
    pub bullets: Vec<Bullet>,
    pub obstacles: Vec<Obstacle>,
    pub powerups: Vec<PowerUp>,
    pub spawn_system: SpawnSystem,
    pub ui: GameUI,
    pub score: i32,
    pub high_score: i32,
    pub wave: i32,
    pub enemies_killed_this_wave: i32,
    pub enemies_per_wave: i32,
    pub difficulty: f32,
    pub last_difficulty_increase: f64,
    pub math_challenge: Option<MathChallenge>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            state: GameState::Menu,
            player_tank: Tank::new_player(400.0, 300.0),
            enemy_tanks: Vec::new(),
            enemy_ais: Vec::new(),
            bullets: Vec::new(),
            obstacles: Vec::new(),
            powerups: Vec::new(),
            spawn_system: SpawnSystem::new(1.0),
            ui: GameUI::new(),
            score: 0,
            high_score: 0,
            wave: 1,
            enemies_killed_this_wave: 0,
            enemies_per_wave: 5,
            difficulty: 1.0,
            last_difficulty_increase: 0.0,
            math_challenge: None,
        };
        
        game.generate_obstacles();
        game
    }
    
    pub fn start_game(&mut self, difficulty: f32) {
        self.difficulty = difficulty;
        self.state = GameState::Playing;
        self.player_tank = Tank::new_player(screen_width() / 2.0, screen_height() / 2.0);
        self.enemy_tanks.clear();
        self.enemy_ais.clear();
        self.bullets.clear();
        self.powerups.clear();
        self.spawn_system = SpawnSystem::new(difficulty);
        self.score = 0;
        self.wave = 1;
        self.enemies_killed_this_wave = 0;
        self.enemies_per_wave = 5;
        self.last_difficulty_increase = get_time();
        self.math_challenge = None;
        self.generate_obstacles();
    }
    
    fn generate_obstacles(&mut self) {
        self.obstacles.clear();
        let mut rng = thread_rng();
        
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // 生成随机障碍物
        for _ in 0..15 {
            let x = rng.gen_range(50.0..screen_w - 100.0);
            let y = rng.gen_range(50.0..screen_h - 100.0);
            let width = rng.gen_range(30.0..80.0);
            let height = rng.gen_range(30.0..80.0);
            
            // 确保不在玩家起始位置附近
            if (x - screen_w / 2.0).abs() > 100.0 || (y - screen_h / 2.0).abs() > 100.0 {
                if rng.gen_bool(0.8) {
                    self.obstacles.push(Obstacle::new_wall(x, y, width, height));
                } else {
                    self.obstacles.push(Obstacle::new_steel(x, y, width, height));
                }
            }
        }
        
        // 添加边界墙
        let wall_thickness = 20.0;
        self.obstacles.push(Obstacle::new_steel(0.0, 0.0, screen_w, wall_thickness));
        self.obstacles.push(Obstacle::new_steel(0.0, screen_h - wall_thickness, screen_w, wall_thickness));
        self.obstacles.push(Obstacle::new_steel(0.0, 0.0, wall_thickness, screen_h));
        self.obstacles.push(Obstacle::new_steel(screen_w - wall_thickness, 0.0, wall_thickness, screen_h));
    }
    
    pub async fn update(&mut self) {
        match self.state {
            GameState::Menu => self.update_menu().await,
            GameState::Playing => self.update_playing().await,
            GameState::Paused => self.update_paused().await,
            GameState::GameOver => self.update_game_over().await,
            GameState::MathChallenge => self.update_math_challenge().await,
        }
    }
    
    async fn update_menu(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.start_game(1.0);
        } else if is_key_pressed(KeyCode::Key2) {
            self.start_game(1.5);
        } else if is_key_pressed(KeyCode::Key3) {
            self.start_game(2.0);
        }
    }
    
    async fn update_playing(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Paused;
            return;
        }
        
        let dt = get_frame_time();
        
        // 处理玩家输入
        let new_bullets = handle_player_input(&mut self.player_tank, dt);
        self.bullets.extend(new_bullets);
        
        // 更新玩家坦克 - 使用安全移动
        self.player_tank.safe_move(dt, &self.obstacles);
        
        // 更新敌方坦克
        for (tank, ai) in self.enemy_tanks.iter_mut().zip(self.enemy_ais.iter_mut()) {
            ai.update(tank, &self.player_tank, &self.obstacles);
            // 使用安全移动，防止卡在障碍物中
            tank.safe_move(dt, &self.obstacles);
            
            // 敌方坦克射击
            let distance = tank.position.distance_to(&self.player_tank.position);
            if tank.can_shoot() && ai.should_shoot(tank, &self.player_tank, distance) {
                tank.shoot();
                let bullet_x = tank.position.x + tank.angle.cos() * (tank.size + 5.0);
                let bullet_y = tank.position.y + tank.angle.sin() * (tank.size + 5.0);
                self.bullets.push(Bullet::new(bullet_x, bullet_y, tank.angle, false));
            }
        }
        
        // 更新子弹
        self.bullets.retain(|bullet| {
            let mut b = bullet.clone();
            b.update(dt)
        });
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        
        // 更新道具
        self.powerups.retain(|powerup| {
            let mut p = powerup.clone();
            p.update(dt)
        });
        for powerup in &mut self.powerups {
            powerup.update(dt);
        }
        
        // 碰撞检测
        let destroyed_tanks = check_bullet_tank_collisions(&mut self.bullets, &mut self.enemy_tanks, &mut self.player_tank);
        
        // 移除被摧毁的敌方坦克和对应的AI
        for &tank_idx in destroyed_tanks.iter().rev() {
            if tank_idx < self.enemy_tanks.len() {
                self.enemy_tanks.remove(tank_idx);
                self.enemy_ais.remove(tank_idx);
                self.score += 100;
                self.enemies_killed_this_wave += 1;
            }
        }
        
        check_bullet_obstacle_collisions(&mut self.bullets, &mut self.obstacles);
        
        // 处理道具收集
        let collected_powerups = check_powerup_collisions(&mut self.player_tank, &mut self.powerups);
        for powerup_type in collected_powerups {
            self.apply_powerup(powerup_type);
        }
        
        // 生成系统更新
        self.spawn_system.update(&mut self.enemy_tanks, &mut self.powerups, &self.obstacles);
        
        // 为新生成的敌人创建AI
        while self.enemy_ais.len() < self.enemy_tanks.len() {
            self.enemy_ais.push(EnemyAI::new_with_difficulty(self.difficulty));
        }
        
        // 检查波数完成
        if self.enemies_killed_this_wave >= self.enemies_per_wave && self.enemy_tanks.is_empty() {
            self.next_wave();
        }
        
        // 定期增加难度
        if get_time() - self.last_difficulty_increase > 30.0 {
            self.spawn_system.increase_difficulty();
            self.last_difficulty_increase = get_time();
        }
        
        // 检查玩家死亡
        if self.player_tank.health <= 0 {
            // 生成数学挑战
            self.math_challenge = Some(MathChallenge::new_random());
            self.state = GameState::MathChallenge;
        }
    }
    
    async fn update_paused(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Playing;
        }
    }
    
    async fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::R) {
            self.state = GameState::Menu;
        }
    }
    
    async fn update_math_challenge(&mut self) {
        if let Some(ref mut challenge) = self.math_challenge {
            // 处理数字输入
            for key_code in [
                KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
            ] {
                if is_key_pressed(key_code) {
                    let digit = match key_code {
                        KeyCode::Key0 => '0',
                        KeyCode::Key1 => '1',
                        KeyCode::Key2 => '2',
                        KeyCode::Key3 => '3',
                        KeyCode::Key4 => '4',
                        KeyCode::Key5 => '5',
                        KeyCode::Key6 => '6',
                        KeyCode::Key7 => '7',
                        KeyCode::Key8 => '8',
                        KeyCode::Key9 => '9',
                        _ => continue,
                    };
                    challenge.add_digit(digit);
                }
            }
            
            // 处理退格键
            if is_key_pressed(KeyCode::Backspace) {
                challenge.remove_digit();
            }
            
            // 处理回车键提交答案
            if is_key_pressed(KeyCode::Enter) {
                if challenge.submit_answer() {
                    // 答案正确，复活玩家
                    self.player_tank.health = self.player_tank.max_health / 2; // 复活时恢复一半血量
                    self.math_challenge = None;
                    self.state = GameState::Playing;
                } else {
                    // 答案错误，游戏结束
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                    self.math_challenge = None;
                    self.state = GameState::GameOver;
                }
            }
            
            // ESC键直接游戏结束
            if is_key_pressed(KeyCode::Escape) {
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                self.math_challenge = None;
                self.state = GameState::GameOver;
            }
        }
    }
    
    fn apply_powerup(&mut self, powerup_type: PowerUpType) {
        match powerup_type {
            PowerUpType::Health => {
                self.player_tank.heal(50);
                self.score += 20;
            }
            PowerUpType::Shield => {
                self.player_tank.add_shield(30.0);
                self.score += 30;
            }
            PowerUpType::ScatterShot => {
                self.player_tank.scatter_shot = true;
                // 散弹效果持续15秒
                self.score += 25;
            }
            PowerUpType::SpeedBoost => {
                self.player_tank.speed = (self.player_tank.speed * 1.5).min(300.0);
                self.score += 25;
            }
            PowerUpType::Damage => {
                // 这里可以增加伤害，暂时增加分数
                self.score += 40;
            }
        }
    }
    
    fn next_wave(&mut self) {
        self.wave += 1;
        self.enemies_killed_this_wave = 0;
        self.enemies_per_wave += 2;
        self.score += self.wave * 50;
        
        // 增加难度
        self.spawn_system.increase_difficulty();
        
        // 恢复玩家一些生命值
        self.player_tank.heal(25);
    }
    
    pub async fn draw(&self) {
        clear_background(BLACK);
        
        match self.state {
            GameState::Menu => {
                self.ui.draw_start_menu(self.high_score);
            }
            GameState::Playing => {
                self.draw_game();
                self.ui.draw_hud(
                    self.player_tank.health,
                    self.player_tank.max_health,
                    self.score,
                    self.wave,
                    self.difficulty,
                );
            }
            GameState::Paused => {
                self.draw_game();
                self.ui.draw_pause_menu();
            }
            GameState::GameOver => {
                self.draw_game();
                self.ui.draw_game_over(self.score, self.wave, self.high_score);
            }
            GameState::MathChallenge => {
                self.draw_game();
                if let Some(ref challenge) = self.math_challenge {
                    self.ui.draw_math_challenge(challenge);
                }
            }
        }
    }
    
    fn draw_game(&self) {
        // 绘制障碍物
        for obstacle in &self.obstacles {
            obstacle.draw();
        }
        
        // 绘制道具
        for powerup in &self.powerups {
            powerup.draw();
        }
        
        // 绘制坦克
        self.player_tank.draw();
        for tank in &self.enemy_tanks {
            tank.draw();
        }
        
        // 绘制子弹
        for bullet in &self.bullets {
            bullet.draw();
        }
    }
}