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

            let r = y as f64 / ((width - 1) as f64) * 255.99;
            let g = x as f64 / ((height - 1) as f64) * 255.99;
            let b = 0.25 * 255.99;

            println!("{:.0} {:.0} {:.0}", r, g, b)
        }
    }
    eprintln!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

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