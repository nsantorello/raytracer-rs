#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64, 
    pub z: f64,
}

use std::num::Float;
use std::ops::{Add, Sub};

impl Vec3 {
	pub fn dot(&self, rhs: &Vec3) -> f64 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn cross(&self, rhs: &Vec3) -> Vec3 {
		Vec3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x
		}
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}

	pub fn normalize(&self) -> Vec3 {
		let len = self.length();
		match len {
			0f64 => Vec3 { x: self.x, y: self.y, z: self.z },
			_ => Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
		} 
	}
}

impl Add for Vec3 {
	type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}
