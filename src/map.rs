use macroquad::prelude::*;
use super::vec::*;

const TRACK_RADIUS: f32 = 35.0;
const TRACK_WIDTH: f32 = 10.0;

pub struct Map;
impl Map {
    pub fn draw(&self) {
        #[cfg(feature = "donutvision")]
        const ROAD_THICKNESS: f32 = 1.0;
        #[cfg(not(feature = "donutvision"))]
        const ROAD_THICKNESS: f32 = 0.175;

        draw_circle(0.0, -ROAD_THICKNESS, TRACK_RADIUS, DARKGRAY);
        draw_circle(0.0,  0.0, TRACK_RADIUS, GRAY);
        draw_circle(0.0,  0.0, TRACK_RADIUS - TRACK_WIDTH, DARKGRAY);
        draw_circle(0.0, -ROAD_THICKNESS, TRACK_RADIUS - TRACK_WIDTH, WHITE);

        const MAX: usize = 50;
        for i in 0..MAX {
            use std::f32::consts::TAU;

            let f = i as f32 / MAX as f32;
            let width = TRACK_RADIUS - TRACK_WIDTH/2.0;
            let (x, y) = (angle_to_vec(f * TAU) * width).into();
            let (w, z) = (angle_to_vec((f + 0.01) * TAU) * width).into();
            draw_line(x, y, w, z, 0.2, YELLOW);
        }

        let v = self.car_spawn();
        draw_triangle(
            v + Vec2::unit_y() * -0.5 * 1.2,
            v + Vec2::unit_y() * 0.5 * 1.2,
            v + Vec2::unit_x() * 1.2,
            YELLOW
        );
    }

    pub fn car_spawn(&self) -> Vec2 {
        vec2(0.0, TRACK_RADIUS - TRACK_WIDTH/4.0)
    }
}
