use std::cell::{Cell};

use seahash::hash;
thread_local!(pub static ENTITY_ID: Cell<usize> = Cell::new(0));

pub fn unique_id_from_optional_name(name: Option<String>) -> Entity {
    if name == None {
        let mut eid: usize = 0;

        ENTITY_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            let hashed = hash(("_auto_".to_string() + id.to_string().as_ref()).as_bytes());
            eid = hashed as usize;
        });
        Entity(eid)
    } else {
        Entity(hash(name.unwrap().as_bytes()) as usize)
    }
}
pub fn unique_id() -> Entity {
        let mut eid: usize = 0;

        ENTITY_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            let hashed = hash(("_auto_".to_string() + id.to_string().as_ref()).as_bytes());
            eid = hashed as usize;
        });
        Entity(eid)

}

#[derive(Hash, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Copy)]
pub struct Entity(pub usize);
impl Entity {
    pub fn new_from_string_id<U:Into<Option<String>>>(value:U) -> Self {
        unique_id_from_optional_name(value.into())
    }
    pub fn new() -> Self {
        unique_id()
    }
}

impl From<String> for Entity {
    fn from(id: String) -> Self {
        Entity(hash(id.as_bytes()) as usize)
    }
}
impl From<usize> for Entity {
    fn from(id: usize) -> Self {
        Entity(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Entity;

    #[test]
    fn it_works() {
        let bob = Entity::new_from_string_id("George".to_string());
        let harry = Entity::new_from_string_id("George".to_string());
        let joe = Entity::new();
        println!("{:?}",bob);
        println!("{:?}",harry);
        println!("{:?}",joe);

    }
}