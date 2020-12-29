use crate::components::*;
use bit_set::BitSet as BitSet;

pub const MAX_ENTITIES: u64 = 4096;
pub type Entity = u64;


pub struct Entity_Manager {

    available_entities: Vec<Entity>,
    entity_signatures: Vec<BitSet>,
    living_entity_count: u64


}

impl Entity_Manager {

    pub fn new() -> Entity_Manager {
        Entity_Manager {
            available_entities: Self::init_available_entities(),
            entity_signatures: Self::init_entity_signatures(),
            living_entity_count: 0
        }
    }

    pub fn init_available_entities() -> Vec<Entity> {
        let mut entities_list = Vec::new();
        for i in MAX_ENTITIES..0 {
            entities_list.push(i);
        }
        return entities_list;
    }

    pub fn init_entity_signatures() -> Vec<BitSet> {
        let mut signature_list = Vec::new();
        for _ in 0..MAX_ENTITIES {
            signature_list.push(BitSet::with_capacity(MAX_COMPONENTS as usize));
        }
        return signature_list;
    }

    pub fn create_entity(&mut self) -> Entity {

        assert!(self.living_entity_count < MAX_ENTITIES, "TOO MANY ENTITIES IN EXISTENCE");
        let id: Entity = self.available_entities.pop().unwrap();
        self.living_entity_count += 1;
        return id;
    }

    pub fn destroy_entity(&mut self, id: Entity) {

        assert!(id < MAX_ENTITIES, "ENTITY OUT OF RANGE");
        self.entity_signatures[id as usize].clear();
        self.available_entities.push(id);
        self.living_entity_count -= 1;
    }

    pub fn set_signature(&mut self, id: Entity, signature: BitSet) {
        std::mem::replace(&mut self.entity_signatures[id as usize], signature);
    }

    pub fn get_signature(&mut self, id: Entity) -> BitSet{
        return self.entity_signatures[id as usize].clone();
    }

}