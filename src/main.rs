use macroquad::prelude::*;

mod vec;
mod car;
use car::Car;

struct Map;
const TRACK_RADIUS: f32 = 35.0;
const TRACK_WIDTH: f32 = 10.0;
impl Map {
    fn draw(&self) {
        draw_circle(0.0, 0.0, TRACK_RADIUS, GRAY);
        draw_circle(0.0, 0.0, TRACK_RADIUS - TRACK_WIDTH, WHITE);

        const MAX: usize = 50;
        for i in 0..MAX {
            use std::f32::consts::TAU;

            let f = i as f32 / MAX as f32;
            let width = TRACK_RADIUS - TRACK_WIDTH/2.0;
            let (x, y) = (vec::angle_to_vec(f * TAU) * width).into();
            let (w, z) = (vec::angle_to_vec((f + 0.01) * TAU) * width).into();
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

    fn car_spawn(&self) -> Vec2 {
        vec2(0.0, TRACK_RADIUS - TRACK_WIDTH/4.0)
    }
}

#[macroquad::main("donuts")]
async fn main() {
    let mut car = Car::new(load_texture("car.png").await);
    let map = Map;
    car.pos = map.car_spawn();

    loop {
        car.controls();

        clear_background(WHITE);

        set_camera(Camera2D {
            target: car.pos,
            rotation: car.angle(),
            zoom: vec2(1.0, -screen_width() / screen_height()) / 8.0,
            ..Default::default()
        });

        map.draw();
        car.draw();

        next_frame().await
    }
}
