use super::*;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            dir: Vec3::normalize(&direction),
        }
    }

    pub fn origin(self: &Self) -> Vec3 {
        self.origin
    }

    pub fn direction(self: &Self) -> Vec3 {
        self.dir
    }

    pub fn at(self: &Self, t: Real) -> Vec3 {
        self.origin + self.dir * t
    }
}
