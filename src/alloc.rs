
use enumset::EnumSet;
use esp_alloc::{EspHeap, MemoryCapability};
use defmt::println;

#[unsafe(no_mangle)]
unsafe extern "Rust" fn _esp_alloc_alloc(
    _heap: &EspHeap,
    _caps: EnumSet<MemoryCapability>,
    ptr: usize,
    size: usize,
) {
    println!("Allocated {} bytes: {:x}", size, ptr);
}


#[unsafe(no_mangle)]
unsafe extern "Rust" fn _esp_alloc_dealloc(_heap: &EspHeap, ptr: usize, size: usize) {
    println!("Deallocated {} bytes: {:x}", size, ptr);
}