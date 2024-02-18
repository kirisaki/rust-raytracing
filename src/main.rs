use std::{cmp::min, f64::{consts::PI, INFINITY}, ops::{Add, Div, Mul, Neg, Sub}};
use rand::prelude::*;

fn main() {
    let ratio = 16.0 / 9.0;
    let width: u64 = 384;
    let height: u64 = (width as f64 / ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3");
    println!("{:}", width);
    println!("{:}", height);
    println!("255");

    let mut world: World = World::new();
    world.add(Box::new(Sphere::new
        ( Point::new(0.0, 0.0, -1.0)
        , 0.5
        , Material::Lambertian{albedo: Color::new(0.7, 0.3, 0.3)})));
    world.add(Box::new(Sphere::new
        ( Point::new(0.0, -100.5, -1.0)
        , 100.0
        , Material::Lambertian{albedo: Color::new(0.8, 0.8, 0.0)})));
    world.add(Box::new(Sphere::new
        ( Point::new(1.0, 0.0, -1.0)
        , 0.5
        , Material::Metal{albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0})));
    world.add(Box::new(Sphere::new
        ( Point::new(-1.0, 0.0, -1.0)
        , 0.5
        , Material::Dielectric{ref_idx: 1.5})));
    world.add(Box::new(Sphere::new
        ( Point::new(-1.0, 0.0, -1.0)
        , -0.45
        , Material::Dielectric{ref_idx: 1.5})));

    let cam = Camera::default();

    for j in (0..height).rev() {
        for i in 0..width {
            eprint!("\rx:{:5}, y:{:5}", i, j);

            let mut pixel = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / ((width - 1) as f64);
                let v = (j as f64 + random::<f64>()) / ((height - 1) as f64);
                let ray = cam.get_ray(u, v);
                pixel = pixel + ray.color(&world, max_depth);
            }
            write_color(pixel, samples_per_pixel);
        }
    }
    eprintln!();
}

struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    fn new() -> Self {
        let objects = Vec::<Box<dyn Hittable>>::new();
        World{objects}
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }

    fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = tmax;
        let mut ret: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, tmin, closest_so_far) {
                Some (rec) => {
                    closest_so_far = rec.t;
                    ret = Some(rec);
                },
                None => (),
            }
        }

        ret
    }
}

fn write_color(color: Color, samples_per_pixel: u64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    b = (b * scale).sqrt();
    g = (g * scale).sqrt();
    println!( "{:.0} {:.0} {:.0}"
            , (clamp(r, 0.0, 0.999) * 256.0).floor()
            , (clamp(g, 0.0, 0.999) * 256.0).floor()
            , (clamp(b, 0.0, 0.999) * 256.0).floor()
        )
}

