use macroquad::prelude::*;

mod map;
use map::Map;
mod math;
mod car;
use car::Car;
mod circle;
use circle::{Circle, Collision, CircleArena, ArenaKey};
mod can;
use can::{Can, Cantainer};
mod hook;
use hook::Hook;

#[cfg(not(feature = "donutvision"))]
const ZOOM: f32 = 8.0;
#[cfg(feature = "donutvision")]
const ZOOM: f32 = 55.0;

#[macroquad::main("donuts")]
async fn main() {
    let mut arena = CircleArena::new();
    let map = Map;
    let mut car = Car {
        pos: map.car_spawn(),
        ..Car::new(load_texture("car.png").await)
    };
    let mut hook = Hook::new();
    let mut cans = Cantainer::new(map.can_spots().map(|pos| Can::new(pos)).collect());

    loop {
        clear_background(WHITE);

        let cam = Camera2D {
            rotation: car.angle(),
            #[cfg(not(feature = "donutvision"))]
            target: car.pos,
            zoom: vec2(1.0, -screen_width() / screen_height()) / ZOOM,
            ..Default::default()
        };

        match hook {
            Hook::Ready { .. } => hook.face(car.dock(), cam.screen_to_world(mouse_position().into())),
            Hook::Launched { .. } | Hook::Retracting { .. } => hook.fly(car.dock()),
            Hook::Locked { can_index: i, .. } => hook.drag(car.dock(), &mut cans[i]),
        }
        car.controls(map.terrain_friction(car.pos));
        for can in &mut *cans {
            can.slide(map.terrain_friction(can.pos))
        }
        if is_mouse_button_down(MouseButton::Left) {
            match hook {
                Hook::Ready { .. } => hook.launch(car.dock()),
                Hook::Locked { can_index: i, .. } => hook.release(&mut cans[i]),
                _ => {},
            }
        }

        set_camera(cam);
        map.draw();
        car.draw();
        hook.draw_hook(car.dock());
        cans.draw();
        hook.draw_chain(car.dock());

        #[cfg(feature = "showcollision")]
        for c in car.circles().chain(cans.circles()).chain(hook.circles()) {
            draw_circle_lines(c.pos.x(), c.pos.y(), c.radius, 0.1, RED);
        }

        arena.collide(car.circles().chain(cans.circles()).chain(hook.circles()));
        for Collision { members, normal, depth, .. } in arena.collided() {
            match members {
                [ArenaKey::Hook, ArenaKey::Can(i)] => hook.lock(car.dock(), i, &mut cans[i]),
                [ArenaKey::Can(i), ArenaKey::Hook] if cans[i].vel.length() < 0.5 => cans[i].knockback(normal * 0.1),
                [ArenaKey::Can(i), _] => cans[i].knockback(normal * depth),
                _ => {},
            }
        }

        next_frame().await
    }
}
