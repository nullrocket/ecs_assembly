use downcast::Any;
use std::collections::BTreeMap;
use crate::entity::Entity;
use std::ops::Deref;


pub trait Component : Any {
type Storage;
}


pub trait Storage :Any{
  fn count(&self) -> usize;


}
downcast!( Storage );


#[derive(Debug)]
pub struct BTreeMapStorage<T> {
  pub  data:BTreeMap<Entity,T>
}


impl <T :'static > BTreeMapStorage<T> {
  pub fn get_data(&self) -> &BTreeMap<Entity,T> {
    &self.data
  }
}

impl<T : 'static> Storage for BTreeMapStorage<T> {
fn count(&self) ->usize {

  self.data.len()
}





}

