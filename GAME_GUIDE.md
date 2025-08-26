# Tank Battle Game Guide

## How to Play

### Starting the Game
1. Run `cargo run --release` to start the game
2. On the main menu, press 1, 2, or 3 to select difficulty:
   - **1 - Easy**: Fewer enemies, slower pace
   - **2 - Normal**: Balanced gameplay
   - **3 - Hard**: More enemies, faster action

### Controls
- **Movement**: WASD keys or Arrow keys
- **Aiming**: Move your mouse to aim the tank cannon
- **Shooting**: Left mouse button or Spacebar
- **Pause**: ESC key
- **Restart**: R key (when game over)

### Game Elements

#### Your Tank (Blue)
- Health: 100 HP
- Can collect power-ups
- Destroyed when health reaches 0

#### Enemy Tanks (Red)
- Health: 50 HP each
- AI-controlled with different behaviors
- Award 100 points when destroyed

#### Obstacles
- **Brown Walls**: Destructible, can be damaged by bullets
- **Gray Steel**: Indestructible, bullets bounce off

#### Power-ups (Appear randomly)
- **🟢 Health** (+): Restores 50 health points
- **🟡 Shield** (S): 10 seconds of invincibility
- **🟣 Scatter Shot** (*): Fires 5 bullets in a spread pattern
- **🔵 Speed Boost** (>): Increases movement speed by 50%
- **🔴 Damage** (!): Increases attack power

### Scoring System
- Destroy enemy tank: +100 points
- Collect health power-up: +20 points
- Collect shield power-up: +30 points
- Collect other power-ups: +25-40 points
- Complete wave bonus: Wave number × 50 points

### Wave System
- Each wave spawns more enemies
- Difficulty increases over time
- Completing a wave restores 25 health
- New wave starts when all enemies are defeated

### Game Features
- **Dynamic Difficulty**: Game gets harder as you progress
- **Smart AI**: Enemies use different tactics (approach, retreat, flank)
- **Collision System**: Realistic physics for bullets and tanks
- **Power-up Effects**: Temporary abilities that change gameplay
- **High Score Tracking**: Beat your personal best

### Tips for Success
1. **Use Cover**: Hide behind destructible walls to avoid enemy fire
2. **Collect Power-ups**: They provide significant advantages
3. **Keep Moving**: Stationary tanks are easy targets
4. **Manage Health**: Don't ignore health power-ups
5. **Use Scatter Shot**: Great for taking out multiple enemies
6. **Shield Timing**: Use shields when surrounded by enemies

### Technical Details
- Built with Rust and Macroquad
- 60 FPS gameplay
- Real-time collision detection
- Procedural obstacle generation
- Adaptive AI difficulty scaling

Enjoy the battle! 🎮