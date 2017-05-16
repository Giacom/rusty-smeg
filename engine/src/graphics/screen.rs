use std;
use gl;

use graphics::opengl::renderer::OpenGLRenderer;

use std::os::raw::c_void;
use glutin;
use glutin::*;

pub struct Screen {
	width: u32,
	height: u32,

	window: Window,
	renderer: OpenGLRenderer,
	events_loop: EventsLoop
}

impl Screen {
	pub fn new(title: &str, width: u32, height: u32) -> Screen {
		let events_loop = EventsLoop::new();
		let profile = glutin::GlProfile::Core;
		let window = WindowBuilder::new()
		                           .with_title(title)
		                           .with_dimensions(width, height)
		                           .with_vsync()
								   .with_gl_profile(profile)
								   .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
		                           .build(&events_loop).unwrap();

		unsafe {
			window.make_current()
		}.unwrap();
		// println!("OpenGL Profile: {:?}", video.gl_attr().context_profile());

		let renderer = OpenGLRenderer();
		renderer.initialise(&window);
		renderer.set_viewport(width as i32, height as i32);

		let version = unsafe {
			let data = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
			String::from_utf8(data).unwrap()
		};
		println!("OpenGL Version: {:?} {}", profile, version);
		
		Screen {
			width, height, window, renderer, events_loop
		}
	}

	pub fn screen_size(&self) -> (u32, u32) {
		(self.width, self.height)
	}
	
	pub fn renderer(&self) -> &OpenGLRenderer {
		&self.renderer
	}

	pub fn events_loop(&self) -> &EventsLoop {
		&self.events_loop
	}

	pub fn swap_buffer(&self) {
		self.window.swap_buffers().unwrap();
	}
}