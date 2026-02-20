use crate::*;

use math::{Vec3, ray::Ray};

use super::HitRecord;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Scene { objects: objects }
    }

    pub fn hit(
        &self,
        rng: &mut dyn RngCore,
        r: &Ray,
        t_min: Real,
        t_max: Real,
    ) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;

        let mut closest: Real = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, t_min, closest) {
                closest = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }

    pub fn raytrace(&self, rng: &mut dyn RngCore, ray: &Ray, depth: usize) -> Option<Vec3> {
        if depth == 0 {
            return None;
        }

        match self.hit(rng, ray, 0.001, Real::INFINITY) {
            Some(rec) => {
                //let target = rec.pos + rec.normal + Vec3::random_in_unit_sphere(rng);
                if let Some(scatter) = rec.mat.scatter(rng, ray, &rec) {
                    Some(scatter.attenuation.component_mul(&self.raytrace(
                        rng,
                        &scatter.scattered,
                        depth - 1,
                    )?))
                } else {
                    None
                }
                /*Some(0.5 * self.raytrace(rng, &Ray {
                    origin: rec.pos,
                    dir: target - rec.pos
                }, depth - 1)?)*/
                //Some(rec.mat.scatter(rng, r, &rec)?.attenuation)
            }
            _ => {
                //None
                let unit_dir = ray.direction().normalize();
                let t = 0.5 * (unit_dir.y + 1.0);
                Some((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
            }
        }
    }
}

/*pub fn ray_color(r: &Ray) -> Rgb<Real> {
    let unit = r.direction();
    let t = 0.5 * (unit.y + 1.0);
    let color = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);

    Rgb([color.x as Real, color.y as Real, color.z as Real])
}*/
