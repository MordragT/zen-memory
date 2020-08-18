use bitfield::bitfield;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use std::num::NonZeroUsize;

const GENERIC_HANDLE_MAX_SIZE_BITS: usize = mem::size_of::<u32>() * 8 * 2;
type Index = Option<std::num::NonZeroUsize>;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Handle {
    index: Index,
    generation: u32,
}

impl Handle {
    pub fn new() -> Handle {
        Handle {
            index: None,
            generation: 0,
        }
    }
    fn invalidate(&mut self) {
        self.index = None;
    }
    fn set_index(&mut self, index: usize) {
        self.index = NonZeroUsize::new(index);
    }
    fn get_index(&self) -> Index {
        self.index
    }
}

impl Ord for Handle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Allocator<T: Default>(HashMap<Handle, T>);

impl<T: Default> Allocator<T> {
    pub fn new() -> Allocator<T> {
        Allocator(HashMap::new())
    }
    pub fn create(&mut self) -> Result<Handle, &str> {
        let index = self.0.len();
        let mut handle = Handle::new();
        handle.set_index(index);
        match self.0.insert(handle, Default::default()) {
            Some(_) => Err("Handle was already in use"),
            None => Ok(handle),
        }
    }
    pub fn remove(&mut self, handle: &Handle) {
        match self.0.remove(handle) {
            Some(_) => (),
            None => println!("Freeing non allocated object."),
        }
    }
    pub fn get(&mut self, handle: &Handle) -> Result<&mut T, &str> {
        match self.0.get_mut(handle) {
            Some(val) => Ok(val),
            None => Err("Could not get object with specified handle"),
        }
    }
}
// /// FreeMap holds the differnt indices that point to a free memory location
// struct FreeMap<'a> {
//     map: HashMap<usize, &'a Handle>,
//     free_indices: Vec<usize>,
//     max_indices: usize,
// }

// impl<'a> FreeMap<'a> {
//     pub fn new(max_indices: usize) -> FreeMap<'a> {
//         let map: HashMap<usize, &Handle> = HashMap::new();
//         let free_indices = (0..max_indices).collect();
//         FreeMap {
//             map,
//             free_indices,
//             max_indices,
//         }
//     }
//     pub fn push(&mut self, handle: &mut Handle) -> Result<(), &str> {
//         let index = match self.free_indices.pop() {
//             Some(index) => index,
//             None => return Err("No free indices available."),
//         };
//         handle.set_index(index);
//         self.map.insert(index, handle);
//         Ok(())
//     }
//     pub fn remove(&mut self, handle: &mut Handle) -> Result<(), &str> {
//         let index = match handle.get_index() {
//             Some(index) => index.get(),
//             None => return Err("Handle had no valid index."),
//         };
//         handle.invalidate();
//         self.map.remove(&index);
//         self.free_indices.push(index);
//         Ok(())
//     }
// }

// /// I is number of Internal Handles that are allowed to be allocated
// pub struct StaticReferencedAllocator<'a, T: Default> {
//     handle_to_object_map: HashMap<Handle, T>,
//     free_map: FreeMap<'a>,
// }

// impl<'a, T: Default> StaticReferencedAllocator<'a, T> {
//     pub fn new(max_handles: usize) -> StaticReferencedAllocator<'a, T> {
//         let handle_to_object_map = HashMap::new();
//         let free_map = FreeMap::new(max_handles);
//         StaticReferencedAllocator {
//             handle_to_object_map,
//             free_map,
//         }
//     }
//     pub fn create_object(&mut self) -> Result<Handle, &str> {
//         let mut handle = Handle::new();
//         self.free_map.push(&mut handle).unwrap();
//         let t: T = Default::default();
//         self.handle_to_object_map.insert(handle, t);
//         //Ok(Box::new(handle))
//         Ok(handle)
//     }
//     pub fn remove_object(&mut self, handle: &mut Handle) -> Result<(), &str> {
//         match self.handle_to_object_map.remove(handle) {
//             Some(value) => (),
//             None => return Err("No value at key"),
//         }
//         self.free_map.remove(handle)?;
//         Ok(())
//     }
//     pub fn get_element(&self, handle: &Handle) -> Result<T, &str> {
//         match self.handle_to_object_map.get(handle) {
//             Some(object) => Ok(object),
//             None => Err("Handle has no pointer to data"),
//         }
//     }
// }
