pub extern crate sdl2;
pub extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod graphics;

use graphics::screen::Screen;

// Vertex data
static VERTEX_DATA: [f32; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 330\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 330\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

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
