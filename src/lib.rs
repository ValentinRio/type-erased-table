#![feature(alloc_layout_extra)]

use std::{alloc::Layout, num::NonZeroUsize, ptr::NonNull};

pub struct Column {
    item_layout: Layout,
    capacity: usize,
    len: usize,
    data: NonNull<u8>
}

impl Column {

    pub unsafe fn new(
        item_layout: Layout,
        capacity: usize,
    ) -> Column {
        let align = NonZeroUsize::new(item_layout.align()).expect("alignment must be > 0");
        let data = unsafe { NonNull::new_unchecked(align.get() as *mut u8) };
        if item_layout.size() == 0 {
            Column {
                data,
                capacity: usize::MAX,
                len: 0,
                item_layout,
            }
        } else {
            let mut column = Column {
                data,
                capacity: 0,
                len: 0,
                item_layout,
            };
            column.reserve(capacity);
            column
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        let available_space = self.capacity - self.len;
        if available_space < additional {
            let increment = unsafe { NonZeroUsize::new_unchecked(additional - available_space) };
            self.grow(increment);
        }
    }

    fn grow(&mut self, increment: NonZeroUsize) {
        let new_capacity = self
            .capacity
            .checked_add(increment.get())
            .expect("capacity overflow");
        let (new_layout, _offset) = 
            Layout::repeat(&self.item_layout, new_capacity).expect("array layout should be valid");
        let new_data = if self.capacity == 0 {
            unsafe { std::alloc::alloc(new_layout) }
        } else {
            unsafe {
                let (array_layout, _offset) = 
                    Layout::repeat(&self.item_layout, self.capacity).expect("array layout should be valid");
                std::alloc::realloc(
                    self.data.as_ptr().as_mut().unwrap(),
                    array_layout,
                    new_layout.size(),
                )
            }
        };

        self.data = NonNull::new(new_data).expect("array layout should be valid");
        self.capacity = new_capacity;
    }

    pub unsafe fn initialize_unchecked(&mut self, index: usize, value: *const u8) {
        debug_assert!(index < self.len());
        let ptr = self.get_mut(index);
        std::ptr::copy_nonoverlapping::<u8>(value, ptr, self.item_layout.size());
    }

    pub unsafe fn push(&mut self, value: *const u8) {
        self.reserve(1);
        let index = self.len;
        self.len += 1;
        self.initialize_unchecked(index, value);
    }

    pub unsafe fn replace(&mut self, index: usize, value: *const u8) {
        let destination = unsafe { self.get_mut(index) };
        unsafe {
            std::ptr::copy_nonoverlapping::<u8>(
                value,
                destination,
                self.item_layout.size(),
            );
        }
    }

    pub unsafe fn remove(&mut self, index: usize) {
        let new_len = self.len - 1;
        let size = self.item_layout.size();
        // If item removed is last then just reduce length
        // TODO: Reduce allocated capacity
        if index != new_len {
            std::ptr::swap_nonoverlapping::<u8>(
                self.get_mut(index),
                self.get_mut(new_len),
                size,
            );
        }
        self.len = new_len;
    }

    pub unsafe fn get(&self, index: usize) -> *const u8 {
        let size = self.item_layout.size();
        unsafe { self.get_ptr().byte_add(index * size) }
    }

    pub unsafe fn get_mut(&mut self, index: usize) -> *mut u8 {
        let size = self.item_layout.size();
        unsafe { self.get_ptr_mut().byte_add(index * size) }
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn get_ptr_mut(&mut self) -> *mut u8 {
        self.data.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn layout(&self) -> Layout {
        self.item_layout
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn can_init_column_and_create_row() {
        let value: u32 = 2;

        let layout = Layout::for_value(&value);
        let mut column = unsafe { Column::new(layout, 1) };

        unsafe { column.push(ptr::addr_of!(value) as *const u8) };

        let row_ptr = unsafe { column.get(0) };
        let row_value: u32 = unsafe { (*row_ptr).into() };

        assert_eq!(value, row_value);
    }

    #[test]
    fn can_modify_raw() {
        let value: u32 = 2;

        let layout = Layout::for_value(&value);
        let mut column = unsafe { Column::new(layout, 1) };

        unsafe { column.push(ptr::addr_of!(value) as *const u8) };

        let modified_value = 3;

        unsafe { column.replace(0, ptr::addr_of!(modified_value) as *const u8) };

        let row_ptr = unsafe { column.get_mut(0) };
        let row_value: u32 = unsafe { (*row_ptr).into() };

        assert_eq!(modified_value, row_value);
    }

    #[test]
    fn can_remove_raw() {
        let value: u32 = 2;

        let layout = Layout::for_value(&value);
        let mut column = unsafe { Column::new(layout, 1) };

        unsafe { column.push(ptr::addr_of!(value) as *const u8) };

        unsafe { column.remove(0) };

        assert_eq!(0, column.len());
    }
}
