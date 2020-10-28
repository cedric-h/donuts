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

use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};
use math::*;

/// How deep in he claw what's being held should go.
const GRIP_DEPTH: f32 = 0.88;
#[derive(Copy, Clone)]
enum Hook {
    Retracting {
        pos: Vec2,
        reached: Vec2,
        facing: Vec2,
        started: f64,
    },
    Launched {
        vel: f32,
        pos: Vec2,
        facing: Vec2,
    },
    Locked {
        end: Vec2,
        facing: Vec2,
        chain_length: f32,
        can_index: usize,
        can_offset: Vec2,
        vel: Vec2,
    },
    Ready {
        facing: Vec2,
    },
}
impl Hook {
    fn new() -> Self {
        Hook::Ready { facing: Vec2::unit_x() }
    }

    fn face(&mut self, dock: Vec2, goal: Vec2) {
        if let Hook::Ready { facing } = self {
            *facing = slerp(*facing, (goal - dock).normalize(), 0.2);
        }
    }

    fn launch(&mut self, dock: Vec2) {
        if let &mut Hook::Ready { facing } = self {
            *self = Hook::Launched {
                pos: dock,
                vel: 1.25,
                facing,
            };
        }
    }

    fn retract(&mut self) {
        use Hook::*;
        if let Launched { pos, facing, .. } | Locked { end: pos, facing, .. } = *self {
            *self = Retracting {
                reached: pos,
                pos,
                facing,
                started: get_time(),
            };
        }
    }

    fn fly(&mut self, dock: Vec2) {
        match self {
            Hook::Launched { vel, pos, facing, } => {
                *vel *= 0.82;
                *pos += *facing * *vel;
                if *vel < 0.00001 {
                    self.retract()
                }
            },
            Hook::Retracting { pos, facing, reached, started } => {
                let delta = smoothstep((get_time() - *started) as f32);
                *pos = reached.lerp(dock, delta);
                if delta >= 1.0 {
                    *self = Hook::Ready { facing: *facing };
                }
            }
            _ => {},
        }
    }

    fn lock(&mut self, dock: Vec2, can_index: usize, can: &mut Can) {
        if let Hook::Launched { pos, facing, .. } = *self {
            let can_offset = (pos - can.pos).normalize() * GRIP_DEPTH;
            *self = Hook::Locked {
                facing: -can_offset.normalize(),
                chain_length: (dock - pos).length() + 0.5,
                end: can.pos + can_offset,
                vel: Vec2::zero(),
                can_offset,
                can_index,
            };
        }
    }

    fn drag(&mut self, dock: Vec2, can: &mut Can) {
        if let Hook::Locked { end, facing, chain_length, can_offset, vel, .. } = self {
            // drag hook towards car
            let delta = *end - dock;
            let hook_dist = delta.length();
            *chain_length = chain_length.min(hook_dist).max(1.5);

            if hook_dist > *chain_length {
                let pull = delta / hook_dist;
                *vel += pull * (*chain_length - hook_dist);
            }

            // drag can with hook
            let delta = can.pos - *end;
            let can_dist = delta.length();

            let pull = delta / can_dist;
            *facing = pull;
            can.pos = *end + pull * GRIP_DEPTH;

            *vel *= 0.98;
            *end += *vel;
        }
    }

    fn release(&mut self, can: &mut Can) {
        if let Hook::Locked { vel, .. } = *self {
            can.vel += vel * 1.4;
            self.retract();
        }
    }

    fn draw_hook(&mut self, dock: Vec2) {
        fn hook(dock: Vec2, facing: Vec2, twist: f32) {
            fn claw(mut base: Vec2, tip: f32, dir: f32) {
                let out = angle_to_vec(tip + FRAC_PI_2 * dir);
                let claw_blade = angle_to_vec(tip + FRAC_2_PI * dir);
                base += out * 0.1;

                fn tri(base: Vec2, x: Vec2, y: Vec2, z: Vec2) {
                    draw_triangle(x + base, y + base, z + base, LIGHTGRAY)
                }
                tri(base, angle_to_vec(tip) * 0.92, claw_blade * 0.34, Vec2::zero());
            };
            let (x, y) = (dock + facing * 0.05).into();
            draw_circle(x, y, 0.17, LIGHTGRAY);

            let facing_angle = vec_to_angle(facing);
            let claw_dock = dock - facing * 0.07;
            claw(claw_dock, facing_angle + twist, -1.0);
            claw(claw_dock, facing_angle - twist, 1.0);
        }

        let squeeze = ((get_time() * 7.5).sin() as f32) * 0.01;
        match *self {
            Hook::Ready { facing } => hook(dock, facing, squeeze),
            Hook::Retracting { pos, facing, .. } => hook(pos, facing, squeeze),
            Hook::Launched { pos, facing, .. } => hook(pos, facing, -0.05 + squeeze),
            Hook::Locked { end, facing, .. } => hook(end, facing, -0.355),
        }
    }

    fn draw_chain(&mut self, dock: Vec2) {
        fn chain(start: Vec2, end: Vec2) {
            const LINK_LENGTH: f32 = 0.35;
            const LINK_WIDTH: f32 = 0.12;
            const LINK_OVERLAP: f32 = 0.12;
            let normal = (start - end).normalize() * LINK_LENGTH;
            let out_dir = angle_to_vec(vec_to_angle(normal) + FRAC_PI_2);
            let out = out_dir * LINK_WIDTH;
            let link_count = (start - end).length() / LINK_LENGTH;

            fn line(s: Vec2, e: Vec2) {
                let (x, y) = s.into();
                let (w, z) = e.into();
                draw_line(x, y, w, z, 0.07, LIGHTGRAY);
            }

            for link in (0..link_count as usize) {
                let w = out_dir * (link as f32 * 0.125 + get_time() as f32 * 4.0).sin() * 0.01;
                let start_middle = end + normal * (link as f32 - LINK_OVERLAP) + w;
                let end_middle = end + normal * ((link + 1) as f32 + LINK_OVERLAP);
                if link % 2 == 0 {
                    line(start_middle, end_middle);
                } else {
                    line(start_middle + out, start_middle - out);
                    line(end_middle + out, end_middle - out);
                    line(start_middle + out, end_middle + out);
                    line(start_middle - out, end_middle - out);
                }
            }

            line(end + normal * link_count.floor(), start);
        }

        match *self {
            Hook::Launched { pos, .. } => chain(dock, pos),
            Hook::Retracting { pos, .. } => chain(dock, pos),
            Hook::Locked { end, .. } => chain(dock, end),
            _ => {},
        }
    }

    fn circles(&self) -> impl Iterator<Item = Circle> {
        use Hook::*;
        match *self {
            Ready { .. } | Retracting { .. } | Locked { .. } => None,
            Launched { facing, pos, .. } => Some(Circle {
                pos: pos + facing * 0.75,
                radius: 0.1,
                key: ArenaKey::Hook,
            }),
        }
        .into_iter()
    }
}

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
