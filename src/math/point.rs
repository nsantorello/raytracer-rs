#[derive(Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64, 
    pub z: f64,
}

use std::num::Float;
use std::ops::{Add, Sub};

impl Point3 {
	pub fn distance(&self, other: &Point3) -> f64 {
		self.distance_squared(other).sqrt()
	}

	pub fn distance_squared(&self, other: &Point3) -> f64 {
	    let dx = self.x - other.x;
	    let dy = self.y - other.y;
	    let dz = self.z - other.z;

    	return dx * dx + dy * dy + dz * dz;
	}
}

impl Add for Point3 {
	type Output = Point3;

    fn add(self, rhs: Point3) -> Point3 {
		Point3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl Sub for Point3 {
	type Output = Point3;

	fn sub(self, rhs: Point3) -> Point3 {
		Point3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}