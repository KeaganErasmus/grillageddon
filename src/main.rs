mod bullet;
mod enemy;
mod player;

use libm::atan2;
use macroquad::prelude::*;

use bullet::Bullet;
use enemy::Enemy;
use player::{Direction, Player};

const MAX_ENEMIES: i32 = 10;
const NUM_PLAYER_FRAMES: i32 = 8;

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
    bullets: Vec<Bullet>,
    spawn_point: Vec<SpawnPoint>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Grillageddon".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = init_game().await;
    loop {
        clear_background(WHITE);
        match game.state {
            GameState::Play => {
                update(&mut game).await;
                draw(&mut game);
            }
            GameState::Pause => menu(&mut game).await,
            GameState::Over => todo!(),
            GameState::Start => todo!(),
        }
        next_frame().await;
    }
}

async fn init_game() -> Game {
    let player_texture = load_texture("assets/player.png").await.unwrap();
    // player_texture.set_filter(FilterMode::Nearest);
    let player = Player::new(Vec2::new(100.0, 100.0), 3.0, player_texture);

    let enemy_texture = load_texture("assets/enemy.png").await.unwrap();
    let mut enemies: Vec<Enemy> = Vec::new();

    let bullets: Vec<Bullet> = Vec::new();

    let spawn_point_texture = load_texture("assets/spawn_point.png").await.unwrap();
    let mut spawn_points: Vec<SpawnPoint> = Vec::new();
    spawn_points.push(SpawnPoint {
        pos: Vec2::new(10.0, 10.0),
        texture: spawn_point_texture.clone(),
    });
    spawn_points.push(SpawnPoint {
        pos: Vec2::new(750.0, 10.0),
        texture: spawn_point_texture.clone(),
    });
    spawn_points.push(SpawnPoint {
        pos: Vec2::new(10.0, 550.0),
        texture: spawn_point_texture.clone(),
    });
    spawn_points.push(SpawnPoint {
        pos: Vec2::new(750.0, 550.0),
        texture: spawn_point_texture.clone(),
    });
    // Vec2::new(
    //     rand::gen_range(0, screen_width() as i32) as f32,
    //     rand::gen_range(0, screen_height() as i32) as f32,
    // ),

    for _ in 0..MAX_ENEMIES {
        // let spawn_point = &spawn_points[rand::gen_range(0, spawn_points.len())];
        // let enemy_pos = spawn_point.pos;
        enemies.push(Enemy::new(
            // enemy_pos,
            Vec2::new(
                rand::gen_range(0, screen_width() as i32) as f32,
                rand::gen_range(0, screen_height() as i32) as f32,
            ),
            &enemy_texture,
            10,
        ));
    }

    Game {
        state: GameState::Play,
        player: player,
        enemies: enemies,
        bullets: bullets,
        spawn_point: spawn_points,
    }
}

fn spawn_enemies(game: &mut Game) {}

async fn update(game: &mut Game) {
    if is_key_pressed(KeyCode::Escape) {
        game.state = GameState::Pause;
    }

    spawn_enemies(game);
    player_update(game);
    bullet_update(game).await;
    enemy_update(game);
    collision_check(game);
}

async fn bullet_update(game: &mut Game) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        let mouse_target = Vec2::new(mouse_pos.0, mouse_pos.1);
        game.bullets.push(
            Bullet::new(
                Vec2::new(game.player.position.x, game.player.position.y + 16.),
                mouse_target,
                true,
                3.0,
            )
            .await,
        )
    }

    for bullet in game.bullets.iter_mut() {
        bullet.position += bullet.velocity * bullet.speed;

        if bullet.position.x > screen_width() || bullet.position.x < 0.0 {
            bullet.is_active = false;
        }

        if bullet.position.y > screen_height() || bullet.position.y < 0.0 {
            bullet.is_active = false;
        }

        bullet.coll_rect.x = bullet.position.x;
        bullet.coll_rect.y = bullet.position.y;
    }

    game.bullets.retain(|bullet| bullet.is_active);
}

fn collision_check(game: &mut Game) {
    for enemy in game.enemies.iter_mut() {
        for bullet in game.bullets.iter_mut() {
            if enemy.coll_rect.overlaps(&bullet.coll_rect) {
                bullet.is_active = false;
                damage_enemy(enemy);
            }
        }
    }
}

fn damage_enemy(enemy: &mut Enemy) {
    enemy.health -= 5;
}

fn animate_player(game: &mut Game) {
    game.player.frame_time += get_frame_time();
    if game.player.frame_time >= 0.1 {
        game.player.fram_index = (game.player.fram_index + 1) % NUM_PLAYER_FRAMES;
        game.player.frame_time = 0.0;
    }
}

