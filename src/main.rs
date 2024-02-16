use std::ops::{Add, Sub, Mul, Div};

fn main() {
    let width: u64 = 256;
    let height: u64 = 256;

    println!("P3");
    println!("{:}", width);
    println!("{:}", height);
    println!("255");

    for y in 0..height {
        for x in 0..width {
            eprint!("\rx:{:5}, y:{:5}", x, y);

            let rgb = Vec3{x: y as f64 / ((width - 1) as f64), y: x as f64 / ((height - 1) as f64), z: 0.25};
            write_color(rgb);
        }
    }
    eprintln!();
}

fn write_color(color: Color) {
    let c = color * 255.99;
    println!("{:.0} {:.0} {:.0}", c.x, c.y, c.z)
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
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    fn dot(self, other: Self) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn unit(self) -> Self {
        self / self.norm()
    }
}