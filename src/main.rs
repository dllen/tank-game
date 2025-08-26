use macroquad::prelude::*;

mod game;
mod entities;
mod systems;
mod ui;

use game::Game;

#[macroquad::main("Tank Battle")]
async fn main() {
    let mut game = Game::new();
    
    loop {
        game.update().await;
        game.draw().await;
        next_frame().await;
    }
}