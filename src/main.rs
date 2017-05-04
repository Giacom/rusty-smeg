pub extern crate sdl2;
pub extern crate gl;
pub extern crate image;

use std::path::Path;

use image::GenericImage;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod graphics;
mod math;

use graphics::screen::Screen;
use math::matrix4::Matrix4;

// Shader sources
static FS_SRC: &str =
"#version 330 core

in vec3 ourColor;
in vec2 TexCoord;

out vec4 color;

uniform sampler2D ourTexture;

void main()
{
	color = texture(ourTexture, TexCoord);
	color.rgb *= ourColor;
}";

static VS_SRC: &str =
"#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texCoord;

out vec3 ourColor;
out vec2 TexCoord;

uniform mat4 translate;

void main()
{
	gl_Position = translate * vec4(position.x, position.y, position.z, 1.0);
	TexCoord = texCoord;
	ourColor = color;
}";

fn main() {
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

	let screen = Screen::new("Rusty Smeg Demo", 800, 600);

	let mut event_pump = screen.event_pump();

	let vbo = screen.renderer().generate_vertex_buffer_object(&vertex_data);
	let vao = screen.renderer().generate_vertex_array_object(vbo);
	let ebo = screen.renderer().generate_element_buffer_object(&indices);
	let program = screen.renderer().generate_shader_program(VS_SRC, FS_SRC);

	let image = image::open(&Path::new("res/duck.png",)).unwrap();
	let image_buffer = image.to_rgba();
	let (width, height) = image.dimensions();

	let data = image_buffer.into_vec();
	let texture = screen.renderer().generate_texture(width as i32, height as i32, data);

	screen.renderer().clear_colour(0.39, 0.58, 0.92);

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
		
		screen.renderer().draw(vbo, vao, ebo, program, texture, indices.len() as i32, &Matrix4::identity());
		screen.renderer().draw(vbo, vao, ebo, program, texture, indices.len() as i32, &Matrix4::translation(0.2, 0.2, -0.5));

		screen.swap_buffer();
	}
}
