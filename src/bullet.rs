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
    pub async fn new(position: Vec2, target: Vec2, is_active: bool, speed: f32) -> Bullet {
        let bullet_texture = load_texture("assets/bullet.png").await.unwrap();
        let direction = target - position;
        Bullet {
            position: position,
            texture: bullet_texture,
            coll_rect: Rect::new(position.x, position.y, 8.0, 8.0),
            target: target,
            is_active: is_active,
            velocity: direction.normalize(),
            speed: speed,
        }
    }
}
