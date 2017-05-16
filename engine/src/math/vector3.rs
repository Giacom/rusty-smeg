
use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 { x, y, z }
	}

	/// Returns (0, 0, 0)
	pub fn zero() -> Vector3 {
		Vector3::new(0.0, 0.0, 0.0)
	}

	/// Returns (1, 1, 1)
	pub fn one() -> Vector3 {
		Vector3::new(1.0, 1.0, 1.0)
	}

	/// Returns (0, 1, 0)
	pub fn up() -> Vector3 {
		Vector3::new(0.0, 1.0, 0.0)
	}

	/// Returns (1, 0, 0)
	pub fn right() -> Vector3 {
		Vector3::new(1.0, 0.0, 0.0)
	}

	/// Returns (0, 0, 1)
	pub fn forward() -> Vector3 {
		Vector3::new(0.0, 0.0, 1.0)
	}

	pub fn length(self) -> f32 {
		f32::sqrt(self.length_sqr())
	}

	pub fn length_sqr(self) -> f32 {
		self.x * self.x + self.y * self.y +  self.z * self.z
	}

	pub fn normalised(self) -> Vector3 {
		self / self.length()
	}

	pub fn dot(self, rhs: Vector3) -> f32 {
		 self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn cross(self, rhs: Vector3) -> Vector3 {
		Vector3::new(self.y * rhs.z - self.z * rhs.y,
		             self.z * rhs.x - self.x * rhs.z,
		             self.x * rhs.y - self.y * rhs.x)
	}
}

/* Operators */

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

impl Div<f32> for Vector3 {
	type Output = Vector3;

	fn div(self, rhs: f32) -> Vector3 {
		Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}

impl DivAssign<f32> for Vector3 {
	fn div_assign(&mut self, rhs: f32) {
		*self = *self / rhs;
	}
}

/* Function Tests */

#[test]
fn test_length() {
	assert_eq!(Vector3::new(4.0, 3.0, 0.0).length(), 5.0);
}

#[test]
fn test_square_length() {
	assert_eq!(Vector3::new(1.0, 3.0, 0.0).length_sqr(), 10.0);
}

#[test]
fn test_normalised() {
	assert_eq!(Vector3::new(2.0, 0.0, 0.0).normalised(), Vector3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_dot() {
	assert_eq!(Vector3::new(0.5, 0.5, 0.0).dot(Vector3::new(-0.5, -0.5, 0.0)), -0.5);
}

#[test]
fn test_cross() {
	assert_eq!(Vector3::new(3.0, -2.0, -2.0).cross(Vector3::new(-1.0, 0.0, 5.0)), Vector3::new(-10.0, -13.0, -2.0));
}

/* Operator Tests */

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
	let mut c = a;
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
	let mut c = a;
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
	let mut c = a;
	c *= b;
	assert_eq!(Vector3::new(4.0, 6.0, 3.0), c);
}