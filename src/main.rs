pub extern crate sdl2;
pub extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod graphics;
mod math;

use graphics::screen::Screen;
use math::matrix4::Matrix4;

// Shader sources
static VS_SRC: &'static str =
	"#version 330 core\n\
	layout(location = 0) in vec3 position;\n
	uniform mat4 translate;
	void main() {\n\
		gl_Position = translate * vec4(position, 1.0);\n\
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

	let mut test = -1.0;
	let mut translation = Matrix4::translation(0.1, 0.1, 0.0);

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
		
		screen.draw(vbo, vao, ebo, program, &translation);

		screen.swap_buffer();
		test += 0.005;
		translation = Matrix4::translation(test, 0.1, 0.0);
	}
}
