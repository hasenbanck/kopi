/// Statistics about the heap.
#[repr(transparent)]
pub struct HeapStatistics(v8::HeapStatistics);

impl HeapStatistics {
    /// Creates a new [`HeapStatistics`].
    pub(crate) fn new(isolate: &mut v8::Isolate) -> Self {
        let mut statistics = v8::HeapStatistics::default();
        isolate.get_heap_statistics(&mut statistics);
        Self(statistics)
    }

    /// Number of bytes the engine has allocated for the heap.
    #[inline(always)]
    pub fn total_heap_size(&self) -> usize {
        self.0.total_heap_size()
    }

    /// Returns the number of bytes for compiled bytecode and JITed code.
    #[inline(always)]
    pub fn total_heap_size_executable(&self) -> usize {
        self.0.total_heap_size_executable()
    }

    /// Returns the number of bytes of committed memory.
    #[inline(always)]
    pub fn total_physical_size(&self) -> usize {
        self.0.total_physical_size()
    }

    /// Returns the currently available free heap space in bytes.
    #[inline(always)]
    pub fn total_available_size(&self) -> usize {
        self.0.total_available_size()
    }

    /// Returns the number of global handles.
    #[inline(always)]
    pub fn total_global_handles_size(&self) -> usize {
        self.0.total_global_handles_size()
    }

    /// Returns the number of currently used global handles.
    #[inline(always)]
    pub fn used_global_handles_size(&self) -> usize {
        self.0.used_global_handles_size()
    }

    /// Returns the currently used amount of used heap space in bytes.
    #[inline(always)]
    pub fn used_heap_size(&self) -> usize {
        self.0.used_heap_size()
    }

    /// Returns the maximum size of bytes the heap can grow to.
    #[inline(always)]
    pub fn heap_size_limit(&self) -> usize {
        self.0.heap_size_limit()
    }

    /// Returns the amount of allocated memory in bytes.
    #[inline(always)]
    pub fn malloced_memory(&self) -> usize {
        self.0.malloced_memory()
    }

    /// Returns the amount of external memory in bytes.
    #[inline(always)]
    pub fn external_memory(&self) -> usize {
        self.0.external_memory()
    }

    /// Returns the maximum of allocated memory in bytes.
    #[inline(always)]
    pub fn peak_malloced_memory(&self) -> usize {
        self.0.peak_malloced_memory()
    }

    /// Returns the number of native contexts.
    #[inline(always)]
    pub fn number_of_native_contexts(&self) -> usize {
        self.0.number_of_native_contexts()
    }

    /// Returns the number of detached contexts.
    #[inline(always)]
    pub fn number_of_detached_contexts(&self) -> usize {
        self.0.number_of_detached_contexts()
    }

    /// Returns `true` when the engine overwrites heap garbage with a bit pattern.
    #[inline(always)]
    pub fn does_zap_garbage(&self) -> usize {
        self.0.does_zap_garbage()
    }
}
