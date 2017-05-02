use gl;
use std;
use sdl2;

use sdl2::{Sdl, EventPump};
use sdl2::video::{GLContext, Window};
use sdl2::VideoSubsystem;

use std::str;
use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::size_of;

use gl::types::*;

pub struct Screen {
	window: Window,
	video: VideoSubsystem,
	gl_context: GLContext,
	sdl_context: Sdl
}

impl Screen {
	pub fn new(width: u32, height: u32) -> Screen {
		let sdl_context = sdl2::init().unwrap();
		let video = sdl_context.video().unwrap();

		let window = video.window("rust", width, height).position_centered().opengl().build().unwrap();
		{
			let gl_attr = video.gl_attr();
			gl_attr.set_context_major_version(3);
			gl_attr.set_context_minor_version(3);
			gl_attr.set_double_buffer(true);
			gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		}

		let gl_context = window.gl_create_context().unwrap();
    	window.gl_make_current(&gl_context).unwrap();

		gl::load_with(|s| {
			let ptr = video.gl_get_proc_address(s);
			if !ptr.is_null() {
				println!("Loaded {}", s);
			} else {
				println!("Could not load {}", s);
			}
			ptr as *const c_void
		});

		println!("OpenGL Context: {}.{}", video.gl_attr().context_major_version(), video.gl_attr().context_minor_version());
		println!("OpenGL Profile: {:?}", video.gl_attr().context_profile());
		
		Screen {
			window, video, gl_context, sdl_context
		}
	}

	pub fn event_pump(&self) -> EventPump {
		self.sdl_context.event_pump().unwrap()
	}

	//
	// OpenGL
	//

	pub fn set_viewport(width: i32, height: i32) {
		unsafe { gl::Viewport(0, 0, width, height) };
	}

	pub fn clear_colour(&self, red: f32, green: f32, blue: f32) {
		unsafe { gl::ClearColor(red, green, blue, 1.0); }
	}

	pub fn clear(&self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
	}

	pub fn swap_buffer(&self) {
		self.window.gl_swap_window();
	}

	// VBO

	pub fn generate_vertex_buffer_object(&self, vertices: &Vec<f32>) -> u32 {
		let mut vbo = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
		}
		self.bind_vertex_buffer_object(vbo, vertices);
		println!("Generating vertex buffer object: {}", vbo);
		return vbo;
	} 

	pub fn bind_vertex_buffer_object(&self, vbo: u32, vertices: &Vec<f32>) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			{
				gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * vertices.len()) as isize ,
				               vertices.as_ptr() as *const std::os::raw::c_void, gl::STATIC_DRAW);
			}
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	// VAO

	pub fn generate_vertex_array_object(&self, vbo: u32) -> u32 {
		let mut vao = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
		}
		self.bind_vertex_array_object(vbo, vao);
		println!("Generating vertex array object: {}", vao);
		return vao;
	}

	pub fn bind_vertex_array_object(&self, vbo: u32, vao: u32) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BindVertexArray(vao);
			{
				// Layout, Size, Type, Normalized, Stride, Offset
				let size = 8 * std::mem::size_of::<f32>() as i32;
				gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, std::ptr::null());
				gl::EnableVertexAttribArray(0);

				// Colour
				gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, (3 * size_of::<f32>()) as *const c_void);
				gl::EnableVertexAttribArray(1);

				// Tex Coord
				gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, (6 * size_of::<f32>()) as *const c_void);
				gl::EnableVertexAttribArray(2);
			}
			gl::BindVertexArray(0);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	// EBO

	pub fn generate_element_buffer_object(&self, indices: &Vec<u16>) -> u32 {
		let mut ebo = 0;
		unsafe {
			gl::GenBuffers(1, &mut ebo);
		}
		self.bind_element_buffer_object(ebo, indices);
		println!("Generating element buffer: {}", ebo);
		return ebo;
	}  

	pub fn bind_element_buffer_object(&self, ebo: u32, indices: &Vec<u16>) {
		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
			{
				gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size_of::<u16>() * indices.len()) as isize,
				               indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
			}
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
		}
	}

	// Shaders

	pub fn generate_shader_program(&self, vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
		let vertex_shader = self.compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
		let fragment_shader = self.compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
		return self.link_program(vertex_shader, fragment_shader);
	}

	fn compile_shader(&self, src: &str, shader_type: GLenum) -> GLuint {
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
		}
		return shader
	}

	fn link_program(&self, vs: GLuint, fs: GLuint) -> GLuint {
		unsafe {
			let program = gl::CreateProgram();
			gl::AttachShader(program, vs);
			gl::AttachShader(program, fs);
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
			return program;
		}
	}
}