use macroquad::prelude::*;

pub enum WeaponType {
    Pistol,
    Machine,
    Shotgun,
}

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: f32,
    pub friction: f32,
    pub health: i32,
    pub speed: f32,
    pub texture: Texture2D,
    pub coll_rect: Rect,
    pub fram_index: i32,
    pub frame_time: f32,
    pub weapon_type: WeaponType,
    pub last_shot: f64,
    pub fire_rate: f64,
    pub shotgun_fire_rate: f64,
    pub is_dead: bool
}

impl Player {
    pub fn new(position: Vec2, speed: f32, texture: Texture2D) -> Player {
        Player {
            position: position,
            velocity: Vec2::new(0.0, 0.0),
            acceleration: 1.0,
            friction: 0.5,
            health: 500,
            speed: speed,
            texture: texture.clone(),
            coll_rect: Rect::new(position.x, position.y, 32.0, 32.0),
            fram_index: 0,
            frame_time: 0.0,
            weapon_type: WeaponType::Pistol,
            last_shot: get_time(),
            fire_rate: 0.1,
            shotgun_fire_rate: 0.9,
            is_dead: false
        }
    }
}
