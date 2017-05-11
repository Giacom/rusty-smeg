use std::rc::Rc;

use node_tree::component::Component;

pub struct Node {
	parent: Option<Rc<Node>>,
	children: Vec<Rc<Node>>,
	components: Vec<Rc<Component>>
}