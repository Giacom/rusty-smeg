
pub extern crate rusty_smeg;

use rusty_smeg::app::App;

fn main() {
	App::new("Rusty App", 800, 600).run();
}