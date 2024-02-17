use std::{f64::INFINITY, ops::{Add, Div, Mul, Sub}};
use rand::prelude::*;

fn main() {
    let ratio = 16.0 / 9.0;
    let width: u64 = 384;
    let height: u64 = (width as f64 / ratio) as u64;
    let samples_per_pixel = 100;

    println!("P3");
    println!("{:}", width);
    println!("{:}", height);
    println!("255");

    let v_height = 2.0;
    let v_width = ratio * v_height;
    let focal_length = 1.0;

    let origin = Point{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Point{x: v_width, y: 0.0, z: 0.0};
    let vertical = Point{x: 0.0, y: v_height, z: 0.0};
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Point{x: 0.0, y: 0.0, z: focal_length};

    let mut world: World = World::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::default();

    for j in (0..height).rev() {
        for i in 0..width {
            eprint!("\rx:{:5}, y:{:5}", i, j);

            let mut pixel = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / ((width - 1) as f64);
                let v = (j as f64 + random::<f64>()) / ((height - 1) as f64);
                let ray = cam.get_ray(u, v);
                pixel = pixel + ray.color(&world);
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
        let mut ret_rec: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, tmin, closest_so_far) {
                Some (rec) => {
                    closest_so_far = rec.t;
                    ret_rec = Some(rec);
                },
                None => (),
            }
        }

        ret_rec
    }
}

fn write_color(color: Color, samples_per_pixel: u64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / (samples_per_pixel as f64);
    r = r * scale;
    b = b * scale;
    g = g * scale;
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

    fn color(self, world: &World) -> Color {
        match world.hit(self, 0.0, INFINITY) {
            Some(rec) => (Color::new(1.0, 1.0, 1.0) + rec.n) * 0.5,
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

#[derive(Copy, Clone, Debug, PartialEq)]
struct HitRecord {
    p: Point,
    n: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn new(ray: Ray, p: Point, out_n: Vec3, t: f64) -> Self {
        let front_face = ray.direction.dot(out_n) < 0.0;
        let n = if front_face { out_n } else { out_n * (-1.0) };
        HitRecord{p, n, t, front_face}
    }
}

trait Hittable {
    fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    fn new(center: Point, radius: f64) -> Self {
        Sphere{center, radius}
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
                Some(HitRecord::new(ray, p, n, t))
            } else if tmin < tplus && tplus < tmax {
                let t = tplus;
                let p = ray.at(t);
                let n = (p - self.center) / self.radius;
                Some(HitRecord::new(ray, p, n, t))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3{x, y, z}
    }
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
    fn norm(self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    fn dot(self, other: Self) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn unit(self) -> Self {
        self / self.norm()
    }
}