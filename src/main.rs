mod bullet;
mod enemy;
mod player;
use libm::atan2;
use macroquad::{
    miniquad::window::quit, prelude::*, ui::{hash, root_ui, widgets, Skin}
};
use quad_snd::{
    decoder::read_wav_ext,
    mixer::{PlaybackStyle, SoundMixer},
    mixer::{Sound, Volume},
};

use bullet::Bullet;
use enemy::Enemy;
use player::{Player, PowerUpType, WeaponType};

pub enum GameState {
    Menu,
    Play,
    Options,
    Over
}

pub enum SoundType {
    MenuClick,
    PistolShot,
    EnemyHit,
    MenuMusic,
    PlayerDie
}

const MAX_ENEMIES: usize = 1000;

pub struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    enemy_texture: Texture2D,
    bullets: Vec<Bullet>,
    spawn_point: Vec<SpawnPoint>,
    last_spawn: f64,
    spawn_rate: f64,
    ui_assets: Vec<Texture2D>,
    ui_skin: Skin,
    score: i32,
    final_score: i32,
    power_up_timer: f32,
    can_get_powerup: bool
    // play_music: bool
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Grillageddon".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
fn create_ui_skin() -> Skin {
    let skin1: Skin = {
        let label_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../assets/ui/label.png"), None)
                    .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .font(include_bytes!("../assets/ui/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(0, 0, 0, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../assets/ui/background.png"), None)
                    .unwrap(),
            )
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../assets/ui/b_background.png"), None)
                    .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 15.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(include_bytes!("../assets/ui/b_hover.png"), None)
                    .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(include_bytes!("../assets/ui/b_pressed.png"), None)
                    .unwrap(),
            )
            .font(include_bytes!("../assets/ui/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(0, 0, 0, 255))
            .font_size(40)
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };
    return skin1;
}


pub fn sound_load(bytes: &[u8], style: PlaybackStyle) -> Sound {
    let sound = read_wav_ext(bytes, style).unwrap();
    return sound;
}

