extern crate image;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
// use crate::hittable::Hittable;
use std::time::Instant;
use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal, Scatter};
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Color, Point3, Vec3};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image::{RgbImage, Rgb};
fn random_scene(rng: &mut ThreadRng) -> HittableList {
    let mut world: HittableList = HittableList::new();
    let material_ground = Rc::new(Lambertian {
        albedo: Color(0.5, 0.5, 0.5),
    });
    world.objects.push(Rc::new(Sphere {
        center: Point3(0., -1000., 0.),
        radius: 1000.,
        mat: Rc::<Lambertian>::clone(&material_ground),
    }));
    for a in -1..1 {
        for b in -1..1 {
            let choose_mat = rng.gen_range(0. ..1.);
            let center = Point3(
                a as f64 + 0.9 * rng.gen_range(0. ..1.),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0. ..1.),
            );
            if (center - Point3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0., 1., rng) * Color::random(0., 1., rng);
                    let sphere_material = Rc::new(Lambertian { albedo });
                    world.objects.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::<Lambertian>::clone(&sphere_material),
                    }))
                } else if choose_mat < 0.95 {
                    // mental
                    let albedo = Color::random(0.5, 1., rng);
                    let fuzz = rng.gen_range(0. ..0.5);
                    let sphere_material = Rc::new(Metal { albedo, fuzz });
                    world.objects.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::<Metal>::clone(&sphere_material),
                    }))
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric { ir: 1.5 });
                    world.objects.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Rc::<Dielectric>::clone(&sphere_material),
                    }))
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.objects.push(Rc::new(Sphere {
        center: Point3(0., 1., 0.),
        radius: 1.,
        mat: Rc::<Dielectric>::clone(&material1),
    }));
    let material2 = Rc::new(Lambertian {
        albedo: Color(0.4, 0.2, 0.1),
    });
    world.objects.push(Rc::new(Sphere {
        center: Point3(-4., 1., 0.),
        radius: 1.,
        mat: Rc::<Lambertian>::clone(&material2),
    }));
    let material3 = Rc::new(Metal {
        albedo: Color(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.objects.push(Rc::new(Sphere {
        center: Point3(4., 1., 0.),
        radius: 1.,
        mat: Rc::<Metal>::clone(&material3),
    }));
    world
}
fn main() {
    let start = Instant::now();
    let aspect_ration = 3.0 / 2.0;
    let nx = 1200;
    let ny = (nx as f64 / aspect_ration) as usize;
    let samples_per_pixel = 50;
    let max_depth = 50;
    let mut rng = thread_rng();

    //world
    let world: HittableList = random_scene(&mut rng);
    let lookfrom = Point3(13., 2., 3.);
    let lookat = Point3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ration,
        aperture,
        dist_to_focus,
    );
    let mut img = RgbImage::new(nx, ny as u32);
    let mut buffer: Vec<u32> = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut pixel_color = Color(0., 0., 0.);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.)) / (nx - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.)) / (ny - 1) as f64;
                let r = cam.get_ray(u, v, &mut rng);
                pixel_color += ray_color(&r, &world, &mut rng, max_depth);
            }
            img.put_pixel(i,(ny - j -1) as u32,Rgb(from_vec3_rgb(pixel_color, samples_per_pixel)));
            // buffer.push(from_vec3_rgb(pixel_color, samples_per_pixel));
        }
    }

    // let mut window = Window::new("Test", nx, ny, WindowOptions::default()).unwrap();
    // window.update_with_buffer(&buffer, nx, ny).unwrap();
    img.save(Path::new("test.png")).unwrap();
    let duration = start.elapsed();
    println!("{:?}", duration);
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     window.update();
    // }
}
fn from_vec3_rgb(color: Color, samples_per_pixel: usize,) -> [u8;3] {
    let scale = 1. / samples_per_pixel as f64;
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let trans = |v: f64| (256. * (scale * v).sqrt().clamp(0., 0.999)) as u8;
    let (r, g, b) = (trans(color.0), trans(color.1), trans(color.2));
    // (r << 16) | (g << 8) | b
    [r,g,b]
}
fn ray_color(ray: &Ray, world: &HittableList, rng: &mut ThreadRng, depth: usize) -> Color {
    if depth <= 0 {
        // writeln!("endray");
        return Color(0., 0., 0.);
    }
    let mut rec = world.hit(ray, 0.001, f64::INFINITY);
    match rec {
        Some(x) => {
            // let target = x.p + x.normal + Vec3::random_in_unit_sphere(rng);
            if let Some((scattered, attenuation)) = x.mat.scatter(ray, &x, rng) {
                return attenuation * ray_color(&scattered, world, rng, depth - 1);
            } else {
                return Color(0., 0., 0.);
            }
        }
        None => {
            let unit_direction = ray.dir.unit_vec();
            let t = 0.5 * (unit_direction.1 + 1.0);
            return (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0);
        }
    }
}
