
use std::ops::{Add, Mul, Sub, AddAssign, SubAssign, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 { x, y, z }
	}

	pub fn zero() -> Vector3 {
		Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn one() -> Vector3 {
		Vector3 { x: 1.0, y: 1.0, z: 1.0 }
	}

}

impl Add for Vector3 {
	type Output = Vector3;

	fn add(self, rhs: Vector3) -> Vector3 {
		Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl AddAssign for Vector3 {
	fn add_assign(&mut self, rhs: Vector3) {
		*self = *self + rhs;
	}
}

impl Sub for Vector3 {
	type Output = Vector3;

	fn sub(self, rhs: Vector3) -> Vector3 {
		Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl SubAssign for Vector3 {
	fn sub_assign(&mut self, rhs: Vector3) {
		*self = *self - rhs;
	}
}

impl Mul<f32> for Vector3 {
	type Output = Vector3;

	fn mul(self, rhs: f32) -> Vector3 {
		Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl MulAssign<f32> for Vector3 {
	fn mul_assign(&mut self, rhs: f32) {
		*self = *self * rhs;
	}
}


#[test]
fn test_add() {
	let a = Vector3::one();
	let b = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(Vector3::new(2.0, 3.0, 4.0), a + b);
}

#[test]
fn test_add_assign() {
	let a = Vector3::one();
	let b = Vector3::new(1.0, 2.0, 3.0);
	let c = a;
	c += b;
	assert_eq!(Vector3::new(2.0, 3.0, 4.0), c);
}

#[test]
fn test_sub() {
	let a = Vector3::one();
	let b = Vector3::new(0.5, -0.5, 1.0);
	assert_eq!(Vector3::new(0.5, 1.5, 0.0), a - b);
}

#[test]
fn test_sub_assign() {
	let a = Vector3::one();
	let b = Vector3::new(0.5, -0.5, 1.0);
	let c = a;
	c -= b;
	assert_eq!(Vector3::new(0.5, 1.5, 0.0), c);
}

#[test]
fn test_multiply() {
	let a = Vector3::new(2.0, 3.0, 1.5);
	let b = 2.0;
	assert_eq!(Vector3::new(4.0, 6.0, 3.0), a * b);
}

#[test]
fn test_multiply_assign() {
	let a = Vector3::new(2.0, 3.0, 1.5);
	let b = 2.0;
	let c = a;
	c *= b;
	assert_eq!(Vector3::new(4.0, 6.0, 3.0), c);
}