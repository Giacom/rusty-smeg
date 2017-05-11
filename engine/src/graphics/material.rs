use graphics::opengl::object_ids::*;

pub struct Material {
	pub vbo: VertexBufferObjectID,
	pub vao: VertexArrayObjectID,
	pub ebo: ElementBufferObjectID,
	pub program: ProgramID,
	
	pub vertices: Vec<f32>,
	pub indices: Vec<u16>
}

impl Material {
	pub fn new(vbo: VertexBufferObjectID, vao: VertexArrayObjectID, ebo: ElementBufferObjectID,
	           program: ProgramID, vertices: Vec<f32>, indices: Vec<u16>) -> Material {
		Material { vbo, vao, ebo, program, vertices, indices }
	}
}