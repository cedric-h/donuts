use super::{ArenaKey, Circle};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Can {
    pub pos: Vec2,
    pub vel: Vec2,
    pub obj: Obj,
}

#[derive(Clone)]
pub enum Obj {
    Barrel,
    Rock,
}

impl Can {
    pub fn new(pos: Vec2, obji: Obj) -> Self {
        Self {
            pos,
            vel: Vec2::zero(),
            obj: obji,
        }
    }

    fn draw(&self) {
        let (x, y) = self.pos.into();
        match self.obj {
            Obj::Barrel => {
                draw_circle(x, y, 0.5, DARKBROWN);
                draw_rectangle(x, y - 0.5, 0.44, 1.0, DARKBROWN);
                draw_circle(x + 0.44, y, 0.5, BROWN);
            }
            Obj::Rock => {
                draw_circle(x, y, 0.5, GRAY);
            }
        }
    }

    pub fn slide(&mut self, friction: f32) {
        let Self { pos, vel, obj } = self;
        match self.obj {
            Obj::Barrel => {
                *vel *= friction;
                *pos += *vel;
            }
            Obj::Rock => {}
        }
    }

    pub fn knockback(&mut self, normal: Vec2) {
        match self.obj {
            Obj::Barrel => {
                self.vel += normal;
            }
            Obj::Rock => {}
        }
    }
}

/// Thin wrapper around a Vec of Cans.
/// Abstracts away the fact that in order to preserve the 3d illusion,
/// cans need to be sorted by their x position before being rendered.
pub struct Cantainer {
    cans: Vec<Can>,
    temp: Vec<Can>,
}
impl Cantainer {
    pub fn new(cans: Vec<Can>) -> Self {
        Self {
            temp: cans.clone(),
            cans,
        }
    }

    pub fn draw(&mut self) {
        use std::cmp::Ordering;
        self.temp = self.cans.clone();
        self.temp
            .sort_by(|a, b| b.pos.x().partial_cmp(&a.pos.x()).unwrap_or(Ordering::Less));
        for can in &self.temp {
            can.draw()
        }
    }

    pub fn circles(&self) -> impl Iterator<Item = Circle> + '_ {
        self.cans.iter().enumerate().map(|(i, c)| Circle {
            pos: c.pos,
            radius: 0.5,
            key: ArenaKey::Can(i),
        })
    }
}
impl std::ops::Deref for Cantainer {
    type Target = Vec<Can>;

    fn deref(&self) -> &Self::Target {
        &self.cans
    }
}
impl std::ops::DerefMut for Cantainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cans
    }
}
