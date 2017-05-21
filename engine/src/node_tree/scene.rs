use node_tree::node::{Node, NodeId};
use node_tree::component::{Component};

pub struct Scene {
	 nodes: Vec<Option<Node>>,
	 free_node_space: Vec<NodeId>,
}

impl Scene {
	pub fn new() -> Scene {
		Scene {
			nodes: vec![],
			free_node_space: vec![],
		}
	}

	/*
		Nodes
	*/

	pub fn new_node(&mut self) -> NodeId  {
		let mut id = NodeId(self.nodes.len());

		if self.free_node_space.len() > 0 {
			id = self.free_node_space.pop().unwrap();
			self.nodes[id.0] = Some(Node::new(id));
		} else {
			self.nodes.push(Some(Node::new(id)));
		}

		println!("Added node {:?}", id);
		return id;
	}

	pub fn get_node(&self, id: NodeId) -> &Node {
		self.nodes.get(id.0).unwrap().as_ref().unwrap()
	}
	
	pub fn get_node_as_mut(&mut self, id: NodeId) -> &mut Node {
		self.nodes.get_mut(id.0).unwrap().as_mut().unwrap()
	}

	pub fn remove_node(&mut self, id: NodeId) {
		if self.nodes[id.0].is_some() {
			self.nodes[id.0] = None;
			self.free_node_space.push(id);
			println!("Removed node {:?}", id)
		} else {
			println!("ERROR: Attempted to remove node {:?} that was not active.", id);
		}
	}

	/*
		Components
	*/

/*
	#[inline(always)]
	pub fn add_component_to_node<C: Component + 'static>(&mut self, node_id: NodeId, component: C) -> ComponentId {
		self.add_boxed_component(node_id, Box::new(component), TypeId::of::<C>())
	}

	fn add_boxed_component(&mut self, node_id: NodeId, component: Box<Any>, type_id: TypeId) -> ComponentId {
		let mut id = ComponentId(self.components.len(), type_id);

		if self.free_component_space.len() > 0 {
			id = self.free_component_space.pop().unwrap();
			self.components[id.0] = Some(component);
		} else {
			self.components.push(Some(component));
		}

		self.get_node_as_mut(node_id).components_as_mut().push(id);

		println!("Added component {:?}", id);
		return id;
	}

	#[inline]
	pub fn get_component_type_from_node<C: Component + 'static>(&mut self, node_id: NodeId) -> Option<&mut C> {
		let type_to_match = TypeId::of::<C>();
		match self.get_uncasted_component(node_id, type_to_match) {
			Some(component) => Some(component.downcast_mut::<C>().unwrap()),
			None => None
		}
	}

	#[inline]
	pub fn get_component_type_id_from_node<C: Component + 'static>(&self, node_id: NodeId) -> Option<ComponentId> {
		self.get_component_id_from_type(node_id, TypeId::of::<C>())
	}

	fn get_uncasted_component(&mut self, node_id: NodeId, type_to_match: TypeId) -> Option<&mut Box<Any>> {
		let component_id = self.get_component_id_from_type(node_id, type_to_match);
		match component_id {
			Some(id) => {
				let component = self.components.get_mut(id.0).unwrap().as_mut().unwrap();
				Some(component)
			},
			None => None,
		}
	}

	fn get_component_id_from_type(&self, node_id: NodeId, type_to_match: TypeId) -> Option<ComponentId> {
		let mut result = None;

		for i in 0..self.get_node(node_id).components().len() {
			let node_component = self.get_node(node_id).components()[i];
			let id = node_component.0;
			let type_id = node_component.1;
			if type_id == type_to_match && self.components[id].is_some() {
				result = Some(node_component);
				break;
			}
		}
		return result;
	}

	#[inline]
	pub fn remove_component_type_from_node<C: Component + 'static>(&mut self, node_id: NodeId) {
		match self.get_component_id_from_type(node_id, TypeId::of::<C>()) {
			Some(id) => self.remove_component_id_from_node(node_id, id),
			None => { println!("Unable to remove component type.") }
		}
	}

	pub fn remove_component_id_from_node(&mut self, node_id: NodeId, component_id: ComponentId) {
		self.get_node_as_mut(node_id).components_as_mut().retain(|&x| x.0 != component_id.0);
		self.remove_component(component_id);
	}

	fn remove_component(&mut self, component_id: ComponentId) {
		if self.components[component_id.0].is_some() {
			self.components[component_id.0] = None;
			self.free_component_space.push(component_id);
			println!("Removed component {:?}", component_id)
		} else {
			println!("ERROR: Attempted to remove component {:?} that was not active.", component_id);
		}
	}
	*/
}