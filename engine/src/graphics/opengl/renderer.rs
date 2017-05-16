pub extern crate term_painter;

use gl;
use sdl2;

use gl::types::*;

use math::matrix4::Matrix4;
use graphics::material::Material;
use graphics::opengl::object_ids::*;

use std;
use std::str;
use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::size_of;

use self::term_painter::ToStyle;
use self::term_painter::Color::{Green, Red};

pub struct OpenGLRenderer();

impl OpenGLRenderer {
	pub fn initialise(&self, video: &sdl2::VideoSubsystem) {
		println!("OpenGL Procs Found:");
		gl::load_with(|s| {
			let ptr = video.gl_get_proc_address(s);
			if !ptr.is_null() {
				println!("\t[{}] {}", Green.paint("Y"), s);
			} else {
				println!("\t[{}] {}", Red.paint("N"), s);
			}
			ptr as *const c_void
		});

		unsafe {
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

//			gl::Enable(gl::CULL_FACE);
			gl::Enable(gl::DEPTH_TEST);
		}
	}

	pub fn set_viewport(&self, width: i32, height: i32) {
		unsafe { gl::Viewport(0, 0, width, height) };
	}

	pub fn clear_colour(&self, red: f32, green: f32, blue: f32) {
		unsafe { gl::ClearColor(red, green, blue, 1.0); }
	}

	pub fn clear(&self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
	}

	// Draw

	pub fn draw(&self, material: &Material, texture: TextureID, perspective: &Matrix4, view: &Matrix4, model: &Matrix4) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0);

			gl::UseProgram(material.program.0);
			gl::BindTexture(gl::TEXTURE_2D, texture.0);
			{
				
				if let Ok(uniform) = self.get_uniform_location(material.program.0, "ourTexture") {
					gl::Uniform1i(uniform.0, 0);
				}

				gl::UniformMatrix4fv(self.get_uniform_location(material.program.0, "perspective").unwrap().0, 1, gl::FALSE, perspective.data.as_ptr());
				gl::UniformMatrix4fv(self.get_uniform_location(material.program.0, "view").unwrap().0, 1, gl::FALSE, view.data.as_ptr());
				gl::UniformMatrix4fv(self.get_uniform_location(material.program.0, "model").unwrap().0, 1, gl::FALSE, model.data.as_ptr());

				gl::BindVertexArray(material.vao.0);
				{
					gl::BindBuffer(gl::ARRAY_BUFFER, material.vbo.0);
					{
						gl::DrawArrays(gl::TRIANGLES, 0, material.vertices.len() as i32 / material.vertex_stride);
					}
					gl::BindBuffer(gl::ARRAY_BUFFER, 0);
				}
				gl::BindVertexArray(0);
			}
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}

	// VBO

	pub fn generate_vertex_buffer_object(&self, vertices: &Vec<f32>) -> VertexBufferObjectID {
		let mut vbo = VertexBufferObjectID(0);
		unsafe {
			gl::GenBuffers(1, &mut vbo.0);
		}
		self.bind_vertex_buffer_object(vbo, vertices);
		println!("Generating {:?}", vbo);
		return vbo;
	} 

	pub fn bind_vertex_buffer_object(&self, vbo: VertexBufferObjectID, vertices: &Vec<f32>) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0);
			{
				gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * vertices.len()) as isize ,
							   vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);
			}
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	// VAO

	pub fn generate_vertex_array_object(&self, vbo: VertexBufferObjectID) -> VertexArrayObjectID {
		let mut vao = VertexArrayObjectID(0);
		unsafe {
			gl::GenVertexArrays(1, &mut vao.0);
		}
		self.bind_vertex_array_object(vbo, vao);
		println!("Generating {:?}", vao);
		return vao;
	}

	pub fn bind_vertex_array_object(&self, vbo: VertexBufferObjectID, vao: VertexArrayObjectID) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0);
			gl::BindVertexArray(vao.0);
			{
				// Layout, Size, Type, Normalized, Stride, Offset
				let size = 5 * std::mem::size_of::<f32>() as i32;
				gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, std::ptr::null());
				gl::EnableVertexAttribArray(0);

				// Tex Coord
				gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size, (3 * size_of::<f32>()) as *const c_void);
				gl::EnableVertexAttribArray(1);
			}
			gl::BindVertexArray(0);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	// Textures

	pub fn generate_texture(&self, width: i32, height: i32, rgba_data: Vec<u8>) -> TextureID {
		let mut texture = TextureID(0);
		unsafe {
			gl::GenTextures(1, &mut texture.0);
			
			self.get_errors();

			gl::BindTexture(gl::TEXTURE_2D, texture.0);
			{
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // X wrapping
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); // Y wrapping

				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32); // Far away
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); // Close up
				gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, rgba_data.as_ptr() as *const c_void);
			}
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
		println!("Generating {:?}", texture);
		return texture;
	}

	pub fn get_errors(&self) {
		unsafe {
			loop {
				let code = gl::GetError();
				if code == gl::NO_ERROR {
					break;
				}
				println!("{}: Code ({})", Red.paint("GL_ERROR"), code);
			}
		}
	}

	// Shaders

	pub fn generate_shader_program(&self, vertex_shader_source: &str, fragment_shader_source: &str) -> ProgramID {
		let vertex_shader = self.compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
		let fragment_shader = self.compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
		return self.link_program(vertex_shader, fragment_shader);
	}

	fn compile_shader(&self, src: &str, shader_type: GLenum) -> ShaderID {
		let shader;
		unsafe {
			shader = gl::CreateShader(shader_type);

			// Attempt to compile the shader
			let c_str = CString::new(src.as_bytes()).unwrap();
			gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
			gl::CompileShader(shader);

			// Get the compile status
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint) {
				let mut len = 0;
				gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
				gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
				panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
			}
			println!("Compiled shader {}:\n{}", shader, Green.paint(src));
		}
		return ShaderID(shader);
	}

	fn link_program(&self, vs: ShaderID, fs: ShaderID) -> ProgramID {
		unsafe {
			let program = gl::CreateProgram();
			gl::AttachShader(program, vs.0);
			gl::AttachShader(program, fs.0);
			gl::LinkProgram(program);

			// Get the link status
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

			// Fail on error
			if status != (gl::TRUE as GLint) {
				let mut len: GLint = 0;
				gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
				gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
				panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
			}
			return ProgramID(program);
		}
	}

	fn get_uniform_location(&self, program: u32, name: &str) -> Result<UniformID, String> {
		let location;
		unsafe {
			location = gl::GetUniformLocation(program, CString::new(name).unwrap().as_ptr());
		}
		if location == -1 {
			return Err(format!("Could not find shader's {} uniform location for: {}", program, name));
		}
		return Ok(UniformID(location));
	}
}