use node_tree::component::Component;

pub struct SpriteRenderer {
	pub data: i32
}

impl SpriteRenderer {
	pub fn new(data: i32) -> SpriteRenderer {
		SpriteRenderer { data }
	}
}

impl Component for SpriteRenderer {
	fn start(&mut self) {
	}

	fn update(&mut self) {
		println!("Test! {0}", self.data);
		self.data += 1;
	}

	fn draw(&self) {
	}
}