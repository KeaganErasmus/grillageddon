use macroquad::prelude::*;

#[derive(Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub speed: f32,
    pub texture: Texture2D,
    pub coll_rect: Rect,
    pub health: i32,
    pub dmg_cd: f64,
    pub can_attack: bool,
}

impl Enemy {
    pub fn new(position: Vec2, texture: &Texture2D, health: i32) -> Enemy {
        Enemy {
            position: position,
            speed: 1.0,
            texture: texture.clone(),
            coll_rect: Rect::new(position.x, position.y, texture.width(), texture.height()),
            health: health,
            dmg_cd: 1.0,
            can_attack: true,
        }
    }
}
