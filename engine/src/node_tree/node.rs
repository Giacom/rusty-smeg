use std::rc::{Rc, Weak};
use std::cell::RefCell;
use node_tree::component::Component;

type WeakNode = Weak<RefCell<NodeData>>;
type StrongNode = Rc<RefCell<NodeData>>;
type StrongComponent = Rc<RefCell<Component>>;

pub struct Node {
	node: StrongNode
}

struct NodeData {
	pub parent: Option<WeakNode>,
	pub children: Vec<StrongNode>,
	pub components: Vec<StrongComponent>
}

impl Node {
	pub fn new_root() -> Node {
		Node {
			node: Rc::new(
				RefCell::new(
					NodeData {
						parent: None,
						children: vec![],
						components: vec![]
					}
				)
			)
		}
	}

	pub fn add_component<C: Component + 'static>(&mut self, component: C) {
		self.node.borrow_mut().components.push(Rc::new(RefCell::new(component)));
	}

	pub fn get_component(&mut self, index: usize) -> StrongComponent {
		self.node.as_ref().borrow().components[index].clone()
	}
}