use std::collections::{BTreeSet, BTreeMap};
use crate::entity::Entity;
use crate::component::{Component, Storage, BTreeMapStorage};
use std::any::{Any, TypeId};
use std::fmt::Debug;

struct EntityStore {
    entities: BTreeSet<Entity>,
    components: BTreeMap<TypeId, Box<dyn Storage>>,
}


impl EntityStore {
    pub fn new() -> Self {
        EntityStore {
            entities: BTreeSet::new(),
            components: BTreeMap::new(),
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::new();
        let created = self.entities.insert(entity);
        entity
    }
    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }
    pub fn count(&self) -> usize {
        self.entities.len()
    }
    pub fn get_component<T: Component + 'static>(&mut self) -> &mut BTreeMapStorage<T> {

   self.components.get_mut(&TypeId::of::<T>()).unwrap().as_mut().downcast_mut::<BTreeMapStorage<T>>().unwrap()







    }
    pub fn register_component<T: Component + 'static>(&mut self) {
        let storage = TypeId::of::<T::Storage>();

        self.components.insert(TypeId::of::<T>(), Box::new(BTreeMapStorage::<T> { data: BTreeMap::<Entity, T>::new() }));

    }

    pub fn add_component_to_entity<T:Component + Clone+ Debug +'static>(&mut self, entity:Entity, mut component:Box<T>){
      //  let tim = self.get_component::<T>();
        let item = component.as_mut().clone();
        self.components.get_mut(&TypeId::of::<T>()).unwrap().as_mut().downcast_mut::<BTreeMapStorage<T>>().unwrap().data.insert(entity,item);
        for i in self.components.get_mut(&TypeId::of::<T>()).unwrap().as_mut().downcast_mut::<BTreeMapStorage<T>>().unwrap().data.iter() {
            println!("POS  {:?}",i.1)
        }

        //let bob = self.components.get(&TypeId::of::<T>()).unwrap().as_ref().get_data();

    }
    pub fn get_component_for_entity<T:Component + Clone+ Debug +'static>(&mut self, entity:Entity) -> & mut T{
        //  let tim = self.get_component::<T>();
        self.components.get_mut(&TypeId::of::<T>()).unwrap().as_mut().downcast_mut::<BTreeMapStorage<T>>().unwrap().data.get_mut(&entity).unwrap()


        //let bob = self.components.get(&TypeId::of::<T>()).unwrap().as_ref().get_data();

    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Entity;
    use crate::entity_store::EntityStore;
    use crate::component::Component;
    use std::any::Any;
    use std::collections::BTreeMap;

    #[test]
    fn entity_store_created() {
        let entity_store = &mut EntityStore::new();
        assert_eq!(entity_store.count(), 0);
    }

    #[test]
    fn register_component() {
        #[derive(Clone,Debug)]
        struct Pos {x:u32,y:u32};
        impl Component for Pos {
            type Storage = BTreeMap<Entity, Self>;
        };

        let mut entity_store =  EntityStore::new();
        entity_store.register_component::<Pos>();
        let test2= entity_store.create_entity();
        let test = entity_store.create_entity();
       // let mut comp = ;
        entity_store.add_component_to_entity::<Pos>(test,Box::new(Pos{x:1,y:2}));
        entity_store.add_component_to_entity::<Pos>(test,Box::new(Pos{x:1,y:99}));
        entity_store.add_component_to_entity::<Pos>(test,Box::new(Pos{x:1,y:2}));
        entity_store.add_component_to_entity::<Pos>(test2,Box::new(Pos{x:1,y:2}));

        let  bob =  entity_store.get_component_for_entity::<Pos>(test2);
        bob.x = 199;
      //  println!("{:?}",bob);

        drop(bob);
        entity_store.add_component_to_entity::<Pos>(test,Box::new(Pos{x:1,y:2}));

        println!("{:?}", entity_store.entities);
    }
}