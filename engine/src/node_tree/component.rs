
pub trait Component {
	fn start(&mut self) { }
	fn update(&mut self) { }
	fn draw(&self) { }
}
