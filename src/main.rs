use log::error;
use std::sync::Arc;

use math::{Real, Vec3};
use pixels::{Error, Pixels, SurfaceTexture};
use rand::{Rng, RngCore, distributions::Uniform, thread_rng};
use scene::{Hittable, camera::Camera, material::*, scene::Scene, sphere::Sphere};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use rayon::prelude::*;

extern crate nalgebra_glm as na;

mod math;
mod scene;

const WIDTH: u32 = 400;
const ASPECT_RATIO: Real = 16.0 / 9.0;
const HEIGHT: u32 = (WIDTH as Real / ASPECT_RATIO) as u32;

const CAMERA_SPEED: Real = 0.5;

fn gen_image(
    frame: &mut [u8],
    rng: &mut dyn RngCore,
    cam: &Camera,
    scene: &Scene,
    width: u32,
    height: u32,
    samples: u32,
) {
    /*let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);*/

    //let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    //for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    frame
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, pixel)| {
            let mut r = thread_rng();
            //let u = (x as Real) / ((WIDTH - 1) as Real);
            //let v = (y as Real) / ((HEIGHT - 1) as Real);
            let x = (i % width as usize) as u32;
            let y = (i / width as usize) as u32;

            //let r = Ray::new(origin, lower_left + u * horizontal + v * vertical - origin);
            let mut color = Vec3::zeros();

            for _ in 0..samples {
                let u = (x as Real + r.sample(Uniform::new_inclusive(0.0 as Real, 1.0 as Real)))
                    / (width as Real - 1.0);
                let v = ((height - y - 1) as Real + r.sample(Uniform::new_inclusive(0.0, 1.0)))
                    / (height as Real - 1.0);

                let ray = cam.get_ray(u, v);

                if let Some(col) = scene.raytrace(&mut r, &ray, 16) {
                    //let col = 0.5 * (rec. + Vec3::new(1.0, 1.0, 1.0));
                    //*pixel = Rgb([col.x as f32, col.y as f32, col.z as f32]);
                    color += col;
                } else {
                    //*pixel = Rgb([0.0, 0.0, 0.0]);
                    //*pixel = f(&r);
                }
            }

            color /= samples as Real;

            let col_arr = [
                (256.0 * color.x.clamp(0.0, 0.999)) as u8,
                (256.0 * color.y.clamp(0.0, 0.999)) as u8,
                (256.0 * color.z.clamp(0.0, 0.999)) as u8,
                255,
            ];

            pixel.copy_from_slice(&col_arr)
        });
}

fn main() -> Result<(), Error> {
    use std::time::Instant;
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

        WindowBuilder::new()
            .with_title("Raytracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut rng = rand::thread_rng();

    let mat_left = Arc::new(Lambertian { albedo: Vec3::z() });
    let mat_right = Arc::new(Lambertian { albedo: Vec3::x() });

    //let mut origin = Vec3::zeros();

    let radius: Real = na::quarter_pi::<Real>().cos();

    let scene = Scene::new(vec![
        Box::new(Sphere {
            center: Vec3::new(-radius, 0.0, -1.0),
            radius: radius,
            material: mat_left.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(radius, 0.0, -1.0),
            radius: radius,
            material: mat_right.clone(),
        }),
        /*Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: mat_ground.clone()
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: mat_center.clone()
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: mat_left.clone()
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.4,
            material: mat_left.clone()
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: mat_right.clone()
        }),*/
    ]);

    let mut cam_pos = Vec3::new(-2.0, 2.0, 1.0);
    let cam_lookat = -Vec3::z();
    let cam_up = Vec3::y();

    let mut camera = Camera::new(cam_pos, &cam_lookat, &cam_up, 90.0, ASPECT_RATIO);

    let now = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            gen_image(
                pixels.get_frame(),
                &mut rng,
                &camera,
                &scene,
                WIDTH,
                HEIGHT,
                16,
            );

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
                return;
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                cam_pos -= (cam_lookat - cam_pos)
                    .normalize()
                    .cross(&cam_up)
                    .normalize()
                    * CAMERA_SPEED;
                camera = Camera::new(cam_pos, &cam_lookat, &cam_up, 90.0, ASPECT_RATIO);
                window.request_redraw();
            } else if input.key_pressed(VirtualKeyCode::Right) {
                cam_pos += (cam_lookat - cam_pos)
                    .normalize()
                    .cross(&cam_up)
                    .normalize()
                    * CAMERA_SPEED;
                camera = Camera::new(cam_pos, &cam_lookat, &cam_up, 90.0, ASPECT_RATIO);
                window.request_redraw();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
        }
    });

    /*{
        let img: RgbImage =
        img.save("image.png").unwrap();
    }*/
}
