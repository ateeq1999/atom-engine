use std::fmt;
pub struct MemoryPool {
    chunk_size: usize,
    max_chunks: usize,
}

impl MemoryPool {
    pub fn new(chunk_size: usize, max_chunks: usize) -> Self {
        MemoryPool {
            chunk_size: chunk_size.max(64),
            max_chunks: max_chunks.max(1),
        }
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

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

pub struct PooledString {
    data: Vec<u8>,
}

impl PooledString {
    pub fn new() -> Self {
        PooledString { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PooledString {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn from_string(s: String) -> Self {
        PooledString {
            data: s.into_bytes(),
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn push_str(&mut self, s: &str) {
        self.data.extend(s.as_bytes());
    }

    pub fn push(&mut self, c: char) {
        self.data.extend(c.encode_utf8(&mut [0u8; 4]).as_bytes());
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

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

pub struct StringPool {
    strings: Vec<PooledString>,
    #[allow(dead_code)]
    max_size: usize,
}

impl StringPool {
    pub fn new() -> Self {
        StringPool {
            strings: Vec::new(),
            max_size: 1024,
        }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        StringPool {
            strings: Vec::new(),
            max_size,
        }
    }

    pub fn store(&mut self, s: &str) -> PooledString {
        PooledString::from_string(s.to_string())
    }

    pub fn get_or_insert(&mut self, s: &str) -> PooledString {
        self.store(s)
    }

    pub fn clear(&mut self) {
        self.strings.clear();
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

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

#[allow(dead_code)]
pub mod pool_alloc {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ALLOC_COUNT_MAX: AtomicUsize = AtomicUsize::new(0);

    pub fn get_alloc_count() -> usize {
        ALLOC_COUNT.load(Ordering::Relaxed)
    }

    pub fn get_max_alloc_count() -> usize {
        ALLOC_COUNT_MAX.load(Ordering::Relaxed)
    }

    pub fn reset_count() {
        ALLOC_COUNT.store(0, Ordering::Relaxed);
    }

    pub fn record_allocation(size: usize) {
        let current = ALLOC_COUNT.fetch_add(size, Ordering::Relaxed);
        let max = ALLOC_COUNT_MAX.load(Ordering::Relaxed);
        if current > max {
            ALLOC_COUNT_MAX.store(current, Ordering::Relaxed);
        }
    }

    pub fn record_deallocation(size: usize) {
        ALLOC_COUNT.fetch_sub(size, Ordering::Relaxed);
    }
}
