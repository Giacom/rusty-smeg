use gl;
use std;
use sdl2;

use sdl2::{Sdl, EventPump};
use sdl2::video::{GLContext, Window};
use sdl2::VideoSubsystem;

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
			ptr as *const std::os::raw::c_void
		});

		println!("OpenGL Context: {}.{}", video.gl_attr().context_major_version(), video.gl_attr().context_minor_version());
		println!("OpenGL Profile: {:?}", video.gl_attr().context_profile());
		
		Screen {
			window, video, gl_context, sdl_context
		}
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

	pub fn event_pump(&self) -> EventPump {
		self.sdl_context.event_pump().unwrap()
	}
}