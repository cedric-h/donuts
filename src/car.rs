use macroquad::prelude::*;
use super::vec::*;

pub struct Car {
    pub tex: Texture2D,
    pub pos: Vec2,
    pub dir: Vec2,
    speed: f32,
    vel: Vec2,
    throttle_time: Option<f64>,
}
impl Car {
    pub fn new(tex: Texture2D) -> Self {
        set_texture_filter(tex, FilterMode::Nearest);

        Self {
            tex,
            pos: vec2(0.0, 0.0),
            dir: vec2(1.0, 0.0),
            vel: vec2(0.0, 0.0),
            speed: 0.0,
            throttle_time: None,
        }
    }
    
    pub fn angle(&self) -> f32 {
        vec_to_angle(self.vel).to_degrees() + 90.0
    }

    pub fn controls(&mut self) {
        use std::f32::consts::PI;
        const MAX_SPEED: f32 = 0.05;
        let Self { speed, throttle_time, dir, pos, vel, .. } = self;
        let angle = vec_to_angle(*dir);

        if is_key_down(KeyCode::W) {
            let throttle = match throttle_time {
                Some(x) => {
                    let t = (get_time() - *x) as f32;
                    (t / 0.8).sqrt().min(1.0)
                },
                None => {
                    *throttle_time = Some(get_time());
                    0.0
                }
            };

            *speed = MAX_SPEED * throttle;
            *vel += *dir * throttle;
            *vel = if vel.length_squared() != 0.0 {
                vel.normalize()
            } else {
                *vel
            };
        } else {
            *throttle_time = None;
        }
            
        match (is_key_down(KeyCode::D), is_key_down(KeyCode::A)) {
            (true, false) => *dir = angle_to_vec(angle + PI/256.0 * *speed/MAX_SPEED),
            (false, true) => *dir = angle_to_vec(angle - PI/256.0 * *speed/MAX_SPEED),
            _ => {}
        }

        *speed *= 0.98;
        *pos += *vel * *speed;
    }

    pub fn draw(&self) {
        let &Self { tex, pos, vel, dir, .. } = self;
        let tex_size = vec2(tex.width(), tex.height()) * 0.125;
        let rot = vel.angle_between(dir);
        let (x, y) = (pos - tex_size / 2.0).into();
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                rotation: if rot.is_nan() { 0.0 } else { rot },
                dest_size: Some(tex_size),
                ..Default::default()
            }
        );
    }
}
