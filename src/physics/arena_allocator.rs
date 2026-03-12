use std::vec::Vec;

/// A simple arena allocator for efficient memory management with generation tracking to prevent use-after-free bugs
pub struct ArenaAllocator<T> {
    items: Vec<Option<T>>,
    free_indices: Vec<usize>,
    generations: Vec<u64>, // Generation counter for each slot to detect use-after-free
    count: usize,
}

impl<T> ArenaAllocator<T> {
    /// Creates a new arena allocator with initial capacity
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            free_indices: Vec::new(),
            generations: Vec::new(),
            count: 0,
        }
    }

    /// Creates a new arena allocator with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            free_indices: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            count: 0,
        }
    }

    /// Allocates a new item in the arena and returns its index
    pub fn allocate(&mut self, item: T) -> usize {
        if let Some(index) = self.free_indices.pop() {
            // Reuse a previously freed slot - increment generation to invalidate old references
            self.generations[index] = self.generations[index].wrapping_add(1);
            self.items[index] = Some(item);
            self.count += 1;
            index
        } else {
            // Add a new slot
            let index = self.items.len();
            self.items.push(Some(item));
            self.generations.push(0);
            self.count += 1;
            index
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

    /// Gets a reference to an item by index with generation check
    pub fn get(&self, index: usize, generation: u64) -> Option<&T> {
        if index < self.items.len() && self.generations.get(index) == Some(&generation) {
            self.items[index].as_ref()
        } else {
            None
        }
    }

    /// Gets a mutable reference to an item by index with generation check
    pub fn get_mut(&mut self, index: usize, generation: u64) -> Option<&mut T> {
        if index < self.items.len() && self.generations.get(index) == Some(&generation) {
            self.items[index].as_mut()
        } else {
            None
        }
    }

    /// Gets a reference without generation check (legacy compatibility - UNSAFE)
    pub fn get_unchecked(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            self.items[index].as_ref()
        } else {
            None
        }
    }

    /// Gets a mutable reference without generation check (legacy compatibility - UNSAFE)
    pub fn get_mut_unchecked(&mut self, index: usize) -> Option<&mut T> {
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

    /// Returns the generation for a given index
    pub fn get_generation(&self, index: usize) -> Option<u64> {
        self.generations.get(index).copied()
    }

    /// Clears the arena, deallocating all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.free_indices.clear();
        self.generations.clear();
        self.count = 0;
    }

    /// Returns capacity hint for pre-allocation
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }
}

impl<T> Default for ArenaAllocator<T> {
    fn default() -> Self {
        Self::new()
    }
}