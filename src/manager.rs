use crate::entities::*;
use crate::components::*;
use crate::systems::*;


pub struct Coordinator {
    pub component_manager: Box<ComponentManager>,
    pub entity_manager: Box<EntityManager>,
    pub system_manager: Box<SystemManager>
}

pub impl Coordinator {

    fn new() -> Coordinator {
        Coordinator {
            component_manager: Box::new(ComponentManager::new()),
            entity_manager: Box::new(EntityManager::new()),
            system_manager: Box::new(SystemManager::new())
        }
    }

    fn create_entity(&self) -> Entity {
        return self.entity_manager.create_entity();
    }

    fn destroy_entity(&self, entity: Entity) {
        self.entity_manager.destroy_entity(entity);
        self.component_manager.destroy_entity(entity);
        self.system_manager.destroy_entity(entity);
    }

    fn register_component<T: Component>(&self) {
        self.component_manager.register_component<T>();
    }

    fn add_component<T>(&self, entity: Entity, component T) {
        self.component_manager.add_component<T>(entity, component);
        
        let mut signature = self.entity_manager.get_signature(entity);
        signature.set(self.component_manager.get_component_type<T>(), true);
        
        self.entity_manager(entity, signature);
        self.system_manager.entity_signature_changed(entity, signature);
    }

    fn remove_component(&self, entity: Entity) {
        self.component_manager.remove_component<T>(entity);

        let mut signature = self.entity_manager.get_signature(entity);
        signature.set(self.componenet_manager.get)
    }
}
/*

class Coordinator
{
	template<typename T>
	void RemoveComponent(Entity entity)
	{
		mComponentManager->RemoveComponent<T>(entity);

		auto signature = mEntityManager->GetSignature(entity);
		signature.set(mComponentManager->GetComponentType<T>(), false);
		mEntityManager->SetSignature(entity, signature);

		mSystemManager->EntitySignatureChanged(entity, signature);
	}

	template<typename T>
	T& GetComponent(Entity entity)
	{
		return mComponentManager->GetComponent<T>(entity);
	}

	template<typename T>
	ComponentType GetComponentType()
	{
		return mComponentManager->GetComponentType<T>();
	}


	// System methods
	template<typename T>
	std::shared_ptr<T> RegisterSystem()
	{
		return mSystemManager->RegisterSystem<T>();
	}

	template<typename T>
	void SetSystemSignature(Signature signature)
	{
		mSystemManager->SetSignature<T>(signature);
	}
*/