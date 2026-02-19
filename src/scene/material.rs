use rand::{Rng, RngCore};

use crate::math::{RandomVec, Real, Vec3, VecUtils, ray::Ray};

use super::HitRecord;

pub struct MaterialResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<MaterialResult>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<MaterialResult> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some(MaterialResult {
            scattered: Ray {
                origin: hit_record.pos,
                dir: scatter_direction,
            },
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    fuzz: Real,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Real) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz.max(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<MaterialResult> {
        let scattered = Ray {
            origin: hit_record.pos,
            dir: (ray.direction().reflect(&hit_record.normal))
                + (self.fuzz * Vec3::random_in_unit_sphere(rng)),
        };

        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some(MaterialResult {
                scattered: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: Real,
}

impl Dielectric {
    pub fn new(ir: Real) -> Self {
        Self { ir: ir }
    }

    fn reflectance(cosine: Real, ref_idx: Real) -> Real {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hit_record: &HitRecord,
    ) -> Option<MaterialResult> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray.direction().normalize();

        // TODO: Remove this line (cos_theta is calculated here and in Vec3::refract)
        let cos_theta = (-unit_dir).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = {
            if cannot_refract
                || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..=1.0)
            {
                Vec3::reflect(&unit_dir, &hit_record.normal)
            } else {
                Vec3::refract(&unit_dir, &hit_record.normal, refraction_ratio)
            }
        };

        Some(MaterialResult {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered: Ray {
                origin: hit_record.pos,
                dir: direction,
            },
        })
    }
}
