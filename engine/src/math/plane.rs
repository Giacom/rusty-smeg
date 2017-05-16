use math::vector3::Vector3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Plane {
	pub normal: Vector3,
	pub distance: f32
}

impl Plane {
	pub fn normalise(self) -> Plane {
		let mut normalised = self;
		let scale = 1.0 / self.normal.length();
		normalised.normal *= scale;
		normalised.distance *= scale;
		return normalised;
	}
}