use super::*;

pub struct Ray {
    dir: Vec3,
    orig: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            dir, orig
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direct(&self) -> Vec3 {
        self.dir
    }
}