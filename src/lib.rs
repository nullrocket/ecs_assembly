
use seahash::hash;
thread_local!(pub static COMPONENT_ID: Cell<usize> = Cell::new(0));

pub fn unique_id(name: Option<String>) -> Entity {
    if name == None {
        let mut eid: usize = 0;

        COMPONENT_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            let hashed = hash(("_auto_".to_string() + id.to_string().as_ref()).as_bytes());
            cid = hashed as usize;
        });
        Entity(eid)
    } else {
        Entity(hash(name.unwrap().as_bytes()) as usize)
    }
}
#[derive(Hash, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Copy)]
pub struct Entity(pub usize);
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
    #[test]
    fn it_works() {
        let bob = Entit(21);
        println!("{}",bob);
    }
}
