use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicUsize, Ordering};

// Entity ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u32);

static NEXT_ENTITY_ID: AtomicUsize = AtomicUsize::new(0);

impl Entity {
    pub fn new() -> Self {
        Entity(NEXT_ENTITY_ID.fetch_add(1, Ordering::SeqCst) as u32)
    }
}

// Component trait
pub trait Component: Any + Send + Sync {}

// Implement Component for any type that meets the requirements
impl<T: Any + Send + Sync> Component for T {}

// Storage for a specific component type
trait ComponentStorage: Send + Sync {
    fn remove(&mut self, entity: Entity);
    fn has(&self, entity: Entity) -> bool;
}

// Concrete storage implementation
struct ConcreteComponentStorage<T: Component> {
    data: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorage for ConcreteComponentStorage<T> {
    fn remove(&mut self, entity: Entity) {
        self.data.remove(&entity);
    }

    fn has(&self, entity: Entity) -> bool {
        self.data.contains_key(&entity)
    }
}

impl<T: Component> ConcreteComponentStorage<T> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: Entity, component: T) {
        self.data.insert(entity, component);
    }

    fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(&entity)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    fn entities(&self) -> Vec<Entity> {
        self.data.keys().cloned().collect()
    }
}

// World that manages entities and components
pub struct EcsManager {
    component_storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl EcsManager {
    pub fn new() -> Self {
        Self {
            component_storages: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        Entity::new()
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        
        let storage = self.component_storages
            .entry(type_id)
            .or_insert_with(|| Box::new(ConcreteComponentStorage::<T>::new()));
        
        if let Some(concrete_storage) = storage.as_any_mut().downcast_mut::<ConcreteComponentStorage<T>>() {
            concrete_storage.insert(entity, component);
        } else {
            panic!("Failed to downcast storage to the correct type");
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.component_storages.get_mut(&type_id) {
            storage.remove(entity);
        }
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        
        self.component_storages
            .get(&type_id)
            .and_then(|storage| {
                storage.as_any().downcast_ref::<ConcreteComponentStorage<T>>()
            })
            .and_then(|concrete_storage| concrete_storage.get(entity))
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        
        self.component_storages
            .get_mut(&type_id)
            .and_then(|storage| {
                storage.as_any_mut().downcast_mut::<ConcreteComponentStorage<T>>()
            })
            .and_then(|concrete_storage| concrete_storage.get_mut(entity))
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        
        self.component_storages
            .get(&type_id)
            .map(|storage| storage.has(entity))
            .unwrap_or(false)
    }
}

// Helper trait to downcast the storage
trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: ComponentStorage + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl dyn ComponentStorage {
    pub fn as_any(&self) -> &dyn Any {
        self.as_any()
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_any_mut()
    }
}