use super::compat::prelude::*;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use super::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use super::chunk::Chunk;
use super::collection::Collection;
use super::list::{ListBase, ListMutBase, List, ListMut};


#[derive(List, ListMut)]
pub struct Array<T, P : CapacityPolicy = DefaultCapacityPolicy> {
    size: isize,
    data: Chunk<T>,
    _policy: PhantomData<P>,
}


impl<T : Clone, P : CapacityPolicy> Array<T,P> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        let mut data = Chunk::new(P::initial(size));

        for i in 0..size {
            Chunk::write(&mut data, i, Clone::clone(&x));
        }

        Array {
            size: size,
            data: data,
            _policy: PhantomData,
        }
    }
}

impl<T, P : CapacityPolicy> Collection for Array<T,P> {
    fn size(&self) -> isize {
        self.size
    }
}

impl<T, P : CapacityPolicy> ListBase for Array<T,P> {
    type Element = T;

    fn get(&self, index: isize) -> Option<&T> {
        if (0 <= index) && (index < self.size) {
            Some(Chunk::get(&self.data, index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy> ListMutBase for Array<T,P> {
    fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if (0 <= index) && (index < self.size) {
            Some(Chunk::get_mut(&mut self.data, index))
        } else {
            None
        }
    }
}

impl<T, P : CapacityPolicy> Drop for Array<T,P>{
    fn drop(&mut self){
        for i in 0..self.size {
            Chunk::read(&mut self.data, i);
        }
    }
}


#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => (
        &mut $crate::array::Array::<_, $crate::capacity::DefaultCapacityPolicy>::new_from_elem($elem, $n)
    );
}