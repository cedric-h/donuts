use macroquad::prelude::*;
use super::vec::*;

fn smoothstep(x: f32) -> f32 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        3.0 * x.powi(2) - 2.0 * x.powi(3)
    }
}

#[derive(Copy, Clone)]
enum ThrottleSlide {
    Back {
        start: f64,
        forward_time: f64,
    },
    Forward {
        start: f64,
    },
    Nah
}

pub struct Car {
    pub tex: Texture2D,
    pub pos: Vec2,
    pub dir: Vec2,
    speed: f32,
    vel: Vec2,
    throttle_slide: ThrottleSlide,
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
            throttle_slide: ThrottleSlide::Nah,
        }
    }
    
    pub fn angle(&self) -> f32 {
        -(vec_to_angle(self.vel).to_degrees() + 90.0)
    }

    pub fn controls(&mut self, friction: f32) {
        use std::f32::consts::PI;
        const MAX_SPEED: f32 = 0.075;
        let Self { speed, throttle_slide, dir, pos, vel, .. } = self;
        let angle = vec_to_angle(*dir);

        *throttle_slide = match (is_key_down(KeyCode::W), *throttle_slide) {
            (true, ThrottleSlide::Nah) => ThrottleSlide::Forward {
                start: get_time()
            },
            (true, ThrottleSlide::Back { start, forward_time }) => ThrottleSlide::Forward {
                start: get_time() - (forward_time - (get_time() - start)).max(0.0),
            },
            (false, ThrottleSlide::Forward { start }) => ThrottleSlide::Back {
                start: get_time(),
                forward_time: (get_time() - start).min(4.0),
            },
            (_, o) => o,
        };

        let throttle = {
            let t = match *throttle_slide {
                ThrottleSlide::Forward { start } => (get_time() - start) as f32,
                ThrottleSlide::Back { forward_time, start } => (forward_time - (get_time() - start) * 2.0).max(0.0) as f32,
                ThrottleSlide::Nah => 0.0
            };

            const ZERO_TO_PLATEAU: f32 = 0.7;
            const PLATEAU: f32 = 0.5;
            const MAX_START: f32 = 2.0;
            const PLATEAU_TO_MAX: f32 = 2.2;
            const MAX: f32 = 1.0;
            if t < ZERO_TO_PLATEAU {
                smoothstep(t / ZERO_TO_PLATEAU) * PLATEAU
            } else if t > MAX_START {
                smoothstep((t - MAX_START) / PLATEAU_TO_MAX) * (MAX - PLATEAU) + PLATEAU
            } else {
                PLATEAU
            }
        };

        *speed = MAX_SPEED * throttle * ((vel.dot(*dir) - 0.87).max(0.0) / 0.1);
        *vel += *dir * 0.04 * throttle;

        *vel = if vel.length_squared() != 0.0 {
            vel.normalize()
        } else {
            *vel
        };
            
        let (l, r) = (is_key_down(KeyCode::D), is_key_down(KeyCode::A));
        if l ^ r {
            *dir = angle_to_vec(
                angle + PI/216.0
                    * (*speed / MAX_SPEED).min(1.0)
                    * if r { -1.0 } else { 1.0 },
            );
        }

        *speed *= friction;
        *pos += *vel * *speed;
    }

    pub fn draw(&self) {
        let &Self { tex, pos, dir, .. } = self;
        let tex_size = vec2(tex.width(), tex.height()) * 0.125;
        let (x, y) = (pos - tex_size / 2.0).into();
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                rotation: vec_to_angle(dir) + std::f32::consts::FRAC_PI_2,
                dest_size: Some(tex_size),
                ..Default::default()
            }
        );
    }
}
