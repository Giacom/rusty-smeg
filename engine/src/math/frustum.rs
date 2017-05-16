use math::plane::Plane;
use math::matrix4::Matrix4;

const FRUSTUM_NEAR: usize = 0;
const FRUSTUM_FAR: usize = 1;
const FRUSTUM_LEFT: usize = 2;
const FRUSTUM_RIGHT: usize = 3;
const FRUSTUM_UP: usize = 4;
const FRUSTUM_DOWN: usize = 5;

// Defined by 6 planes enclosing the frustum, the normals face inward
#[derive(Default, Debug)]
pub struct Frustum {
	pub planes: [Plane; 6]
}

impl Frustum {
	pub fn new() -> Frustum {
		Frustum::default()
	}

	pub fn from_matrix(matrix: Matrix4) -> Frustum {
		let mut frustum = Frustum::new();
		frustum.planes[FRUSTUM_RIGHT].normal.x = matrix.get(0, 3) - matrix.get(0, 0);
		frustum.planes[FRUSTUM_RIGHT].normal.y = matrix.get(1, 3) - matrix.get(1, 0);
		frustum.planes[FRUSTUM_RIGHT].normal.z = matrix.get(2, 3) - matrix.get(2, 0);
		frustum.planes[FRUSTUM_RIGHT].distance = matrix.get(3, 3) - matrix.get(3, 0);
		
		frustum.planes[FRUSTUM_LEFT].normal.x = matrix.get(0, 3) + matrix.get(0, 0);
		frustum.planes[FRUSTUM_LEFT].normal.y = matrix.get(1, 3) + matrix.get(1, 0);
		frustum.planes[FRUSTUM_LEFT].normal.z = matrix.get(2, 3) + matrix.get(2, 0);
		frustum.planes[FRUSTUM_LEFT].distance = matrix.get(3, 3) + matrix.get(3, 0);

		frustum.planes[FRUSTUM_DOWN].normal.x = matrix.get(0, 3) + matrix.get(0, 1);
		frustum.planes[FRUSTUM_DOWN].normal.y = matrix.get(1, 3) + matrix.get(1, 1);
		frustum.planes[FRUSTUM_DOWN].normal.z = matrix.get(2, 3) + matrix.get(2, 1);
		frustum.planes[FRUSTUM_DOWN].distance = matrix.get(3, 3) + matrix.get(3, 1);

		frustum.planes[FRUSTUM_UP].normal.x = matrix.get(0, 3) - matrix.get(0, 1);
		frustum.planes[FRUSTUM_UP].normal.y = matrix.get(1, 3) - matrix.get(1, 1);
		frustum.planes[FRUSTUM_UP].normal.z = matrix.get(2, 3) - matrix.get(2, 1);
		frustum.planes[FRUSTUM_UP].distance = matrix.get(3, 3) - matrix.get(3, 1);

		frustum.planes[FRUSTUM_FAR].normal.x = matrix.get(0, 3) - matrix.get(0, 2);
		frustum.planes[FRUSTUM_FAR].normal.y = matrix.get(1, 3) - matrix.get(1, 2);
		frustum.planes[FRUSTUM_FAR].normal.z = matrix.get(2, 3) - matrix.get(2, 2);
		frustum.planes[FRUSTUM_FAR].distance = matrix.get(3, 3) - matrix.get(3, 2);

		frustum.planes[FRUSTUM_NEAR].normal.x = matrix.get(0, 3) + matrix.get(0, 2);
		frustum.planes[FRUSTUM_NEAR].normal.y = matrix.get(1, 3) + matrix.get(1, 2);
		frustum.planes[FRUSTUM_NEAR].normal.z = matrix.get(2, 3) + matrix.get(2, 2);
		frustum.planes[FRUSTUM_NEAR].distance = matrix.get(3, 3) + matrix.get(3, 2);

		for plane in &mut frustum.planes {
			*plane = plane.normalise();
		}

		return frustum;
	}
}