fn clamp(x: f64, xmin: f64, xmax: f64) -> f64 {
    if x < xmin {
        xmin
    } else if xmax < x {
        xmax
    } else {
        x
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Point, direction: Vec3) -> Self {
        Ray{origin, direction}
    }
    fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    fn color(self, world: &World, depth: i64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(self, 0.0001, INFINITY) {
            Some(rec) => {
                match rec.material.scatter(self, &rec) {
                    Some((scattered, attenuation)) => scattered.color(world, depth - 1) * attenuation,
                    None => Color::new(0.0, 0.0, 0.0),
                }
            },
            None => {
                let u = self.direction.unit();
                let t = (u.y + 1.0) * 0.5;
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new(origin: Point, lower_left_corner: Point, horizontal: Vec3, vertical: Vec3) -> Self {
        Camera{origin, lower_left_corner, horizontal, vertical}
    }

    fn default() -> Self {
        let ratio = 16.0 / 9.0;
        let height = 2.0;
        let width = ratio * height;
        let focal = 1.0;
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal);
        Camera{origin, lower_left_corner, horizontal, vertical}
    }

    fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}

struct HitRecord {
    p: Point,
    n: Vec3,
    t: f64,
    material: Material,
    front_face: bool,
}

impl HitRecord {
    fn new(ray: Ray, p: Point, out_n: Vec3, t: f64, material: Material) -> Self {
        let front_face = ray.direction.dot(out_n) < 0.0;
        let n = if front_face { out_n } else { out_n * (-1.0) };
        HitRecord{p, n, t, material, front_face}
    }
}

trait Hittable {
    fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    fn new(center: Point, radius: f64, material: Material) -> Self {
        Sphere{center, radius, material}
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b =oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = half_b * half_b - a * c;

        if d > 0.0 {
            let root = d.sqrt();
            let tminus = (-half_b - root) / a;
            let tplus = (-half_b + root) / a;
            if tmin < tminus && tminus < tmax {
                let t = tminus;
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                Some(HitRecord::new(ray, p, n, t, self.material))
            } else if tmin < tplus && tplus < tmax {
                let t = tplus;
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                Some(HitRecord::new(ray, p, n, t, self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Lambertian{albedo: Color},
    Metal{albedo: Color, fuzz: f64},
    Dielectric{ref_idx: f64},
}

impl Material {
    fn scatter(self, rin: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian{albedo} => {
                let scatter_direction = rec.n + Vec3::random_unit();
                let scattered = Ray::new(rec.p, scatter_direction);
                let attenuation = albedo;
                Some((scattered, attenuation))
            },
            Material::Metal{albedo, fuzz} => {
                let reflected = rin.direction.unit().reflect(rec.n);
                let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * fuzz);
                let attenuation = albedo;
                if scattered.direction.dot(rec.n) > 0.0 {
                    Some((scattered, attenuation))
                } else {
                    None
                }
            },
            Material::Dielectric{ref_idx} => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let etai_over_etat = if rec.front_face {
                    1.0 / ref_idx
                } else {
                    ref_idx
                };

                let unit = rin.direction.unit();
                let cos_theta = f64::min((unit * (-1.0)).dot(rec.n), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let scattered = if etai_over_etat * sin_theta > 1.0 {
                    let reflected = unit.reflect(rec.n);
                    Ray::new(rec.p, reflected)
                } else {
                    let reflected_prob = schlick(cos_theta, etai_over_etat);
                    if random::<f64>() < reflected_prob {
                        let reflected = unit.reflect(rec.n);
                        Ray::new(rec.p, reflected)
                    } else {
                        let refracted = unit.refract(rec.n, etai_over_etat);
                        Ray::new(rec.p, refracted)
                    }
                };

                Some((scattered, attenuation))
            },
        }
    }
}

fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cos).powi(5)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

type Color = Vec3;
type Point = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self{x: self.x * t, y: self.y * t, z: self.z * t}
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self{x: self.x / t, y: self.y / t, z: self.z / t}
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3{x, y, z}
    }

    fn norm(self) -> f64 {
        self.dot(self).abs().sqrt()
    }

    fn norm_squared(self) -> f64 {
        self.dot(self).abs()
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Self::new( self.y * other.z - self.z * other.y
                 , self.z * other.x - self.x * other.z
                 , self.x * other.y - self.y * other.x
                )
    }

    fn unit(self) -> Self {
        self / self.norm()
    }

    fn random(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen_range(min..max) , rng.gen_range(min..max), rng.gen_range(min..max))
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.norm_squared() < 1.0 {
                return p;
            }
        }
    }

    fn random_unit() -> Self {
        let mut rng = thread_rng();
        let a: f64 = rng.gen_range(0.0..(2.0 * PI));
        let z: f64 = rng.gen_range(-1.0..1.0);
        let r: f64 = (1.0 - z * z).sqrt();
        Self::new(r*a.cos(), r*a.sin(), z)
    }

    fn random_in_hemisphere(self) -> Self {
        let in_unit_sphere = Self:: random_in_unit_sphere();
        if in_unit_sphere.dot(self) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * (-1.0)
        }
    }

    fn reflect(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n) * 2.0
    }

    fn refract(self, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (self * (-1.0)).dot(n);
        let rout_parallel = (self + n * cos_theta) * etai_over_etat;
        let rout_prep = n * (1.0 - rout_parallel.norm_squared()).sqrt().neg();
        rout_parallel + rout_prep
    }
}