pub extern crate sdl2;
pub extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod graphics;

use graphics::screen::Screen;

fn main() {

    let screen = Screen::new(800, 600);

    let mut green = 0;
    let mut event_pump = screen.event_pump();

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

        screen.swap_buffer();
    }
}
