pub fn setup_heap() {
    // Allocate 72KB of heap memory for dynamic allocations
    esp_alloc::heap_allocator!(72 * 1024);
}
