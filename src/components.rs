use crate::entities::*;
use std::collections::HashMap as HashMap;
use std::any::TypeId as TypeId;

pub const MAX_COMPONENTS: u64 = 1024;
pub type ComponentType = u16;

pub trait Component {
    fn make_default() -> Self;
}

pub struct ComponentStorage<T: Component + Copy> {

    component_array: [T; MAX_ENTITIES as usize],
    entity_index_map: HashMap<Entity, usize>,
    index_entity_map: HashMap<usize, Entity>,
    array_size: usize
}


impl<T: Component + Copy> ComponentStorage<T> {

    pub fn new() -> ComponentStorage<T> {
        ComponentStorage {
            component_array: [<T>::make_default(); MAX_ENTITIES as usize],
            entity_index_map: HashMap::new(),
            index_entity_map: HashMap::new(),
            array_size: 0
        }
    }

    pub fn insert_data(&mut self, entity: &Entity, component: T) {

        assert!(self.entity_index_map.get(entity).unwrap() == self.entity_index_map.iter().last().unwrap().1, "COMPONENT ADDED TO ENTITY MORE THAN ONCE");

        let new_index: usize = self.array_size;
        self.entity_index_map.insert(*entity, new_index);
        self.index_entity_map.insert(new_index, *entity);
        std::mem::replace(&mut self.component_array[new_index], component);
        self.array_size += 1;
    }

    pub fn remove_data(&mut self, entity: &Entity) {

        assert!(self.entity_index_map.get(entity).unwrap() != self.entity_index_map.iter().last().unwrap().1, "COMPONENT DOES NOT EXIST");

        // scope trickery to fuck with the borrow checker
        let removed_entity_index: usize;
        {
        removed_entity_index = *self.entity_index_map.get(entity).unwrap();
        }

        let last_entity_index = self.array_size - 1;

        self.component_array.swap(removed_entity_index, last_entity_index);

        let last_entity: Entity = *self.index_entity_map.get(&last_entity_index).unwrap();
        
        self.entity_index_map.insert(last_entity, removed_entity_index);
        self.index_entity_map.insert(removed_entity_index, last_entity);

        self.entity_index_map.remove(entity);
        self.index_entity_map.remove(&last_entity_index);
        self.array_size -= 1;

    }

    pub fn get_data(&mut self, entity: &Entity) -> &T {

        assert!(self.entity_index_map.get(entity).unwrap() != self.entity_index_map.iter().last().unwrap().1, "COMPONENT DOES NOT EXIST");
        return &self.component_array[*self.entity_index_map.get(entity).unwrap()];
    }

    pub fn entity_destroyed(&mut self, entity: &Entity) {

        if self.entity_index_map.get(entity).unwrap() != self.entity_index_map.iter().last().unwrap().1 {
            self.remove_data(entity);
        }
    }
}

/**
pub struct ComponentManager{

    component_types: HashMap<TypeId, ComponentType>,
    component_arrays: HashMap<TypeId, ComponentStorage>,
    next_component_type: ComponentType

}

impl ComponentManager{

    pub fn new() -> ComponentManager {
        ComponentManager {
            component_types: HashMap::new(),
            component_arrays: HashMap::new(),
            next_component_type: 0
        }

    }

    pub fn get_component_array<T>(&mut self) ->  &mut ComponentStorage<T> {
        let typeid = TypeId::of::<T>();
        assert!(self.component_types.get(&typeid).unwrap() != self.component_types.iter().last().unwrap().1, "COMPONENT NOT REGISTERED");
        return self.component_arrays.get_mut(&typeid).unwrap();
    }

    pub fn register_component<T: Component + Copy>(&mut self){
        let typeid = TypeId::of::<T>();
        assert!(self.component_types.get(&typeid).unwrap() == self.component_types.iter().last().unwrap().1, "COMPONENT TYPE ALREADY REGISTERED");
        self.component_arrays.insert(typeid, ComponentStorage::<T>::new());
        self.next_component_type += 1;
    }

    pub fn get_component_type<T: Component + Copy + 'static>(&mut self) -> ComponentType {
        let typeid = TypeId::of::<T>();
        assert!(self.component_types.get(&typeid).unwrap() != self.component_types.iter().last().unwrap().1, "COMPONENT TYPE ALREADY REGISTERED");
        return *self.component_types.get(&typeid).unwrap();
    }

    pub fn add_component<T: Component + Copy>(&mut self, entity: Entity, component: T) {
        self.get_component_array().insert_data(&entity, component);

    }

    pub fn remove_component<T: Component + Copy>(&mut self, entity: Entity) {
        self.get_component_array::<T>().remove_data(&entity);

    }

    pub fn get_component<T: Component + Copy>(&mut self, entity: Entity) -> &T {
        let component: &T = self.get_component_array().get_data(&entity);
        return component;

    }

    pub fn destroy_entity(&mut self, entity: Entity){
        for storage in self.component_arrays.iter_mut() {
            storage.1.entity_destroyed(&entity);
        }

    }

}
*/
pub struct CommentOut {}