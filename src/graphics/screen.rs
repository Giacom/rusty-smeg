use gl;
use sdl2;

use sdl2::{Sdl, EventPump};
use sdl2::video::{GLContext, Window};
use sdl2::VideoSubsystem;

use graphics::opengl::renderer::OpenGLRenderer;

use std::os::raw::c_void;

pub struct Screen {
	window: Window,
	video: VideoSubsystem,
	renderer: OpenGLRenderer,
	gl_context: GLContext,
	sdl_context: Sdl
}

impl Screen {
	pub fn new(title: &str, width: u32, height: u32) -> Screen {
		let sdl_context = sdl2::init().unwrap();
		let video = sdl_context.video().unwrap();

		let window = video.window(title, width, height).position_centered().opengl().build().unwrap();
		{
			let gl_attr = video.gl_attr();
			gl_attr.set_context_major_version(3);
			gl_attr.set_context_minor_version(3);
			gl_attr.set_double_buffer(true);
			gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		}

		let gl_context = window.gl_create_context().unwrap();
		window.gl_make_current(&gl_context).unwrap();

		println!("OpenGL Context: {}.{}", video.gl_attr().context_major_version(), video.gl_attr().context_minor_version());
		println!("OpenGL Profile: {:?}", video.gl_attr().context_profile());

		let renderer = OpenGLRenderer();
		renderer.initialise(&video);
		renderer.set_viewport(width as i32, height as i32);
		
		Screen {
			window, video, renderer, gl_context, sdl_context
		}
	}
	
	pub fn renderer(&self) -> &OpenGLRenderer {
		&self.renderer
	}

	pub fn event_pump(&self) -> EventPump {
		self.sdl_context.event_pump().unwrap()
	}

	pub fn swap_buffer(&self) {
		self.window.gl_swap_window();
	}
}