use std::rc::{Rc, Weak};

use node_tree::component::{Component, ComponentPair};
use graphics::opengl::renderer::OpenGLRenderer;
use services::service::ServiceContainer;
use std::any::{Any, TypeId};

#[derive(Debug, Copy, Clone, Default)]
pub struct NodeId(pub usize);

pub struct Node {
	id: NodeId,
	parent: Option<NodeId>,
	children: Vec<NodeId>,
	components: Vec<ComponentPair>
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


	#[inline]
	pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut C {
		self.add_boxed_component(Box::new(component), TypeId::of::<C>()).downcast_mut::<C>().unwrap()
	}

	fn add_boxed_component(&mut self, component: Box<Any>, type_id: TypeId) -> &mut Box<Any> {
		self.components.push(ComponentPair { component, type_id });
		println!("{:?}: Added component {:?}", self.id, type_id);
		return &mut self.components.last_mut().unwrap().component
	}

	#[inline]
	pub fn get_component<C: Component + 'static>(&mut self) -> Option<&C> {
		match self.get_boxed_component(TypeId::of::<C>()) {
			Some(boxed_component) => Some(boxed_component.downcast_ref::<C>().unwrap()),
			None => None
		}
	}

	#[inline]
	pub fn get_component_mut<C: Component + 'static>(&mut self) -> Option<&mut C> {
		match self.get_boxed_component(TypeId::of::<C>()) {
			Some(boxed_component) => Some(boxed_component.downcast_mut::<C>().unwrap()),
			None => None
		}
	}

	pub fn get_boxed_component(&mut self, type_id: TypeId) -> Option<&mut Box<Any>> {
		for i in 0..self.components.len() {
			if type_id == self.components[i].type_id {
				return Some(&mut self.components.last_mut().unwrap().component);
			}
		}
		return None;
	}

	pub fn remove_component_at(&mut self, index: usize) {
		println!("{:?}: Removing component at index {}", self.id, index);
		self.components.remove(index);
	}
}