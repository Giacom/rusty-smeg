pub extern crate sdl2;
pub extern crate gl;

use sdl2::video::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use gl::types::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();

    let window = sdl_video.window("rust", 800, 600).position_centered().opengl().build().unwrap();
    {
        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_major_version(3);
        gl_attr.set_context_minor_version(3);
        gl_attr.set_double_buffer(true);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    }

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    gl::load_with(|s| {
        let ptr = sdl_video.gl_get_proc_address(s);
        if !ptr.is_null() {
            println!("Loaded {}", s);
        } else {
            println!("Could not load {}", s);
        }
        ptr as *const std::os::raw::c_void
    });

    println!("OpenGL Context: {}.{}", sdl_video.gl_attr().context_major_version(), sdl_video.gl_attr().context_minor_version());
    println!("OpenGL Profile: {:?}", sdl_video.gl_attr().context_profile());

    let mut renderer = window.renderer().build().unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut green = 0;

    'main: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                _ => { }
            }
        }

        green = (green + 1) % (255 * 20);
        unsafe {
            gl::ClearColor(0.0, green as f32 / (255.0 * 20.0), 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.present();
    }
}
