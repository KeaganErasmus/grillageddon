use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub texture: Texture2D,
    pub coll_rect: Rect,
    pub target: Vec2,
    pub is_active: bool,
    pub velocity: Vec2,
    pub speed: f32,
}
impl Bullet {
    pub fn new(
        position: Vec2,
        texture: &Texture2D,
        target: Vec2,
        is_active: bool,
        speed: f32,
    ) -> Bullet {
        let direction = target - position;
        Bullet {
            position: position,
            texture: texture.clone(),
            coll_rect: Rect::new(position.x, position.y, texture.width(), texture.height()),
            target: target,
            is_active: is_active,
            velocity: direction.normalize(),
            speed: speed,
        }
    }
}
