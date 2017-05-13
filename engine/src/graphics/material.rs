use graphics::opengl::object_ids::*;

pub struct Material {
	pub vbo: VertexBufferObjectID,
	pub vao: VertexArrayObjectID,
	pub program: ProgramID,
	
	pub vertices: Vec<f32>,
	pub vertex_stride: i32
}

impl Material {
	pub fn new(vbo: VertexBufferObjectID, vao: VertexArrayObjectID,
	           program: ProgramID, vertices: Vec<f32>, vertex_stride: i32) -> Material {
		Material { vbo, vao, program, vertices, vertex_stride }
	}
}