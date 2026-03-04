```rust
use std::vec::Vec;

/// A simple arena allocator for efficient memory management
pub struct ArenaAllocator<T> {
    items: Vec<Option<T>>,
    free_indices: Vec<usize>,
    count: usize,
}

impl<T> ArenaAllocator<T> {
    /// Creates a new arena allocator with initial capacity
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            free_indices: Vec::new(),
            count: 0,
        }
    }

    /// Creates a new arena allocator with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            free_indices: Vec::new(),
            count: 0,
        }
    }

    /// Allocates a new item in the arena and returns its index
    pub fn allocate(&mut self, item: T) -> usize {
        if let Some(index) = self.free_indices.pop() {
            // Reuse a previously freed slot
            self.items[index] = Some(item);
            self.count += 1;
            index
        } else {
            // Add a new slot
            self.items.push(Some(item));
            self.count += 1;
            self.items.len() - 1
        }
    }

    /// Deallocates an item by index
    pub fn deallocate(&mut self, index: usize) {
        if index < self.items.len() {
            if self.items[index].is_some() {
                self.items[index] = None;
                self.free_indices.push(index);
                self.count -= 1;
            }
        }
    }

    /// Gets a reference to an item by index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            self.items[index].as_ref()
        } else {
            None
        }
    }

    /// Gets a mutable reference to an item by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.items.len() {
            self.items[index].as_mut()
        } else {
            None
        }
    }

    /// Checks if an index is valid and allocated
    pub fn is_allocated(&self, index: usize) -> bool {
        if index < self.items.len() {
            self.items[index].is_some()
        } else {
            false
        }
    }

    /// Returns the number of allocated items
    pub fn count(&self) -> usize {
        self.count
    }

    /// Clears the arena, deallocating all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.free_indices.clear();
        self.count = 0;
    }
}
```