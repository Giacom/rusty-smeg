
use std::path::Path;

use gl;
use image;

use image::GenericImage;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use graphics::screen::Screen;
use math::matrix4::Matrix4;
use math::vector3::Vector3;

// Shader sources
static VS_SRC: &str = include_str!("../res/shader.vert");
static FS_SRC: &str = include_str!("../res/shader.frag");

// NOTE: Build for windows without console
// cargo rustc -- -Clink-args="-Wl,--subsystem,windows"

pub fn run() {
	let vertex_data = vec![
		0.5, 0.5, 0.0, /* */ 1.0, 1.0, 1.0, /* */ 1.0, 0.0, // Bottom Right
		-0.5, 0.5, 0.0, /* */ 1.0, 1.0, 1.0, /* */ 0.0, 0.0, // Top Right
		-0.5, -0.5, 0.0, /* */ 1.0, 1.0, 1.0, /* */ 0.0, 1.0, // Top Left
		0.5, -0.5, 0.0, /* */ 1.0, 1.0, 1.0, /* */ 1.0, 1.0, // Bottom Left
	];

	let indices: Vec<gl::types::GLushort> = vec![
		0, 1, 2,
		2, 3, 0
	];

	let screen_width = 800;
	let screen_height = 600;
	let screen = Screen::new("Rusty Smeg Demo", screen_width, screen_height);

	let mut event_pump = screen.event_pump();

	let vbo = screen.renderer().generate_vertex_buffer_object(&vertex_data);
	let vao = screen.renderer().generate_vertex_array_object(vbo);
	let ebo = screen.renderer().generate_element_buffer_object(&indices);
	let program = screen.renderer().generate_shader_program(VS_SRC, FS_SRC);

	let image = image::load_from_memory(include_bytes!("../res/duck.png")).unwrap();
	let image_buffer = image.to_rgba();
	let (width, height) = image.dimensions();

	let data = image_buffer.into_vec();
	let texture = screen.renderer().generate_texture(width as i32, height as i32, data);

	screen.renderer().clear_colour(0.39, 0.58, 0.92);

	let screen_half = Vector3::new((screen_width / 2) as f32, (screen_height / 2) as f32, 0.0);

	let position = Vector3::zero();

	let perspective = Matrix4::ortho(screen_half.x, -screen_half.x, screen_half.y, -screen_half.y, 1000.0, -1000.0);
	let model_size = Vector3::new(256.0, 256.0, 1.0);
	
	let model = Matrix4::translate_and_scale(position, model_size);
	let model2 = Matrix4::translate_and_scale(Vector3::new(50.0, 50.0, 1.0), model_size);

	let view = Matrix4::identity();

	'main: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main;
				},
				_ => { }
			}
		}

		screen.renderer().clear();
		
		screen.renderer().draw(vbo, vao, ebo, program, texture, indices.len() as i32, &perspective, &view, &model);
		screen.renderer().draw(vbo, vao, ebo, program, texture, indices.len() as i32, &perspective, &view, &model2);

		screen.swap_buffer();
	}
}