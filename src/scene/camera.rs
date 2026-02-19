use crate::math::{Real, Vec3, ray::Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, lookat: &Vec3, up: &Vec3, vfov: Real, aspect_ratio: Real) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;

        let viewport_width = aspect_ratio * viewport_height;

        let w = (pos - lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let hori = viewport_width * u;
        let vert = viewport_height * v;
        let llc = pos - hori / 2.0 - vert / 2.0 - w;

        Self {
            origin: pos,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, s: Real, t: Real) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
