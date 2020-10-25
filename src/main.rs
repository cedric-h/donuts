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

        let v = self.car_spawn();
        draw_triangle(
            v + Vec2::unit_y() * -0.5,
            v + Vec2::unit_y() * 0.5,
            v + Vec2::unit_x(),
            YELLOW
        );
    }

    fn car_spawn(&self) -> Vec2 {
        vec2(0.0, TRACK_RADIUS - TRACK_WIDTH/2.0)
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
