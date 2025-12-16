use crate::collection::base::Collectable;
use std::alloc::{Layout, alloc, dealloc, realloc};
use std::fmt::{Display, Formatter};
use std::ptr;

#[derive(Debug)]
pub struct VectorError;

impl Display for VectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector Error")
    }
}

pub struct NaiveVector<T> {
    ptr: *mut T, // Changed: Points to the heap memory
    capacity: usize,
    length: usize,
}

impl<T> NaiveVector<T> {
    pub fn new() -> Self {
        NaiveVector {
            ptr: ptr::null_mut(), // Start with a null pointer
            capacity: 0,
            length: 0,
        }
    }

    pub fn get_len(&self) -> usize {
        self.length
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for NaiveVector<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.ptr.is_null() {
            unsafe {
                // Calculate the memory layout to know how much to free
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

impl<T: Copy> Collectable<T, VectorError> for NaiveVector<T> {
    fn add(&mut self, element: T) -> Result<(), VectorError> {
        // 1. Check if we need to grow
        if self.length == self.capacity {
            let new_capacity = if self.capacity == 0 {
                2
            } else {
                self.capacity * 2
            };

            // Calculate memory layout for the new size
            let new_layout = Layout::array::<T>(new_capacity).map_err(|_| VectorError)?;

            let new_ptr = unsafe {
                if self.capacity == 0 {
                    // First allocation
                    alloc(new_layout)
                } else {
                    // Reallocation (resizing): This copies the old data to the new larger block automatically
                    let old_layout = Layout::array::<T>(self.capacity).map_err(|_| VectorError)?;
                    realloc(self.ptr as *mut u8, old_layout, new_layout.size())
                }
            };

            if new_ptr.is_null() {
                return Err(VectorError); // Allocation failed
            }

            self.ptr = new_ptr as *mut T;
            self.capacity = new_capacity;
        }

        unsafe {
            // ptr.add(i) calculates the address of the i-th element
            ptr::write(self.ptr.add(self.length), element);
        }

        self.length += 1;

        dbg!(
            "Added element. New Len: {}, Cap: {}",
            self.length,
            self.capacity
        );

        Ok(())
    }

    fn remove(&mut self, _identifier: T) -> Result<(), VectorError> {
        Ok(())
    }
}
