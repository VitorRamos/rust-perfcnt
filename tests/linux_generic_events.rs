#![feature(alloc, heap_api)]

extern crate perfcnt;
extern crate alloc;

use perfcnt::{PerfCounter, AbstractPerfCounter};
use perfcnt::linux::{SoftwareEventType, PerfCounterBuilderLinux, ReadFormat, CacheId, CacheOpId, CacheOpResultId, HardwareEventType};
use alloc::heap::{allocate};

#[test]
pub fn sample_event() {
    let mut pc: PerfCounter = PerfCounterBuilderLinux::from_cache_event(CacheId::L1D, CacheOpId::Read, CacheOpResultId::Miss)
        .set_sample_frequency(10000)
        .enable_mmap()
        .finish()
        .expect("Could not create counter");

        pc.start().expect("Can not start the counter");
        pc.stop().expect("Can not stop the counter");
        let res = pc.read().expect("Can not read the counter");

        println!("{:?}", res);
        assert!(res > 0);
}

#[test]
pub fn test_cache_events() {
    let mut pc: PerfCounter = PerfCounterBuilderLinux::from_cache_event(CacheId::L1D, CacheOpId::Read, CacheOpResultId::Miss)
        .finish()
        .expect("Could not create counter");

        pc.start().expect("Can not start the counter");
        pc.stop().expect("Can not stop the counter");
        let res = pc.read().expect("Can not read the counter");
        assert!(res > 0);
}

#[test]
pub fn test_hardware_counter() {
    let mut pc: PerfCounter = PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::CacheMisses)
        .exclude_kernel()
        .exclude_idle()
        .finish()
        .expect("Could not create counter");

        pc.reset().expect("Can not reset");
        pc.start().expect("Can not stop the counter");
        pc.stop().expect("Can not start the counter");

        let res = pc.read().expect("Can not read the counter");
        assert!(res < 100);
}


#[test]
pub fn test_software_events() {
    let mut pc: PerfCounter = PerfCounterBuilderLinux::from_software_event(SoftwareEventType::PageFaults)
        .exclude_idle()
        .exclude_kernel()
        .add_read_format(ReadFormat::TotalTimeEnabled)
        .add_read_format(ReadFormat::TotalTimeRunning)
        .add_read_format(ReadFormat::FormatId)
        .finish()
        .expect("Could not create counter");

    let size = 1024*1024*16;
    let page_size = 4096;
    let p: *mut u8 = unsafe {
        // Make sure the buffer is big enough such that it is going to be
        // mmaped and not coming from some pre-allocated buffers
        allocate(size, page_size)
    };

    pc.reset().expect("Can not reset");
    pc.start().expect("Can not start the counter");
    // Touch two pages:
    unsafe {
        std::ptr::write(p, 0x1);
        std::ptr::write(p.offset(((size/page_size/2)*page_size) as isize), 0x01);
    }
    pc.stop().expect("Can not stop the counter");

    // Should be ~= 2
    let res = pc.read_fd().expect("Can not read the counter");
    assert_eq!(res.value, 2);
}