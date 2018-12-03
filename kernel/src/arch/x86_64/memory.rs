use bit_allocator::BitAlloc;
use consts::KERNEL_OFFSET;
// Depends on kernel
use memory::{FRAME_ALLOCATOR, active_table};
use super::{BootInfo, MemoryRegionType};
use super::super::HEAP_ALLOCATOR;
use ucore_memory::PAGE_SIZE;
use ucore_memory::paging::*;

pub fn init(boot_info: &BootInfo) {
    assert_has_not_been_called!("memory::init must be called only once");
    init_frame_allocator(boot_info);
    init_device_vm_map();
    init_heap();
    info!("memory: init end");
}

/// Init FrameAllocator and insert all 'Usable' regions from BootInfo.
fn init_frame_allocator(boot_info: &BootInfo) {
    let mut ba = FRAME_ALLOCATOR.lock();
    for region in boot_info.memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable {
            ba.insert(region.range.start_frame_number as usize..region.range.end_frame_number as usize);
        }
    }
}

pub fn init_heap() {
    use consts::KERNEL_HEAP_SIZE;
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe { HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE); }
    info!("heap init end");
}


fn init_device_vm_map() {
    let mut page_table = active_table();
    // IOAPIC
    page_table.map(KERNEL_OFFSET + 0xfec00000, 0xfec00000).update();
    // LocalAPIC
    page_table.map(KERNEL_OFFSET + 0xfee00000, 0xfee00000).update();
}
