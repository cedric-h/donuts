use macroquad::prelude::Vec2;

#[derive(Copy, Clone)]
pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
    pub key: ArenaKey,
}
#[derive(Copy, Clone, PartialEq)]
pub enum ArenaKey {
    Hook,
    Car,
    Rock,
    Can(usize),
}
pub struct Collision {
    pub members: [ArenaKey; 2],
    pub normal: Vec2,
    pub depth: f32,
}

pub struct CircleArena {
    circles: Vec<Circle>,
    collided: Vec<Collision>,
}
impl CircleArena {
    pub fn new() -> Self {
        Self {
            circles: Vec::with_capacity(1000),
            collided: Vec::with_capacity(1000),
        }
    }

    pub fn collide(&mut self, new_circles: impl Iterator<Item = Circle>) {
        let Self {
            circles, collided, ..
        } = self;
        circles.clear();
        circles.extend(new_circles);

        collided.clear();
        for c0 in &*circles {
            for c1 in &*circles {
                if c0.key != c1.key {
                    let delta = c0.pos - c1.pos;
                    let dist = delta.length();
                    let depth = (c0.radius + c1.radius) - dist;
                    if depth > 0.0 {
                        collided.push(Collision {
                            normal: delta.normalize(),
                            members: [c0.key, c1.key],
                            depth,
                        });
                    }
                }
            }
        }
    }

    pub fn collided(&mut self) -> impl ExactSizeIterator<Item = Collision> + '_ {
        self.collided.drain(..)
    }
}
