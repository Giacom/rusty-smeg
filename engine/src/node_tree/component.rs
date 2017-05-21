use std::rc::Rc;

use std::any::{Any, TypeId};
use node_tree::node::NodeId;
use services::service::ServiceContainer;
use graphics::opengl::renderer::OpenGLRenderer;

#[derive(Debug)]
pub struct ComponentPair {
	pub component: Box<Any>,
	pub type_id: TypeId
}

pub trait Component {
	fn start(&mut self, node: NodeId) { }
	fn update(&mut self, node: NodeId, services: &ServiceContainer) { }
	fn draw(&self, node: NodeId, renderer: &OpenGLRenderer) { }
}