fn player_update(game: &mut Game) {
    let mut movement = Vec2::default();

    if is_key_down(KeyCode::A) {
        movement.x -= 1.0;
        game.player.dir = Direction::Left;
        animate_player(game);
    }

    if is_key_down(KeyCode::D) {
        movement.x += 1.0;
        game.player.dir = Direction::Right;
        animate_player(game);
    }

    if is_key_down(KeyCode::W) {
        movement.y -= 1.0;
        animate_player(game);
    }

    if is_key_down(KeyCode::S) {
        movement.y += 1.0;
        animate_player(game);
    }

    if movement.length() > 1.0 {
        movement = movement.normalize();
    }

    game.player.position += movement * game.player.speed;
    game.player.coll_rect.x = game.player.position.x;
    game.player.coll_rect.y = game.player.position.y;
}

fn enemy_update(game: &mut Game) {
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

        enemy.position    += normalized_direction * enemy.speed;
        enemy.coll_rect.x =  enemy.position.x;
        enemy.coll_rect.y =  enemy.position.y;
    }

    game.enemies.retain(|enemy| enemy.health > 0);
}

fn draw(game: &mut Game) {
    for point in game.spawn_point.iter() {
        draw_texture(&point.texture, point.pos.x, point.pos.y, WHITE);
    }

    for bullet in game.bullets.iter_mut() {
        draw_rectangle_lines(
            bullet.position.x,
            bullet.position.y,
            bullet.coll_rect.w,
            bullet.coll_rect.h,
            2.,
            RED,
        );
        draw_texture(&bullet.texture, bullet.position.x, bullet.position.y, BLACK);
    }

    for enemy in game.enemies.iter_mut() {
        draw_rectangle_lines(
            enemy.position.x,
            enemy.position.y,
            enemy.coll_rect.w,
            enemy.coll_rect.h,
            2.,
            RED,
        );

        let direction = game.player.position - enemy.position;
        let angle_to_player = atan2(direction.y as f64, direction.x as f64);
        let rotation = angle_to_player;
        draw_texture_ex(
            &enemy.texture,
            enemy.position.x,
            enemy.position.y,
            GREEN,
            DrawTextureParams {
                rotation: rotation as f32,
                ..Default::default()
            },
        )
    }

    let mouse_pos = mouse_position();
    let mouse_target = Vec2::new(mouse_pos.0, mouse_pos.1);
    let direction = game.player.position - mouse_target;
    let angle_to_mouse = atan2(direction.y as f64, direction.x as f64);
    let rotation = angle_to_mouse;
    draw_texture_ex(
        &game.player.texture,
        game.player.position.x,
        game.player.position.y,
        WHITE,
        DrawTextureParams {
            rotation: rotation as f32,
            ..Default::default()
        },
    )
}

async fn menu(game: &mut Game) {
    match game.state {
        GameState::Pause => {
            let play_item = MenuItem::new("Play".to_owned(), Vec2::new(300., 200.), 100, BLUE);
            clear_background(WHITE);
            draw_text_ex(
                "Game is Paused",
                100.0,
                100.0,
                TextParams {
                    font_size: 100,
                    color: BLACK,
                    ..Default::default()
                },
            );
            // Draw the play button
            draw_text_ex(
                &play_item.text,
                play_item.pos.x,
                play_item.pos.y,
                TextParams {
                    font_size: play_item.font_size,
                    color: play_item.font_colour,
                    ..Default::default()
                },
            );

            if is_key_pressed(KeyCode::Escape) {
                game.state = GameState::Play;
            }
        }
        GameState::Play => todo!(),
        GameState::Over => todo!(),
        GameState::Start => todo!(),
    };
}

pub struct SpawnPoint {
    pos: Vec2,
    texture: Texture2D,
}
impl SpawnPoint {
    pub fn new(pos: Vec2, texture: Texture2D) -> SpawnPoint {
        SpawnPoint {
            pos: pos,
            texture: texture.clone(),
        }
    }
}

pub struct MenuItem {
    text: String,
    pos: Vec2,
    font_size: u16,
    font_colour: Color,
    rect: Rect,
}
impl MenuItem {
    pub fn new(text: String, pos: Vec2, font_size: u16, font_colour: Color) -> MenuItem {
        MenuItem {
            text: text,
            pos: pos,
            font_size: font_size,
            font_colour: font_colour,
            rect: Rect::new(pos.x, pos.y, font_size as f32, font_size as f32),
        }
    }
}
