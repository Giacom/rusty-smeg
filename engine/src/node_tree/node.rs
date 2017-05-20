use std::rc::{Rc, Weak};

use node_tree::component::{Component, ComponentId};
use graphics::opengl::renderer::OpenGLRenderer;
use services::service::ServiceContainer;

#[derive(Debug, Copy, Clone, Default)]
pub struct NodeId(pub usize);

pub struct Node {
	id: NodeId,
	parent: Option<NodeId>,
	children: Vec<NodeId>,
	components: Vec<ComponentId>
}

impl Node {
	pub fn new(id: NodeId) -> Node {
		Node { 
			id: id,
			parent: None,
			children: vec![],
			components: vec![]
		}
	}

	pub fn components(&self) -> &Vec<ComponentId> {
		&self.components
	}

	pub fn components_as_mut(&mut self) -> &mut Vec<ComponentId> {
		&mut self.components
	}
}