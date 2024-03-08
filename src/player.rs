use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub texture: Texture2D,
    pub coll_rect: Rect,
}

impl Player {
    pub fn new(position: Vec2, speed: f32, texture: &Texture2D) -> Player {
        Player {
            position: position,
            speed: speed,
            texture: texture.clone(),
            coll_rect: Rect::new(position.x, position.y, texture.width(), texture.height()),
        }
    }
}
