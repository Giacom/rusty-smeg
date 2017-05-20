use std::rc::Rc;

use std::any::TypeId;
use node_tree::node::Node;
use services::service::ServiceContainer;
use graphics::opengl::renderer::OpenGLRenderer;

#[derive(Debug, Copy, Clone)]
// TODO: Add CheckSum to make sure that a component we retrieve is the same as we expected
pub struct ComponentId(pub usize, pub TypeId);

pub trait Component {
	fn start(&mut self, node: &Rc<Node>) { }
	fn update(&mut self, node: &Rc<Node>, services: &ServiceContainer) { }
	fn draw(&self, node: &Rc<Node>, renderer: &OpenGLRenderer) { }
}
