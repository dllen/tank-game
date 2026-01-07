use super::Position;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainType {
    Wall,  // 普通墙体，可摧毁
    Steel, // 钢墙，不可摧毁
    River, // 河流，减速
    Grass, // 草地，提供隐蔽
    Sand,  // 沙地，轻度减速
    Ice,   // 冰面，滑行
}

#[derive(Clone)]
pub struct Obstacle {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    pub health: i32,
    pub max_health: i32,
    pub terrain_type: TerrainType,
}

impl Obstacle {
    pub fn new_wall(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 100,
            max_health: 100,
            terrain_type: TerrainType::Wall,
        }
    }

    pub fn new_steel(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1000,
            max_health: 1000,
            terrain_type: TerrainType::Steel,
        }
    }

    pub fn new_river(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1,
            max_health: 1,
            terrain_type: TerrainType::River,
        }
    }

    pub fn new_grass(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1,
            max_health: 1,
            terrain_type: TerrainType::Grass,
        }
    }

    pub fn new_sand(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1,
            max_health: 1,
            terrain_type: TerrainType::Sand,
        }
    }

    pub fn new_ice(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
            health: 1,
            max_health: 1,
            terrain_type: TerrainType::Ice,
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        if !self.is_destructible() {
            return false;
        }

        self.health -= damage;
        self.health <= 0
    }

    pub fn is_destructible(&self) -> bool {
        match self.terrain_type {
            TerrainType::Wall => true,
            TerrainType::Steel => false,
            TerrainType::River | TerrainType::Grass | TerrainType::Sand | TerrainType::Ice => false,
        }
    }

    pub fn blocks_movement(&self) -> bool {
        match self.terrain_type {
            TerrainType::Wall | TerrainType::Steel => true,
            TerrainType::River | TerrainType::Grass | TerrainType::Sand | TerrainType::Ice => false,
        }
    }

    pub fn get_speed_modifier(&self) -> f32 {
        match self.terrain_type {
            TerrainType::River => 0.5, // 河流减速50%
            TerrainType::Sand => 0.7,  // 沙地减速30%
            TerrainType::Grass => 0.9, // 草地轻度减速
            TerrainType::Ice => 1.3,   // 冰面加速30%（滑行效果）
            TerrainType::Wall | TerrainType::Steel => 1.0,
        }
    }

    pub fn provides_cover(&self) -> bool {
        match self.terrain_type {
            TerrainType::Grass => true,                     // 草地提供隐蔽
            TerrainType::Wall | TerrainType::Steel => true, // 墙体提供遮挡
            _ => false,
        }
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x
            && x <= self.position.x + self.width
            && y >= self.position.y
            && y <= self.position.y + self.height
    }

    pub fn collides_with_circle(&self, pos: &Position, radius: f32) -> bool {
        let closest_x = pos.x.clamp(self.position.x, self.position.x + self.width);
        let closest_y = pos.y.clamp(self.position.y, self.position.y + self.height);

        let distance = ((pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2)).sqrt();
        distance < radius
    }

    pub fn draw(&self) {
        let (color, border_color) = match self.terrain_type {
            TerrainType::Wall => {
                let health_ratio = self.health as f32 / self.max_health as f32;
                (Color::new(0.6 * health_ratio, 0.3, 0.1, 1.0), DARKGRAY)
            }
            TerrainType::Steel => (GRAY, BLACK),
            TerrainType::River => (Color::new(0.2, 0.4, 0.8, 0.7), BLUE),
            TerrainType::Grass => (Color::new(0.2, 0.6, 0.2, 0.8), DARKGREEN),
            TerrainType::Sand => (
                Color::new(0.8, 0.7, 0.4, 0.9),
                Color::new(0.6, 0.5, 0.2, 1.0),
            ),
            TerrainType::Ice => (Color::new(0.8, 0.9, 1.0, 0.6), LIGHTGRAY),
        };

        // 绘制主体
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            color,
        );

        // 绘制边框
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            2.0,
            border_color,
        );

        // 添加特殊效果
        match self.terrain_type {
            TerrainType::River => {
                // 添加水流效果
                let time = macroquad::prelude::get_time() as f32;
                for i in 0..3 {
                    let wave_y = self.position.y + (i + 1) as f32 * self.height / 4.0;
                    let wave_offset = (time * 2.0 + i as f32).sin() * 5.0;
                    draw_line(
                        self.position.x + wave_offset,
                        wave_y,
                        self.position.x + self.width + wave_offset,
                        wave_y,
                        1.0,
                        Color::new(1.0, 1.0, 1.0, 0.3),
                    );
                }
            }
            TerrainType::Grass => {
                // 添加草地质感
                let time = macroquad::prelude::get_time() as f32;
                for i in 0..5 {
                    for j in 0..3 {
                        let grass_x = self.position.x + (i + 1) as f32 * self.width / 6.0;
                        let grass_y = self.position.y + (j + 1) as f32 * self.height / 4.0;
                        let sway = (time * 3.0 + i as f32 + j as f32).sin() * 2.0;
                        draw_line(
                            grass_x,
                            grass_y,
                            grass_x + sway,
                            grass_y - 5.0,
                            1.0,
                            DARKGREEN,
                        );
                    }
                }
            }
            TerrainType::Ice => {
                // 添加冰面反光效果
                let time = macroquad::prelude::get_time() as f32;
                if (time * 2.0).sin() > 0.5 {
                    draw_rectangle(
                        self.position.x + 5.0,
                        self.position.y + 5.0,
                        self.width - 10.0,
                        self.height - 10.0,
                        Color::new(1.0, 1.0, 1.0, 0.2),
                    );
                }
            }
            _ => {}
        }
    }
}
