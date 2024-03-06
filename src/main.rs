mod player;
mod enemy;

use enemy::Enemy;
use macroquad::{prelude::*, rand};
use player::Player;


const MAX_ENEMIES: i32 = 2;

pub enum GameState {
    Pause,
    Play,
    Over,
    Start,
}
pub struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
}

#[macroquad::main("Grillageddon")]
async fn main() {
    let mut game = init_game().await;
    loop {
        clear_background(WHITE);
        match game.state {
            GameState::Play => {
                update(&mut game);
                draw(&mut game);
            }
            GameState::Pause => menu(&mut game),
            GameState::Over => todo!(),
            GameState::Start => todo!(),
        }
        next_frame().await;
    }
}

async fn init_game() -> Game {
    let player_texture = load_texture("assets/player.png").await.unwrap();
    let enemy_texture = load_texture("assets/player.png").await.unwrap();
    let player = Player::new(Vec2::new(100.0, 100.0), 3.0, &player_texture);
    let mut enemies: Vec<Enemy> = Vec::new();

    for _ in 0..MAX_ENEMIES {
        enemies.push(Enemy::new(
            Vec2::new(
                rand::gen_range(0, screen_width() as i32) as f32,
                rand::gen_range(0, screen_height() as i32) as f32,
            ),
            &enemy_texture,
        ));
    }

    Game {
        state: GameState::Play,
        player: player,
        enemies: enemies,
    }
}

fn update(game: &mut Game) {

    if is_key_pressed(KeyCode::Escape) {
        game.state = GameState::Pause;
    }

    if is_mouse_button_down(MouseButton::Left) {
        
    }
    
    player_update(game);
    enemy_update(game);
}

fn player_update(game: &mut Game){
    if is_key_down(KeyCode::A) {
        game.player.position.x -= game.player.speed;
    }

    if is_key_down(KeyCode::D) {
        game.player.position.x += game.player.speed;
    }

    if is_key_down(KeyCode::W) {
        game.player.position.y -= game.player.speed;
    }

    if is_key_down(KeyCode::S) {
        game.player.position.y += game.player.speed;
    }

    game.player.coll_rect.x = game.player.position.x;
    game.player.coll_rect.y = game.player.position.y;
}

fn enemy_update(game: &mut Game){
    let player_pos: Vec2 = game.player.position;

    // Clone the enemies vector to iterate over
    let enemies_clone = game.enemies.clone();

    for enemy in game.enemies.iter_mut() {
        // Calculate the direction towards the player
        let direction = player_pos - enemy.position;
        let distance = direction.length();

        // Normalize the direction
        let mut normalized_direction = direction;
        if distance != 0.0 {
            normalized_direction /= distance;
        }

        // Check for collisions with other enemies and adjust position
        for other_enemy in enemies_clone.iter() {
            if enemy.coll_rect.overlaps(&other_enemy.coll_rect) {
                let avoidance_direction = enemy.position - other_enemy.position;
                let avoidance_distance = avoidance_direction.length();

                if avoidance_distance != 0.0 {
                    // Adjust the normalized direction based on avoidance direction
                    normalized_direction += avoidance_direction.normalize() / avoidance_distance;
                }
            }
        }

        enemy.position = enemy.position + normalized_direction * enemy.speed;
        enemy.coll_rect.x = enemy.position.x;
        enemy.coll_rect.y = enemy.position.y;
    
    }
}

fn draw(game: &mut Game) {
    for enemy in game.enemies.iter_mut() {
        draw_texture(&enemy.texture, enemy.position.x, enemy.position.y, RED);
    }
    draw_texture(
        &game.player.texture,
        game.player.position.x,
        game.player.position.y,
        WHITE,
    );
}

fn menu(game: &mut Game) {
    match game.state {
        GameState::Pause => {
            clear_background(WHITE);
            draw_text("Game is paused", 100.0, 100.0, 100.0, BLACK);
            if is_key_pressed(KeyCode::Escape) {
                game.state = GameState::Play;
            }
        }
        GameState::Play => todo!(),
        GameState::Over => todo!(),
        GameState::Start => todo!(),
    };
}
