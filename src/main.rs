use std::ops::{Add, Sub, Mul, Div};

fn main() {
    let ratio = 16.0 / 9.0;
    let width: u64 = 384;
    let height: u64 = (width as f64 / ratio) as u64;

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

    for j in (0..height).rev() {
        for i in 0..width {
            eprint!("\rx:{:5}, y:{:5}", i, j);
            let u = i as f64 / ((width - 1) as f64);
            let v = j as f64 / ((height - 1) as f64);
            let r = Ray{origin: origin, direction: lower_left_corner + horizontal * u + vertical * v - origin};
            write_color(r.color());
        }
    }
    eprintln!();
}

fn write_color(color: Color) {
    let c = color * 255.99;
    println!("{:.0} {:.0} {:.0}", c.x.floor(), c.y.floor(), c.z.floor())
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    fn color(self) -> Color {
        let t = hit_sphere(Point{x: 0.0, y: 0.0, z: -1.0}, 0.5, self);
        if (t > 0.0) {
            let n = (self.at(t) - Point{x: 0.0, y: 0.0, z: -1.0}).unit();
            Color{x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0} * 0.5
        } else {
            let u = self.direction.unit();
            let t = (u.y + 1.0) * 0.5;
            Color{x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + Color{x: 0.5, y: 0.7 ,z: 1.0} * t
        }
    }
}

fn hit_sphere(center: Point, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b =2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    if (d < 0.0) {
        -1.0
    } else {
        (-b - f64::sqrt(d)) / (2.0 * a)
    }
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
    }}