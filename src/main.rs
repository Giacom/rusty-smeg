pub extern crate sdl2;
pub extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod graphics;
mod math;

use graphics::screen::Screen;

// Shader sources
static VS_SRC: &'static str =
   "#version 330 core\n\
	in vec2 position;\n\
	void main() {\n\
	   gl_Position = vec4(position, 0.0, 1.0);\n\
	}";

static FS_SRC: &'static str =
   "#version 330 core\n\
	out vec4 out_color;\n\
	void main() {\n\
	   out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
	}";

fn main() {

	let vertex_data = vec![
		0.0,  0.5, 0.0,
		0.5, -0.5, 0.0,
		-0.5, -0.5, 0.0
	];

	let indices: Vec<u16> = vec![
		0, 1, 2
	];

	let screen = Screen::new(800, 600);

	let mut green = 0;
	let mut event_pump = screen.event_pump();

	let vbo = screen.generate_vertex_buffer_object(&vertex_data);
	let vao = screen.generate_vertex_array_object(vbo);
	let ebo = screen.generate_element_buffer_object(&indices);
	let program = screen.generate_shader_program(VS_SRC, FS_SRC);


	'main: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main;
				},
				_ => { }
			}
		}

		green = (green + 1) % (255);
		let float_green = green as f32 / (255.0);

		screen.clear_colour(0.0, float_green, 0.0);
		screen.clear();
		
		screen.draw(vbo, vao, ebo, program);

		screen.swap_buffer();
	}
}
