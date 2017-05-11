const SIZE: usize = 4 * 4;

use math::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Matrix4 {
	pub data: [f32; SIZE]
}

impl Matrix4 {
	pub fn new(data: [f32; SIZE]) -> Matrix4 {
		Matrix4 { data }
	}

	pub fn identity() -> Matrix4 {
		Matrix4::new([1.0, 0.0, 0.0, 0.0,
		              0.0, 1.0, 0.0, 0.0,
		              0.0, 0.0, 1.0, 0.0,
		              0.0, 0.0, 0.0, 1.0])
	}

	pub fn translation(a: Vector3) -> Matrix4 {
		Matrix4::new([1.0, 0.0, 0.0, 0.0,
		              0.0, 1.0, 0.0, 0.0,
		              0.0, 0.0, 1.0, 0.0,
		              a.x, a.y, a.z, 1.0])
	}

	pub fn scale(a: Vector3) -> Matrix4 {
		Matrix4::new([a.x, 0.0, 0.0, 0.0,
		              0.0, a.y, 0.0, 0.0,
		              0.0, 0.0, a.z, 0.0,
		              0.0, 0.0, 0.0, 1.0])
	}

	pub fn translate_and_scale(pos: Vector3, scale: Vector3) -> Matrix4 {
		Matrix4::new([scale.x,    0.0,        0.0,     0.0,
		              0.0,        scale.y,    0.0,     0.0,
		              0.0,        0.0,        scale.z, 0.0,
		              pos.x,      pos.y,      pos.z,   1.0])
	}

	pub fn ortho(right: f32, left: f32, top: f32, bottom: f32, far: f32, near: f32) -> Matrix4 {
		Matrix4::new([2.0 / (right - left),     0.0,                  0.0,                   0.0,
		              0.0,                      2.0 / (top - bottom), 0.0,                   0.0,
		              0.0,                      0.0,                  -(2.0 / (far - near)), 0.0,
		              -((right + left) / (right - left)), -((top + bottom) / (top - bottom)), -((far + near) / (far - near)), 1.0])
	}
}

#[test]
fn test_comparison() {
	let a = Matrix4::identity();
	let b = Matrix4::identity();
	assert_eq!(a, b);

	let mut c = b;
	c.data[0] = 3.0;
	assert!(a != c);
}

#[test]
fn test_copying_data() {
	let mut data = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
	let a = Matrix4::new(data);

	data[0] = -99.0;

	let b = Matrix4::new(data);
	assert!(a != b);
}