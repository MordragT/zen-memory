use bitfield::bitfield;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use std::num::NonZeroUsize;

const GENERIC_HANDLE_MAX_SIZE_BITS: usize = mem::size_of::<u32>() * 8 * 2;
type Index = Option<std::num::NonZeroUsize>;

#[derive(Copy, Clone, Eq, PartialEq)]
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
    pub fn invalidate(&mut self) {
        self.index = None;
    }
    pub fn set_index(&mut self, index: usize) {
        self.index = NonZeroUsize::new(index);
    }
    pub fn get_index(&self) -> Index {
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
/// FreeMap holds the differnt indices that point to a free memory location
struct FreeMap {
    map: HashMap<usize, Box<Handle>>,
    free_indices: Vec<usize>,
    max_indices: usize,
}

impl FreeMap {
    pub fn new(max_indices: usize) -> FreeMap {
        let map: HashMap<usize, Box<Handle>> = HashMap::new();
        let free_indices = (0..max_indices).collect();
        // for (index, handle) in internal_handles.iter().enumerate() {
        //     if index == max_indices {
        //         break; // TODO error ausspucken ?
        //     }
        //     if let Some(handle) = handle {
        //         map.insert(index, Box::new(*handle));
        //     }
        // }

        FreeMap {
            map,
            free_indices,
            max_indices,
        }
    }
    pub fn push(&mut self, handle: Box<Handle>) -> Result<(), &str> {
        let index = match self.free_indices.pop() {
            Some(index) => index,
            None => return Err("No free indices available."),
        };
        handle.set_index(index);
        self.map.insert(index, handle);
        Ok(())
    }
    pub fn remove(&mut self, handle: Box<Handle>) -> Result<(), &str> {
        let index = handle.get_index().get();
        self.map.remove(index);
        self.free_indices.push(index);
        OK(())
    } 
}

/// I is number of Internal Handles that are allowed to be allocated
pub struct StaticReferencedAllocator<T: Default> {
    handle_to_object_map: HashMap<Handle, T>,
    free_map: FreeMap,
}

impl<T> StaticReferencedAllocator<T> {
    pub fn new(max_handles: usize) -> StaticReferencedAllocator<T> {
        let handle_to_object_map = vec![];k
        let free_map = FreeMap::new(max_handles);
        StaticReferencedAllocator {
            handle_to_object_map,
            free_map,
        }
    }
    pub fn create_object(&mut self) -> Result<Box<Handle>, &str> {
        let mut handle = Box::new(Handle::new());
        self.free_map.push(handle).unwrap();
        let t: T = Default::default();
        self.handle_to_object_map.insert(handle, t);
        handle
    }
    pub fn remove_object(&mut self, handle: Box<Handle>) -> Result<(), &str> {
        self.handle_to_object_map.remove(handle)?;
        self.free_map.remove(handle)?;
    }
    pub fn get_element(&self, handle: &Handle) -> Result<T, &str> {
        match self.handle_to_object_map.get(handle) {
            Some(object) => Ok(object),
            None => Err("Handle has no pointer to data")
        }
    }
}
