//! Memory pool utilities for optimizing string allocations.
//!
//! This module provides memory pool structures for efficient string
//! handling in high-throughput scenarios.

use std::fmt;

/// A memory pool for managing chunk allocations.
pub struct MemoryPool {
    chunk_size: usize,
    max_chunks: usize,
}

impl MemoryPool {
    /// Creates a new MemoryPool.
    ///
    /// # Arguments
    ///
    /// * `chunk_size` - Size of each chunk in bytes (minimum 64)
    /// * `max_chunks` - Maximum number of chunks (minimum 1)
    pub fn new(chunk_size: usize, max_chunks: usize) -> Self {
        MemoryPool {
            chunk_size: chunk_size.max(64),
            max_chunks: max_chunks.max(1),
        }
    }

    /// Returns the chunk size.
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Returns the maximum number of chunks.
    pub fn max_chunks(&self) -> usize {
        self.max_chunks
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new(4096, 16)
    }
}

impl fmt::Debug for MemoryPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryPool")
            .field("chunk_size", &self.chunk_size)
            .field("max_chunks", &self.max_chunks)
            .finish()
    }
}

/// A string backed by a pooled buffer.
pub struct PooledString {
    data: Vec<u8>,
}

impl PooledString {
    /// Creates a new empty PooledString.
    pub fn new() -> Self {
        PooledString { data: Vec::new() }
    }

    /// Creates a PooledString with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        PooledString {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Creates a PooledString from a String.
    pub fn from_string(s: String) -> Self {
        PooledString {
            data: s.into_bytes(),
        }
    }

    /// Returns the string as a &str.
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }

    /// Returns the string as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Appends a string to the buffer.
    pub fn push_str(&mut self, s: &str) {
        self.data.extend(s.as_bytes());
    }

    /// Appends a character to the buffer.
    pub fn push(&mut self, c: char) {
        self.data.extend(c.encode_utf8(&mut [0u8; 4]).as_bytes());
    }

    /// Returns the length of the string in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the string.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the capacity of the underlying buffer.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves additional capacity.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Truncates the string to the given length.
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }
}

impl Default for PooledString {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for PooledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PooledString")
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .finish()
    }
}

impl Clone for PooledString {
    fn clone(&self) -> Self {
        PooledString {
            data: self.data.clone(),
        }
    }
}

impl From<String> for PooledString {
    fn from(s: String) -> Self {
        PooledString::from_string(s)
    }
}

impl From<&str> for PooledString {
    fn from(s: &str) -> Self {
        PooledString::from_string(s.to_string())
    }
}

impl std::ops::Deref for PooledString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// A pool for managing multiple PooledStrings.
pub struct StringPool {
    strings: Vec<PooledString>,
    #[allow(dead_code)]
    max_size: usize,
}

impl StringPool {
    /// Creates a new StringPool with default max size of 1024.
    pub fn new() -> Self {
        StringPool {
            strings: Vec::new(),
            max_size: 1024,
        }
    }

    /// Creates a StringPool with the specified max size.
    pub fn with_max_size(max_size: usize) -> Self {
        StringPool {
            strings: Vec::new(),
            max_size,
        }
    }

    /// Stores a string in the pool.
    pub fn store(&mut self, s: &str) -> PooledString {
        PooledString::from_string(s.to_string())
    }

    /// Gets a string from the pool or inserts it if not present.
    pub fn get_or_insert(&mut self, s: &str) -> PooledString {
        self.store(s)
    }

    /// Clears all strings from the pool.
    pub fn clear(&mut self) {
        self.strings.clear();
    }

    /// Returns the number of strings in the pool.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Returns the total capacity of all strings in the pool.
    pub fn total_capacity(&self) -> usize {
        self.strings.iter().map(|s| s.capacity()).sum()
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StringPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StringPool")
            .field("len", &self.len())
            .field("total_capacity", &self.total_capacity())
            .finish()
    }
}

/// Allocation tracking utilities.
#[allow(dead_code)]
pub mod pool_alloc {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ALLOC_COUNT_MAX: AtomicUsize = AtomicUsize::new(0);

    /// Returns the current allocation count.
    pub fn get_alloc_count() -> usize {
        ALLOC_COUNT.load(Ordering::Relaxed)
    }

    /// Returns the maximum allocation count ever recorded.
    pub fn get_max_alloc_count() -> usize {
        ALLOC_COUNT_MAX.load(Ordering::Relaxed)
    }

    /// Resets the allocation counter to zero.
    pub fn reset_count() {
        ALLOC_COUNT.store(0, Ordering::Relaxed);
    }

    /// Records an allocation of the given size.
    pub fn record_allocation(size: usize) {
        let current = ALLOC_COUNT.fetch_add(size, Ordering::Relaxed);
        let max = ALLOC_COUNT_MAX.load(Ordering::Relaxed);
        if current > max {
            ALLOC_COUNT_MAX.store(current, Ordering::Relaxed);
        }
    }

    /// Records a deallocation of the given size.
    pub fn record_deallocation(size: usize) {
        ALLOC_COUNT.fetch_sub(size, Ordering::Relaxed);
    }
}
