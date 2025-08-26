use macroquad::prelude::*;
use crate::math_challenge::MathChallenge;

pub struct GameUI {
    pub font_size: f32,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            font_size: 20.0,
        }
    }
    
    pub fn draw_hud(&self, player_health: i32, max_health: i32, score: i32, wave: i32, difficulty: f32) {
        let margin = 10.0;
        
        // Health Bar
        let health_bar_width = 200.0;
        let health_bar_height = 20.0;
        let health_ratio = player_health as f32 / max_health as f32;
        
        // Health Bar Background
        draw_rectangle(margin, margin, health_bar_width, health_bar_height, DARKGRAY);
        
        // Health Bar
        let health_color = if health_ratio > 0.6 {
            GREEN
        } else if health_ratio > 0.3 {
            YELLOW
        } else {
            RED
        };
        
        draw_rectangle(
            margin,
            margin,
            health_bar_width * health_ratio,
            health_bar_height,
            health_color,
        );
        
        // Health Bar Border
        draw_rectangle_lines(margin, margin, health_bar_width, health_bar_height, 2.0, WHITE);
        
        // Health Text
        let health_text = format!("Health: {}/{}", player_health, max_health);
        draw_text(&health_text, margin + 5.0, margin + 15.0, 16.0, WHITE);
        
        // Score
        let score_text = format!("Score: {}", score);
        draw_text(&score_text, margin, margin + 50.0, self.font_size, WHITE);
        
        // Wave
        let wave_text = format!("Wave: {}", wave);
        draw_text(&wave_text, margin, margin + 80.0, self.font_size, WHITE);
        
        // Difficulty
        let difficulty_text = format!("Difficulty: {:.1}", difficulty);
        draw_text(&difficulty_text, margin, margin + 110.0, self.font_size, WHITE);
        
        // Controls
        let controls = [
            "WASD/Arrow Keys: Move",
            "Mouse: Aim",
            "Left Click/Space: Shoot",
            "ESC: Pause",
        ];
        
        let start_y = screen_height() - 100.0;
        for (i, control) in controls.iter().enumerate() {
            draw_text(control, margin, start_y + i as f32 * 20.0, 16.0, LIGHTGRAY);
        }
    }
    
    pub fn draw_game_over(&self, score: i32, wave: i32, high_score: i32) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // Semi-transparent background
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));
        
        // Game Over Title
        let title = "GAME OVER";
        let title_size = 48.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_w / 2.0 - title_dims.width / 2.0,
            screen_h / 2.0 - 100.0,
            title_size,
            RED,
        );
        
        // Score Information
        let score_text = format!("Final Score: {}", score);
        let score_dims = measure_text(&score_text, None, self.font_size as u16, 1.0);
        draw_text(
            &score_text,
            screen_w / 2.0 - score_dims.width / 2.0,
            screen_h / 2.0 - 40.0,
            self.font_size,
            WHITE,
        );
        
        let wave_text = format!("Waves Reached: {}", wave);
        let wave_dims = measure_text(&wave_text, None, self.font_size as u16, 1.0);
        draw_text(
            &wave_text,
            screen_w / 2.0 - wave_dims.width / 2.0,
            screen_h / 2.0 - 10.0,
            self.font_size,
            WHITE,
        );
        
        let high_score_text = format!("High Score: {}", high_score);
        let high_score_dims = measure_text(&high_score_text, None, self.font_size as u16, 1.0);
        draw_text(
            &high_score_text,
            screen_w / 2.0 - high_score_dims.width / 2.0,
            screen_h / 2.0 + 20.0,
            self.font_size,
            GOLD,
        );
        
        // Restart Prompt
        let restart_text = "Press R to Restart";
        let restart_dims = measure_text(restart_text, None, self.font_size as u16, 1.0);
        draw_text(
            restart_text,
            screen_w / 2.0 - restart_dims.width / 2.0,
            screen_h / 2.0 + 80.0,
            self.font_size,
            YELLOW,
        );
    }
    
    pub fn draw_pause_menu(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // Semi-transparent background
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.5));
        
        // Pause Title
        let title = "PAUSED";
        let title_size = 36.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_w / 2.0 - title_dims.width / 2.0,
            screen_h / 2.0 - 50.0,
            title_size,
            WHITE,
        );
        
        // Continue Prompt
        let continue_text = "Press ESC to Continue";
        let continue_dims = measure_text(continue_text, None, self.font_size as u16, 1.0);
        draw_text(
            continue_text,
            screen_w / 2.0 - continue_dims.width / 2.0,
            screen_h / 2.0 + 20.0,
            self.font_size,
            YELLOW,
        );
    }
    
    pub fn draw_start_menu(&self, high_score: i32) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // Background
        clear_background(BLACK);
        
        // Game Title
        let title = "TANK BATTLE";
        let title_size = 64.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_w / 2.0 - title_dims.width / 2.0,
            screen_h / 2.0 - 150.0,
            title_size,
            GOLD,
        );
        
        // Difficulty Selection
        let difficulty_text = "Select Difficulty:";
        let diff_dims = measure_text(difficulty_text, None, self.font_size as u16, 1.0);
        draw_text(
            difficulty_text,
            screen_w / 2.0 - diff_dims.width / 2.0,
            screen_h / 2.0 - 50.0,
            self.font_size,
            WHITE,
        );
        
        let difficulties = ["1 - Easy", "2 - Normal", "3 - Hard"];
        for (i, diff) in difficulties.iter().enumerate() {
            let diff_dims = measure_text(diff, None, self.font_size as u16, 1.0);
            draw_text(
                diff,
                screen_w / 2.0 - diff_dims.width / 2.0,
                screen_h / 2.0 - 10.0 + i as f32 * 30.0,
                self.font_size,
                LIGHTGRAY,
            );
        }
        
        // High Score
        let high_score_text = format!("High Score: {}", high_score);
        let high_dims = measure_text(&high_score_text, None, self.font_size as u16, 1.0);
        draw_text(
            &high_score_text,
            screen_w / 2.0 - high_dims.width / 2.0,
            screen_h / 2.0 + 120.0,
            self.font_size,
            GOLD,
        );
    }
    
    pub fn draw_math_challenge(&self, challenge: &MathChallenge) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // Semi-transparent background
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        
        // Title
        let title = "MATH CHALLENGE - REVIVE!";
        let title_size = 36.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_w / 2.0 - title_dims.width / 2.0,
            screen_h / 2.0 - 120.0,
            title_size,
            GOLD,
        );
        
        // Instructions
        let instruction = "Solve the math problem to revive:";
        let inst_dims = measure_text(instruction, None, self.font_size as u16, 1.0);
        draw_text(
            instruction,
            screen_w / 2.0 - inst_dims.width / 2.0,
            screen_h / 2.0 - 60.0,
            self.font_size,
            WHITE,
        );
        
        // Math Problem
        let problem = challenge.get_question_text();
        let problem_size = 32.0;
        let problem_dims = measure_text(&problem, None, problem_size as u16, 1.0);
        draw_text(
            &problem,
            screen_w / 2.0 - problem_dims.width / 2.0,
            screen_h / 2.0 - 10.0,
            problem_size,
            YELLOW,
        );
        
        // Input Box
        let input_box_width = 200.0;
        let input_box_height = 40.0;
        let input_box_x = screen_w / 2.0 - input_box_width / 2.0;
        let input_box_y = screen_h / 2.0 + 30.0;
        
        draw_rectangle(input_box_x, input_box_y, input_box_width, input_box_height, DARKGRAY);
        draw_rectangle_lines(input_box_x, input_box_y, input_box_width, input_box_height, 2.0, WHITE);
        
        // User Input
        let input_text = challenge.get_user_answer();
        let input_dims = measure_text(&input_text, None, 24, 1.0);
        draw_text(
            &input_text,
            input_box_x + 10.0,
            input_box_y + 28.0,
            24.0,
            WHITE,
        );
        
        // Cursor
        let cursor_x = input_box_x + 10.0 + input_dims.width;
        draw_line(cursor_x, input_box_y + 8.0, cursor_x, input_box_y + 32.0, 2.0, WHITE);
        
        // Instructions
        let submit_text = "Press ENTER to submit, ESC to give up";
        let submit_dims = measure_text(submit_text, None, 16, 1.0);
        draw_text(
            submit_text,
            screen_w / 2.0 - submit_dims.width / 2.0,
            screen_h / 2.0 + 100.0,
            16.0,
            LIGHTGRAY,
        );
        
        // Show result if completed
        if challenge.is_completed {
            let result_text = if challenge.is_correct {
                "Correct! You are revived!"
            } else {
                "Wrong answer. Game Over!"
            };
            let result_color = if challenge.is_correct { GREEN } else { RED };
            let result_dims = measure_text(result_text, None, self.font_size as u16, 1.0);
            draw_text(
                result_text,
                screen_w / 2.0 - result_dims.width / 2.0,
                screen_h / 2.0 + 130.0,
                self.font_size,
                result_color,
            );
        }
    }
}