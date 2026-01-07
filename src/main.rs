use macroquad::prelude::*;

mod game;
mod entities;
mod systems;
mod ui;
mod math_challenge;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tank Battle".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    
    loop {
        game.update().await;
        game.draw().await;
        next_frame().await;
    }
}