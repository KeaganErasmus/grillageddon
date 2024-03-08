use macroquad::prelude::*;

#[derive(Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub speed: f32,
    pub texture: Texture2D,
    pub coll_rect: Rect,
}

impl Enemy {
    pub fn new(position: Vec2, texture: &Texture2D) -> Enemy {
        Enemy {
            position: position,
            speed: 1.0,
            texture: texture.clone(),
            coll_rect: Rect::new(position.x, position.y, texture.width(), texture.height()),
        }
    }
}