fn sound_play(sound: SoundType, volume: Volume, mixer: &mut SoundMixer) {
    match sound {
        SoundType::MenuClick => {
            let sound = sound_load(include_bytes!("../assets/sounds/button_click.wav"), PlaybackStyle::Once);
            mixer.play_ext(sound, volume);
        },
        SoundType::PistolShot => {
            let sound = sound_load(include_bytes!("../assets/sounds/gun_shoot.wav"), PlaybackStyle::Once);
            mixer.play_ext(sound, volume);
        },
        SoundType::EnemyHit => {
            let sound = sound_load(include_bytes!("../assets/sounds/enemy_hit.wav"), PlaybackStyle::Once);
            mixer.play_ext(sound, volume);
        },
        SoundType::MenuMusic => {
            let sound = sound_load(include_bytes!("../assets/sounds/menu_music.wav"), PlaybackStyle::Once);
            mixer.play_ext(sound, volume);
        },
        SoundType::PlayerDie => {
            let sound = sound_load(include_bytes!("../assets/sounds/player_die.wav"), PlaybackStyle::Once);
            mixer.play_ext(sound, volume);
        }
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut game = init_game().await;
    let mut mixer = SoundMixer::new();

    loop {
        clear_background(WHITE);
        match game.state {
            GameState::Play => {
                update(&mut game, &mut mixer).await;
                draw(&mut game);
            }
            GameState::Menu => menu(&mut game, &mut mixer).await,
            GameState::Options => menu(&mut game, &mut mixer).await,
            GameState::Over => menu(&mut game, &mut mixer).await
        }
        next_frame().await;
    }
}

async fn init_game() -> Game {
    let ui_skin = create_ui_skin();
    let player_texture = load_texture("assets/player.png").await.unwrap();
    let player = Player::new(
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        3.0,
        player_texture,
    );

    let enemy_texture = load_texture("assets/enemy.png").await.unwrap();
    let enemies: Vec<Enemy> = Vec::new();

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

    let pistol_textrue = load_texture("assets/pistol.png").await.unwrap();
    let shotgun_texture = load_texture("assets/shotgun.png").await.unwrap();
    let machinegun_texture = load_texture("assets/machine_gun.png").await.unwrap();

    let mut assets = Vec::new();
    assets.push(pistol_textrue);
    assets.push(shotgun_texture);
    assets.push(machinegun_texture);

    Game {
        state: GameState::Menu,
        player: player,
        enemies: enemies,
        enemy_texture: enemy_texture,
        bullets: bullets,
        spawn_point: spawn_points,
        last_spawn: get_time(),
        spawn_rate: 0.5,
        ui_assets: assets,
        ui_skin: ui_skin,
        score: 0,
        final_score: 0,
        power_up_timer: 0.0,
        can_get_powerup: true,
        // play_music: true
    }

}

fn spawn_enemies(game: &mut Game) {
    let spawn_timer = get_time();
    if spawn_timer - game.last_spawn > game.spawn_rate && game.enemies.len() < MAX_ENEMIES {
        let spawn_point = &game.spawn_point[rand::gen_range(0, game.spawn_point.len())];
        let enemy_pos = spawn_point.pos;
        game.enemies
            .push(Enemy::new(enemy_pos, &game.enemy_texture, 10));
        game.last_spawn = spawn_timer;
    }
}

fn reset_game(game: &mut Game){
    game.score = 0;
    game.bullets.clear();
    game.enemies.clear();
    game.player.is_dead = false;
    game.player.health = 500;
    game.player.position = Vec2::new(screen_width()/2.0, screen_height()/2.0);
}

async fn update(game: &mut Game, mixer: &mut SoundMixer) {
    if is_key_pressed(KeyCode::Escape) {
        game.state = GameState::Menu;
    }

    if game.player.is_dead {
        reset_game(game);
        game.state = GameState::Over
    }

    if !game.player.is_dead{
        spawn_enemies(game);
        player_update(game, mixer);
        bullet_update(game, mixer).await;
        enemy_update(game);
        collision_check(game, mixer);
    }
}

async fn bullet_update(game: &mut Game, mixer: &mut SoundMixer) {

    match game.player.weapon_type {
        WeaponType::Pistol => {
            if is_mouse_button_pressed(MouseButton::Left) {
                sound_play(SoundType::PistolShot,Volume(0.3), mixer);
                let mouse_pos = mouse_position();
                let mouse_target = Vec2::new(mouse_pos.0, mouse_pos.1);
                game.bullets.push(
                    Bullet::new(
                        Vec2::new(game.player.position.x, game.player.position.y + 16.),
                        mouse_target,
                        true,
                        5.0,
                    )
                    .await,
                )
            }
        }
        WeaponType::Machine => {
            let current_time = get_time();
            if is_mouse_button_down(MouseButton::Left)
                && current_time - game.player.last_shot > game.player.fire_rate
            {
                sound_play(SoundType::PistolShot,Volume(0.3), mixer);
                let mouse_pos = mouse_position();
                let mouse_target = Vec2::new(mouse_pos.0, mouse_pos.1);
                game.bullets.push(
                    Bullet::new(
                        Vec2::new(game.player.position.x, game.player.position.y + 16.),
                        mouse_target,
                        true,
                        7.0,
                    )
                    .await,
                );
                game.player.last_shot = current_time;
            }
        }
        WeaponType::Shotgun => {
            let current_time = get_time();
            if is_mouse_button_down(MouseButton::Left)
                && current_time - game.player.last_shot > game.player.shotgun_fire_rate
            {
                sound_play(SoundType::PistolShot,Volume(0.3), mixer);
                let player_pos = Vec2::new(game.player.position.x, game.player.position.y + 16.);
                let mouse_pos = mouse_position();
                let mouse_target = Vec2::new(mouse_pos.0, mouse_pos.1);
                let spread_angle: f64 = 20.0;

                let mouse_direction = (mouse_target - player_pos).normalize(); // Calculate direction to mouse
                let base_angle = mouse_direction.y.atan2(mouse_direction.x); // Calculate base angle

                let spread_increment = spread_angle.to_radians() / (3 - 1) as f64;

                for i in 0..3 {
                    let angle = base_angle
                        + (-spread_angle.to_radians() as f32 / 2.0
                            + spread_increment as f32 * i as f32);
                    let bullet_direction = Vec2::new(angle.cos() as f32, angle.sin() as f32);
                    let bullet_target = player_pos + bullet_direction * 100.0;
                    game.bullets
                        .push(Bullet::new(player_pos, bullet_target, true, 7.0).await);
                }

                game.player.last_shot = current_time
            }
        }
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

fn collision_check(game: &mut Game, mixer: &mut SoundMixer) {
    for enemy in game.enemies.iter_mut() {
        for bullet in game.bullets.iter_mut() {
            if enemy.coll_rect.overlaps(&bullet.coll_rect) {
                bullet.is_active = false;
                let dmg: i32;
                match game.player.weapon_type {
                    WeaponType::Pistol =>       dmg = game.player.damage,
                    WeaponType::Machine =>      dmg = game.player.damage - 2,
                    WeaponType::Shotgun =>      dmg = game.player.damage
                }
                damage_enemy(enemy, dmg);
                sound_play(SoundType::EnemyHit, Volume(0.2), mixer);
            }
        }
    }

    for enemy in game.enemies.iter_mut() {
        let current_time = get_time();

        if enemy.coll_rect.overlaps(&game.player.coll_rect) && enemy.can_attack{
            game.player.health -= 10;
            enemy.can_attack = false;
            enemy.dmg_cd = current_time;
            sound_play(SoundType::EnemyHit, Volume(0.2), mixer);
        }
        // reset can attack to true after a few seconds
        if !enemy.can_attack && (current_time - enemy.dmg_cd as f64) > 0.5 {
            enemy.can_attack = true
        }
    }
}

fn damage_enemy(enemy: &mut Enemy, dmg: i32) {
    enemy.health -= dmg;
}

fn player_update(game: &mut Game, mixer: &mut SoundMixer) {
    if is_key_pressed(KeyCode::Key1) {
        game.player.weapon_type = WeaponType::Pistol
    }

    if is_key_pressed(KeyCode::Key2) {
        game.player.weapon_type = WeaponType::Machine
    }

    if is_key_pressed(KeyCode::Key3) {
        game.player.weapon_type = WeaponType::Shotgun
    }

    if is_key_down(KeyCode::A) {
        game.player.velocity.x -= game.player.acceleration;
    }

    if is_key_down(KeyCode::D) {
        game.player.velocity.x += game.player.acceleration;
    }

    if is_key_down(KeyCode::W) {
        game.player.velocity.y -= game.player.acceleration;
    }

    if is_key_down(KeyCode::S) {
        game.player.velocity.y += game.player.acceleration;
    }

    game.player.velocity.y = clamp(game.player.velocity.y, -game.player.speed, game.player.speed);
    game.player.velocity.x = clamp(game.player.velocity.x, -game.player.speed, game.player.speed);

    if game.player.velocity.x > 0.0 {
        game.player.velocity.x -= game.player.friction;
    } else if game.player.velocity.x < 0.0 {
        game.player.velocity.x += game.player.friction; 
    }

    if game.player.velocity.y > 0.0 {
        game.player.velocity.y -= game.player.friction;
    } else if game.player.velocity.y < 0.0 {
        game.player.velocity.y += game.player.friction; 
    }

    if game.player.health <= 0 {
        game.final_score = game.score;
        game.player.is_dead = true;
        sound_play(SoundType::PlayerDie, Volume(0.3), mixer)
    }

    player_powerups(game);

    
    game.player.position += game.player.velocity;
    game.player.coll_rect.x = game.player.position.x;
    game.player.coll_rect.y = game.player.position.y;
    
    bounds_check(game);

}

fn bounds_check(game: &mut Game) {
    if game.player.position.x >= screen_width()  - game.player.texture.width() {
        game.player.position.x = screen_width() - game.player.texture.width()
    }
    if game.player.position.x <= 0.0 {
        game.player.position.x = 0.0
    }

    if game.player.position.y > screen_height()  - (game.player.texture.height() - 10.0) {
        game.player.position.y = screen_height() - (game.player.texture.height() - 10.0)
    }
    if game.player.position.y <= 0.0 {
        game.player.position.y = 0.0
    }
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
        if enemy.health <= 0 {
            game.score += 5;
        }
        enemy.position += normalized_direction * enemy.speed;
        enemy.coll_rect.x = enemy.position.x;
        enemy.coll_rect.y = enemy.position.y;
    }

    game.enemies.retain(|enemy| enemy.health > 0);
}

fn draw(game: &mut Game) {
    draw_text_ex(&game.score.to_string(), screen_width()/2.0, 50.0, TextParams{
        font_size: 50,
        color: BLACK,
        ..Default::default()
    });
    // Draw the three guns at the bottom
    draw_inventory(game);
    draw_hud(game);

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
    );
}

fn draw_hud(game: &mut Game) {
    let hp_text = "HP ".to_owned() + &game.player.health.to_string();
    draw_text_ex(&hp_text, 50.0, screen_height() - 50.0, TextParams{
        font_size: 30,
        color: BLACK,
        ..Default::default()
    });
}

fn draw_inventory(game: &mut Game) {
    let color: Color;
    match game.player.power_up {
        player::PowerUpType::None => color = WHITE,
        player::PowerUpType::FastAttack => color = GREEN,
        player::PowerUpType::QuadDamage => color = PURPLE,
    };
    match game.player.weapon_type {
        WeaponType::Pistol => draw_texture_ex(
            &game.ui_assets[0],
            screen_width() / 2.0,
            screen_height() - 50.0,
            color,
            DrawTextureParams {
                ..Default::default()
            },
        ),
        WeaponType::Machine => draw_texture_ex(
            &game.ui_assets[2],
            screen_width() / 2.0,
            screen_height() - 50.0,
            color,
            DrawTextureParams {
                ..Default::default()
            },
        ),
        WeaponType::Shotgun => draw_texture_ex(
            &game.ui_assets[1],
            screen_width() / 2.0,
            screen_height() - 50.0,
            color,
            DrawTextureParams {
                ..Default::default()
            },
        ),
    }
}


async fn menu(game: &mut Game, mixer: &mut SoundMixer) {
    match game.state {
        GameState::Menu => {
            // TODO: This is not a good way to do this.
            // if game.play_music {
            //     sound_play(SoundType::MenuMusic, Volume(0.2), mixer);
            //     game.play_music = false
            // }
            root_ui().push_skin(&game.ui_skin);
            root_ui().window(
                hash!(),
                vec2(0.0 - 5., 0.0),
                vec2(screen_width() + 5., screen_height() + 5.0),
                |ui| {
                    widgets::Label::new("Grillageddon")
                        .position(vec2(260.0, 10.0))
                        .ui(ui);
                    let play_button = widgets::Button::new("Play")
                        .position(vec2(300., 100.0))
                        .ui(ui);
                    let info_button = widgets::Button::new("Info")
                        .position(vec2(300.0, 200.0))
                        .ui(ui);

                    let quit_button = widgets::Button::new("Quit")
                        .position(vec2(300.0, 300.0))
                        .ui(ui);

                    if play_button {
                        sound_play(SoundType::MenuClick, Volume(0.5), mixer);
                        game.state = GameState::Play;
                    }

                    if info_button {
                        sound_play(SoundType::MenuClick, Volume(0.5), mixer);
                        game.state = GameState::Options;
                    }

                    if quit_button {
                        sound_play(SoundType::MenuClick, Volume(0.5), mixer);
                        quit()
                    }
                },
            );
            root_ui().pop_skin();
        }
        GameState::Play => {},
        GameState::Options => {
            root_ui().push_skin(&game.ui_skin);
            root_ui().window(
                hash!(),
                vec2(0.0 - 5., 0.0),
                vec2(screen_width() + 5., screen_height() + 5.0),
                |ui| {
                    widgets::Label::new("Grillageddon")
                        .position(vec2(260.0, 10.0))
                        .ui(ui);

                    widgets::Label::new(
                        "Controls: WASD cycle through weapons: 1, 2, 3",
                    )
                    .position(vec2(100.0, 100.0))
                    .ui(ui);

                    widgets::Label::new(
                        "You get a random powerup at the start of the game"
                    )
                    .position(vec2(100.0, 200.0))
                    .ui(ui);

                    widgets::Label::new(
                        "And a new one every 50 points, They are represented by color"
                    )
                    .position(vec2(-20.0, 230.0))
                    .ui(ui);

                    widgets::Label::new(
                        "Green: Fast fire, Purple: Quad Damage"
                    )
                    .position(vec2(100.0, 290.0))
                    .ui(ui);

                    let back_button = widgets::Button::new("Back")
                        .position(vec2(300., 400.0))
                        .ui(ui);

                    if back_button {
                        sound_play(SoundType::MenuClick, Volume(0.5), mixer);
                        game.state = GameState::Menu
                    }
                },
            );
            root_ui().pop_skin();
        }
        GameState::Over => {
            root_ui().push_skin(&game.ui_skin);
            root_ui().window(
                hash!(),
                vec2(0.0 - 5., 0.0),
                vec2(screen_width() + 5., screen_height() + 5.0),
                |ui| {
                    widgets::Label::new("Grillageddon")
                        .position(vec2(260.0, 10.0))
                        .ui(ui);

                    widgets::Label::new(
                        "You Died!! Score ".to_owned() + &game.final_score.to_string(),
                    )
                    .position(vec2(100.0, 150.0))
                    .ui(ui);

                    let back_button = widgets::Button::new("Back")
                        .position(vec2(300., 300.0))
                        .ui(ui);

                    if back_button {
                        game.state = GameState::Menu
                    }
                },
            );
            root_ui().pop_skin();
        },
    };
}

fn player_powerups(game: &mut Game) {
    // Look into switching powerups every 50 points
    let random: i32;
    if game.score % 50 == 0 && game.can_get_powerup{

        random = rand::gen_range(1, 3);
        
        if random == 1{
            game.player.has_power_up = true;
            game.player.power_up = PowerUpType::FastAttack;
        }
        
        if random == 2 {
            game.player.has_power_up = true;
            game.player.power_up = PowerUpType::QuadDamage;
        }

        game.can_get_powerup = false;
    }

    
    if game.player.has_power_up == true {
        game.power_up_timer += 0.1;
        
        if game.power_up_timer >= 50.0 {
            game.player.power_up = PowerUpType::None;
            game.player.has_power_up = false;
            game.power_up_timer = 0.0;
            game.can_get_powerup = true;
        }
    }
    

    match game.player.power_up {
        player::PowerUpType::None => {
            // Reset player back to normal
            game.player.fire_rate           = 0.1;
            game.player.shotgun_fire_rate   = 0.9;
            game.player.damage              = 5 
        },
        player::PowerUpType::FastAttack => {
            game.player.fire_rate           = 0.05;
            game.player.shotgun_fire_rate   = 0.05;

        },
        player::PowerUpType::QuadDamage => {
            game.player.damage = 20; 
        },
    }  
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
