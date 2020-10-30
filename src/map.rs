use super::math::*;
use macroquad::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

const TRACK_RADIUS: f32 = 35.0;
const TRACK_WIDTH: f32 = 10.0;
#[cfg(feature = "donutvision")]
const ROAD_3DNESS: f32 = 1.0;
#[cfg(not(feature = "donutvision"))]
const ROAD_3DNESS: f32 = 0.175;

pub struct Map;
impl Map {
    pub fn draw(&self) {
        self.track();
        self.lines();
        self.arrows();
    }

    fn track(&self) {
        draw_circle(0.0, -ROAD_3DNESS, TRACK_RADIUS, DARKGRAY);
        draw_circle(0.0, 0.0, TRACK_RADIUS, GRAY);
        draw_circle(0.0, 0.0, TRACK_RADIUS - TRACK_WIDTH, DARKGRAY);
        draw_circle(0.0, -ROAD_3DNESS, TRACK_RADIUS - TRACK_WIDTH, WHITE);
    }

    pub fn lines(&self) {
        const MAX: usize = 50;
        for i in 0..MAX {
            let f = i as f32 / MAX as f32;
            let width = TRACK_RADIUS - TRACK_WIDTH / 2.0;
            let (x, y) = (angle_to_vec(f * TAU) * width).into();
            let (w, z) = (angle_to_vec((f + 0.01) * TAU) * width).into();
            draw_line(x, y, w, z, 0.2, YELLOW);
        }
    }

    fn arrows(&self) {
        let v = self.car_spawn().y();
        const MAX: usize = 4;
        const SIZE: f32 = 1.2;
        for i in 0..MAX {
            let a = -(i as f32 / MAX as f32) * TAU;
            let p = angle_to_vec(a + FRAC_PI_2);
            let tip = angle_to_vec(a);
            for n in 0..(i + 1) {
                let arrow_offset = tip * n as f32 * 0.545;
                draw_triangle(
                    p * (v - SIZE / 2.0) + arrow_offset,
                    p * (v + SIZE / 2.0) + arrow_offset,
                    p * v + tip * SIZE + arrow_offset,
                    YELLOW,
                );
            }
        }
    }

    pub fn car_spawn(&self) -> Vec2 {
        vec2(0.0, TRACK_RADIUS - TRACK_WIDTH / 4.0)
    }

    pub fn can_spots(&self) -> impl Iterator<Item = Vec2> {
        const MAX: usize = 20;
        (0..MAX).map(|i| {
            angle_to_vec((i as f32 / MAX as f32) * TAU)
                * (TRACK_RADIUS - (TRACK_WIDTH * [-0.35, 0.3][i % 2]) - (TRACK_WIDTH / 2.0))
        })
    }

    pub fn rock_spots(&self) -> impl Iterator<Item = Vec2> {
        const MAX: usize = 50;
        (0..MAX).map(|i| {
            angle_to_vec((i as f32 / MAX as f32) * TAU)
                * (TRACK_RADIUS)
        })
    }

    pub fn terrain_friction(&self, pos: Vec2) -> f32 {
        if pos.length() < TRACK_RADIUS - TRACK_WIDTH {
            0.98
        } else if pos.length() > TRACK_RADIUS {
            0.98
        } else {
            0.96
        }
    }
}
