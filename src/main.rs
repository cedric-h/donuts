use macroquad::prelude::*;

mod map;
use map::Map;
mod vec;
mod car;
use car::Car;

struct Can {
    pos: Vec2,
}
impl Can {
    fn draw(&self) {
        let &Self { pos } = self;
        let (x, y) = pos.into();
        draw_circle(x, y, 0.5, DARKBROWN);
        draw_rectangle(x, y - 0.5, 0.44, 1.0, DARKBROWN);
        draw_circle(x + 0.44, y, 0.5, BROWN);
    }
}

#[macroquad::main("donuts")]
async fn main() {
    let mut car = Car::new(load_texture("car.png").await);
    let map = Map;
    car.pos = map.car_spawn();
    let can = Can {
        pos: map.car_spawn() - vec2(-1.0, 4.3),
    };

    loop {
        car.controls();

        clear_background(WHITE);

        #[cfg(feature = "donutvision")]
        set_camera(Camera2D {
            rotation: car.angle(),
            zoom: vec2(1.0, -screen_width() / screen_height()) / 55.0,
            ..Default::default()
        });

        #[cfg(not(feature = "donutvision"))]
        set_camera(Camera2D {
            target: car.pos,
            rotation: car.angle(),
            zoom: vec2(1.0, -screen_width() / screen_height()) / 8.0,
            ..Default::default()
        });

        map.draw();
        car.draw();
        can.draw();

        next_frame().await
    }
}
