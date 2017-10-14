use node_tree::node::Node;

pub struct Scene {
	pub root_node: Option<Node>
}

impl Scene {
	pub fn new_empty() -> Scene {
		Scene { root_node: None }
	}

	pub fn set_root_node(&mut self, node: Option<Node>) {
		self.root_node = node;
	}

	pub fn get_root_node(&mut self) -> Option<&mut Node> {
		self.root_node.as_mut()
	}
}

