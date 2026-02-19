use na;
use rand::{Rng, RngCore};
pub mod ray;
pub type Real = f32;

pub const PI: Real = 3.1415926535897932385;

pub trait RandomVec {
    fn random(rng: &mut dyn RngCore, min: Real, max: Real) -> Self;

    fn random_in_unit_sphere(rng: &mut dyn RngCore) -> Self;

    fn random_unit_vector(rng: &mut dyn RngCore) -> Self;

    fn random_in_hemisphere(rng: &mut dyn RngCore, normal: &Vec3) -> Self;
}

pub trait VecUtils {
    fn near_zero(&self) -> bool;

    fn reflect(&self, rhs: &Self) -> Self;

    fn refract(uv: &Self, n: &Self, etai_over_etat: Real) -> Self;
}

pub type Vec3 = na::TVec3<Real>;

impl VecUtils for Vec3 {
    fn near_zero(&self) -> bool {
        const S: Real = 1e-8;

        self.x.abs() < S && self.y.abs() < S && self.z < S
        //self.abs().lt(&Vec3::from_element(1e-8))
    }

    fn reflect(&self, rhs: &Self) -> Self {
        self - 2.0 * self.dot(rhs) * rhs
    }

    fn refract(uv: &Self, n: &Self, etai_over_etat: Real) -> Self {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl RandomVec for Vec3 {
    fn random(rng: &mut dyn RngCore, min: Real, max: Real) -> Self {
        Vec3::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }
    fn random_in_unit_sphere(rng: &mut dyn RngCore) -> Self {
        let mut p;

        loop {
            p = Self::random(rng, -1.0, 1.0);
            if p.magnitude_squared() < 1.0 {
                break;
            }
        }

        p
    }

    fn random_unit_vector(rng: &mut dyn RngCore) -> Self {
        Self::random_in_unit_sphere(rng).normalize()
    }

    fn random_in_hemisphere(rng: &mut dyn RngCore, normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}
