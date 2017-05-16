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
		let window = WindowBuilder::new()
		                           .with_title(title)
		                           .with_dimensions(width, height)
		                           .with_vsync()
		                           .build(&events_loop).unwrap();
		{
			// let gl_attr = video.gl_attr();
			// gl_attr.set_context_major_version(3);
			// gl_attr.set_context_minor_version(3);
			// gl_attr.set_double_buffer(true);
			// gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		}

		unsafe {
			window.make_current()
		}.unwrap();


		// println!("OpenGL Context: {}.{}", video.gl_attr().context_major_version(), video.gl_attr().context_minor_version());
		// println!("OpenGL Profile: {:?}", video.gl_attr().context_profile());

		let renderer = OpenGLRenderer();
		renderer.initialise(&window);
		renderer.set_viewport(width as i32, height as i32);
		
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