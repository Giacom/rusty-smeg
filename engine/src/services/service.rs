use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceContainer {
    pub services: HashMap<TypeId, Box<Any>>
}

impl ServiceContainer {
	pub fn new() -> ServiceContainer {
		ServiceContainer { services: HashMap::new() }
	}

    pub fn set<T: Service + 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }
    
    pub fn get<T: Service + 'static>(&mut self) -> &mut T {
        let any_service = self.services.get_mut(&TypeId::of::<T>()).expect("Error when trying to retrieve service!");
        any_service.downcast_mut::<T>().expect("Unable to downcast service to its true type.")
    }
}

pub trait Service {
}