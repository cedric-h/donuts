use super::{ArenaKey, Circle};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Rock {
    pub pos: Vec2,
    sides: u8,
    rotation: f32,
    radius: f32,

}
impl Rock {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            sides: rand::gen_range(3, 9),
            rotation: rand::gen_range(0.0, 6.3),
            radius: rand::gen_range(0.1, 1.0),

        }
    }

    fn draw(&self) {
        let (x, y) = self.pos.into();
        draw_poly(x, y, self.sides, self.radius , self.rotation, BLACK);
    }
}

/// Thin wrapper around a Vec of Cans.
/// Abstracts away the fact that in order to preserve the 3d illusion,
/// cans need to be sorted by their x position before being rendered.
pub struct Rocktainer {
    rocks: Vec<Rock>,
    temp: Vec<Rock>,
}
impl Rocktainer {
    pub fn new(rocks: Vec<Rock>) -> Self {
        Self {
            temp: rocks.clone(),
            rocks,
        }
    }

    pub fn draw(&mut self) {
        use std::cmp::Ordering;
        self.temp = self.rocks.clone();
        self.temp
            .sort_by(|a, b| b.pos.x().partial_cmp(&a.pos.x()).unwrap_or(Ordering::Less));
        for can in &self.temp {
            can.draw()
        }
    }

    pub fn circles(&self) -> impl Iterator<Item = Circle> + '_ {
        self.rocks.iter().enumerate().map(|(i, c)| Circle {
            pos: c.pos,
            radius: 0.5,
            key: ArenaKey::Rock,
        })
    }
}
impl std::ops::Deref for Rocktainer {
    type Target = Vec<Rock>;

    fn deref(&self) -> &Self::Target {
        &self.rocks
    }
}
impl std::ops::DerefMut for Rocktainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rocks
    }
}
