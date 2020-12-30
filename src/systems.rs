use crate::entities::{BitSet};
use crate::actions::Event;
use std::any::TypeId as TypeId;
use std::collections::{HashMap};

pub trait System {
    fn new() -> Self;
    fn handle_event(event: Event);
}


pub struct SystemManager<T:System> {
    pub signatures_list: HashMap<TypeId, BitSet>,
    pub systems_list: HashMap<TypeId, T>
}

impl <T: System> SystemManager<T> {

    pub fn new() -> SystemManager<T> {
        SystemManager {
            signatures_list: HashMap::new(),
            systems_list: HashMap::new()
        }
    }

    pub fn register_system(&self) -> T {
        let typeid = TypeId::of::<T>();
        assert!(self.systems_list.get(&typeid).unwrap() == self.systems_list.iter().last().unwrap().1, "SYSTEM ALREADY REGISTERED");
        let mut system = T::new();
        self.systems_list.insert(typeid, system);
        return system;

    }

    pub fn set_signature(&self, signature: BitSet){
        let typeid = TypeId::of::<T>();
        assert!(self.systems_list.get(&typeid).unwrap() != self.systems_list.iter().last().unwrap().1, "SYSTEM USED BEFORE REGISTERED");
        self.signatures_list.insert(typeid, signature);

    }

    /*
    pub fn destroy_entity(&self, entity: Entity) {
        for pair in self.systems_list.iter() {
            let mut system = *pair.1;
            system.entities_list.remove(entity);
        }
    }

    pub fn entity_signature_changed(&self, entity: Entity, signature: BitSet) {

        for pair in self.systems_list.iter() {
            let typeid = pair.0;
            let mut system = pair.1;
            let system_signature = self.signatures_list.get(typeid).unwrap();

            if (signature == system_signature) {
                system.entities_list.insert(entity);
            }
            else{
                system.entities_list.remove(entity);
            }
        }
    }
    */
}