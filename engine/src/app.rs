
use gl;
use image;
use glutin;

use std::f32;
use image::GenericImage;

use node_tree::scene::Scene;
use graphics::material::Material;
use graphics::screen::Screen;
use math::matrix4::Matrix4;
use math::vector3::Vector3;

use services::service::ServiceContainer;
use services::time::Time;

use glutin::Event;
use glutin::Event::WindowEvent;
use glutin::VirtualKeyCode;

// Shader sources
static VS_SRC: &str = include_str!("../res/shader.vert");
static FS_SRC: &str = include_str!("../res/shader.frag");

// NOTE: Build for windows without console
// cargo rustc -- -Clink-args="-Wl,--subsystem,windows"

pub struct App {
	active_scene: Option<Scene>,
	screen: Screen,
	pub services: ServiceContainer
}

impl App {
	pub fn new(title: &str, width: u32, height: u32) -> App {
		let screen = Screen::new(title, width, height);
		let mut services = ServiceContainer::new();

		services.set(Time::new());
		
		App { active_scene: None, screen: screen, services: services }
	}

	pub fn set_scene(&mut self, scene: Scene) {
		self.active_scene = Some(scene);
	}

	pub fn run(&mut self) {

		let default_program =  self.screen.renderer().generate_shader_program(VS_SRC, FS_SRC);
		let sprite_material = {
			let vertex_data = vec![
				0.5, 0.5, 0.0, /* */ 1.0, 0.0,
				-0.5, 0.5, 0.0, /* */ 0.0, 0.0,
				-0.5, -0.5, 0.0, /* */ 0.0, 1.0,

				-0.5, -0.5, 0.0, /* */ 0.0, 1.0,
				0.5, -0.5, 0.0, /* */ 1.0, 1.0,
				0.5, 0.5, 0.0, /* */ 1.0, 0.0,
			];

			let vbo = self.screen.renderer().generate_vertex_buffer_object(&vertex_data);
			let vao = self.screen.renderer().generate_vertex_array_object(vbo);

			Material::new(vbo, vao, default_program, vertex_data, 5)
		};

		let box_material = {
			let vertex_data = vec![
				-0.5, -0.5, -0.5,  0.0, 0.0,
				0.5, -0.5, -0.5,  1.0, 0.0,
				0.5,  0.5, -0.5,  1.0, 1.0,
				0.5,  0.5, -0.5,  1.0, 1.0,
				-0.5,  0.5, -0.5,  0.0, 1.0,
				-0.5, -0.5, -0.5,  0.0, 0.0,

				-0.5, -0.5,  0.5,  0.0, 0.0,
				0.5, -0.5,  0.5,  1.0, 0.0,
				0.5,  0.5,  0.5,  1.0, 1.0,
				0.5,  0.5,  0.5,  1.0, 1.0,
				-0.5,  0.5,  0.5,  0.0, 1.0,
				-0.5, -0.5,  0.5,  0.0, 0.0,

				-0.5,  0.5,  0.5,  1.0, 0.0,
				-0.5,  0.5, -0.5,  1.0, 1.0,
				-0.5, -0.5, -0.5,  0.0, 1.0,
				-0.5, -0.5, -0.5,  0.0, 1.0,
				-0.5, -0.5,  0.5,  0.0, 0.0,
				-0.5,  0.5,  0.5,  1.0, 0.0,

				0.5,  0.5,  0.5,  1.0, 0.0,
				0.5,  0.5, -0.5,  1.0, 1.0,
				0.5, -0.5, -0.5,  0.0, 1.0,
				0.5, -0.5, -0.5,  0.0, 1.0,
				0.5, -0.5,  0.5,  0.0, 0.0,
				0.5,  0.5,  0.5,  1.0, 0.0,

				-0.5, -0.5, -0.5,  0.0, 1.0,
				0.5, -0.5, -0.5,  1.0, 1.0,
				0.5, -0.5,  0.5,  1.0, 0.0,
				0.5, -0.5,  0.5,  1.0, 0.0,
				-0.5, -0.5,  0.5,  0.0, 0.0,
				-0.5, -0.5, -0.5,  0.0, 1.0,

				-0.5,  0.5, -0.5,  0.0, 1.0,
				0.5,  0.5, -0.5,  1.0, 1.0,
				0.5,  0.5,  0.5,  1.0, 0.0,
				0.5,  0.5,  0.5,  1.0, 0.0,
				-0.5,  0.5,  0.5,  0.0, 0.0,
				-0.5,  0.5, -0.5,  0.0, 1.0
			];

			let vbo = self.screen.renderer().generate_vertex_buffer_object(&vertex_data);
			let vao = self.screen.renderer().generate_vertex_array_object(vbo);

			Material::new(vbo, vao, default_program, vertex_data, 5)
		};


		let sprite_texture = {
			let image = image::load_from_memory(include_bytes!("../res/duck.png")).unwrap();
			let image_buffer = image.to_rgba();
			let data = image_buffer.into_vec();
			let (image_width, image_height) = image.dimensions();

			self.screen.renderer().generate_texture(image_width as i32, image_height as i32, data)
		};

		let box_texture = {
			let image = image::load_from_memory(include_bytes!("../res/duck_opaque.jpg")).unwrap();
			let image_buffer = image.to_rgba();
			let data = image_buffer.into_vec();
			let (image_width, image_height) = image.dimensions();

			self.screen.renderer().generate_texture(image_width as i32, image_height as i32, data)
		};

		self.screen.renderer().clear_colour(0.39, 0.58, 0.92);

		let (screen_width, screen_height) = self.screen.screen_size();
		let screen_half = Vector3::new((screen_width / 2) as f32, (screen_height / 2) as f32, 0.0);

		let position = Vector3::zero();

		let perspective = Matrix4::ortho(screen_half.x, -screen_half.x, screen_half.y, -screen_half.y, 1000.0, 0.1);
		// let perspective = Matrix4::perspective(90.0, screen_width as f32 / screen_height as f32, 1000.0, 0.1);;

		let model_size = Vector3::new(32.0, 32.0, 32.0);
		
		let model = Matrix4::translate_and_scale(position, model_size);
		let model2 = Matrix4::translate_and_scale(Vector3::new(100.0, 50.0, -1.0), model_size);

		let mut camera_pos = Vector3::new(0.0, 0.0, 150.0);
		let mut camera_rot = Vector3::new(0.0, 0.0, 0.0);

		let mut view = Matrix4::translation(camera_pos);

		let cube_positions = {
			let mut cube_positions = vec![];
			let size = 2;
			for x in -size..size {
				for y in -size..size {
					for z in -size..size {
						cube_positions.push(Vector3::new(x as f32, y as f32, z as f32) * 32.0);
					}
				}
			}
			cube_positions
		};


		let mut running = true;
		while running {

			self.services.get::<Time>().ticks += 1;

			// Hacky for now
			view = Matrix4::translation_and_rotation(camera_pos, camera_rot);
			// println!("Pos: {:?} - Rot: {:?}", camera_pos, camera_rot);

			camera_rot += Vector3::new(0.0, 0.1, 0.0);

			self.screen.events_loop().poll_events(|event| {
				match event {
					WindowEvent { event: window_event, .. } => {
						match window_event {
							glutin::WindowEvent::Closed { .. } | glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
								running = false
							},
							// Event::KeyDown { keycode, .. } => {
							// 	let mut movement = Vector3::zero();
							// 	let mut rotation = Vector3::zero();
							// 	let sensitivity = 5.0;
							// 	match keycode {
							// 		Some(Keycode::W) => {
							// 			movement.y = -sensitivity;
							// 		},
							// 		Some(Keycode::S) => {
							// 			movement.y = sensitivity;
							// 		},
							// 		Some(Keycode::A) => {
							// 			movement.x = sensitivity;
							// 		},
							// 		Some(Keycode::D) => {
							// 			movement.x = -sensitivity;
							// 		},
							// 		Some(Keycode::Q) => {
							// 			rotation.y = -sensitivity;
							// 		},
							// 		Some(Keycode::E) => {
							// 			rotation.y = sensitivity;
							// 		},
							// 		Some(Keycode::LCtrl) => {
							// 			movement.z = sensitivity;
							// 		},
							// 		Some(Keycode::LShift) => {
							// 			movement.z = -sensitivity;
							// 		}
							// 		_ => {}
							// 	}
							// 	camera_pos += movement;
							// 	camera_rot += rotation;
							_ => { }
						}
					},
					_ => { }
				}
			});

			self.screen.renderer().clear();
			
			// for cube_position in &cube_positions {
			// 	let box_model = Matrix4::translate_and_scale(*cube_position, model_size);
			// 	self.screen.renderer().draw_material(&box_material, box_texture, &perspective, &view, &box_model);
			// }

			self.screen.renderer().draw_material(&sprite_material, sprite_texture, &perspective, &view, &model);
			self.screen.renderer().draw_material(&sprite_material, sprite_texture, &perspective, &view, &model2);

			self.screen.swap_buffer();
		}
	}
}