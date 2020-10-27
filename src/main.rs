use macroquad::prelude::*;

mod map;
use map::Map;
mod vec;
mod car;
use car::Car;
mod circle;
use circle::{Circle, Collision, CircleArena, ArenaKey};
mod can;
use can::{Can, Cantainer};

#[macroquad::main("donuts")]
async fn main() {
    let mut arena = CircleArena::new();
    let map = Map;
    let mut car = Car {
        pos: map.car_spawn(),
        ..Car::new(load_texture("car.png").await)
    };
    let mut cans = Cantainer::new(map.can_spots().map(|pos| Can::new(pos)).collect());

    loop {
        car.controls(map.terrain_friction(car.pos));
        for can in &mut *cans {
            can.slide(map.terrain_friction(can.pos))
        }

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
        cans.draw();

        arena.collide(car.circles().chain(cans.circles()));
        for Collision { members, normal, .. } in arena.collided() {
            match members {
                [_, ArenaKey::Can(i)] => cans[i].knockback(-normal),
                _ => {},
            }
        }

        next_frame().await
    }
}
