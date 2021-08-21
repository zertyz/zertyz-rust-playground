// TODO this crate to be renamed to 'syntax-semantics-spikes'

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::fmt::{Display, Formatter};

fn main() {
    eprintln!("##################################");
    eprintln!("## RUST SYNTAX/SEMANTICS SPIKES ##");
    eprintln!("##################################");
    eprintln!();
//    match_lifetime_trick();
//    match_with_additional_case_conditions();
    custom_allocator();
}

fn match_with_additional_case_conditions() {
    let values = [
        Some(1),
        Some(2),
        Some(3),
        None
    ];

    let even_mapped_values: Vec<bool> = values.iter()
        .map(|optional_val| match optional_val {
            Some(val) if val % 2 == 0 => true,
            _ => false,
        })
        .collect();
    println!("Some values:            {:?}", values);
    println!("Trickily matched evens: {:?}", even_mapped_values);
}

fn match_lifetime_trick() {

    eprintln!("## match_lifetime_trick: here we demonstrate how using 'match' may elongate statements for the purposes of lifetimes");
    eprintln!("## more info: https://stackoverflow.com/questions/48732263/why-is-rusts-assert-eq-implemented-using-a-match/48732525#48732525");

    fn f(x: &u32) -> &u32 {
        &x
    }
    fn g() -> u32 {
        10
    }
    // the following line fails with "temporary value is freed at the end of this statement" for '&g()' as of rust 1.53
    //let y = f(&g());
    //println!("y = {}", y);

    // trick to make the statement last longer
    match &g() {
        x => println!("y = {}", f(x)),
    };
}

pub struct CustomAllocatorStatistics<NumericType> {
    pub allocations_count:           NumericType,
    pub deallocations_count:         NumericType,
    pub zeroed_allocations_count:    NumericType,
    pub reallocations_count:         NumericType,
    pub allocated_bytes:             NumericType,
    pub deallocated_bytes:           NumericType,
    pub zeroed_allocated_bytes:      NumericType,
    pub reallocated_originals_bytes: NumericType,
    pub reallocated_news_bytes:      NumericType,
}
impl<NumericType> CustomAllocatorStatistics<NumericType> {
    fn fmt(&self, statistics: &CustomAllocatorStatistics<usize>, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{counts: {{allocations: {}, deallocations: {}, zeroed_allocations: {}, reallocations: {}}}, bytes: {{allocated: {}, deallocated: {}, zeroed: {}, reallocated: {{originals: {}, news: {}}}}}}}",
               statistics.allocations_count,
               statistics.deallocations_count,
               statistics.zeroed_allocations_count,
               statistics.reallocations_count,
               statistics.allocated_bytes,
               statistics.deallocated_bytes,
               statistics.zeroed_allocated_bytes,
               statistics.reallocated_originals_bytes,
               statistics.reallocated_news_bytes)
    }
}
impl Display for CustomAllocatorStatistics<AtomicUsize> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(&CustomAllocatorStatistics::<usize> {
            allocations_count:           self.allocations_count.          load(Ordering::Relaxed),
            deallocations_count:         self.deallocations_count.        load(Ordering::Relaxed),
            zeroed_allocations_count:    self.zeroed_allocations_count.   load(Ordering::Relaxed),
            reallocations_count:         self.reallocations_count.        load(Ordering::Relaxed),
            allocated_bytes:             self.allocated_bytes.            load(Ordering::Relaxed),
            deallocated_bytes:           self.deallocated_bytes.          load(Ordering::Relaxed),
            zeroed_allocated_bytes:      self.zeroed_allocated_bytes.     load(Ordering::Relaxed),
            reallocated_originals_bytes: self.reallocated_originals_bytes.load(Ordering::Relaxed),
            reallocated_news_bytes:      self.reallocated_news_bytes.     load(Ordering::Relaxed)}, f)
    }
}
impl Display for CustomAllocatorStatistics<usize> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(self, f)
    }
}

//#[derive(Debug)]
pub struct CustomAllocator<'a> {
    system_allocator: &'a System,
    statistics: CustomAllocatorStatistics<AtomicUsize>,
}
impl<'a> CustomAllocator<'a> {
    pub const fn new(system_allocator: &'a System) -> Self {
        Self {
            system_allocator,
            statistics: CustomAllocatorStatistics {
                allocations_count:           AtomicUsize::new(0),
                deallocations_count:         AtomicUsize::new(0),
                zeroed_allocations_count:    AtomicUsize::new(0),
                reallocations_count:         AtomicUsize::new(0),
                allocated_bytes:             AtomicUsize::new(0),
                deallocated_bytes:           AtomicUsize::new(0),
                zeroed_allocated_bytes:      AtomicUsize::new(0),
                reallocated_originals_bytes: AtomicUsize::new(0),
                reallocated_news_bytes:      AtomicUsize::new(0)},
        }
    }
    pub fn reset(&self) {
        self.statistics.allocations_count          .store(0, Ordering::Relaxed);
        self.statistics.deallocations_count        .store(0, Ordering::Relaxed);
        self.statistics.zeroed_allocations_count   .store(0, Ordering::Relaxed);
        self.statistics.reallocations_count        .store(0, Ordering::Relaxed);
        self.statistics.allocated_bytes            .store(0, Ordering::Relaxed);
        self.statistics.deallocated_bytes          .store(0, Ordering::Relaxed);
        self.statistics.zeroed_allocated_bytes     .store(0, Ordering::Relaxed);
        self.statistics.reallocated_originals_bytes.store(0, Ordering::Relaxed);
        self.statistics.reallocated_news_bytes     .store(0, Ordering::Relaxed);
    }
    pub fn save_point(&self) -> CustomAllocatorStatistics<usize> {
        CustomAllocatorStatistics {
            allocations_count:           self.statistics.allocations_count          .load(Ordering::Relaxed),
            deallocations_count:         self.statistics.deallocations_count        .load(Ordering::Relaxed),
            zeroed_allocations_count:    self.statistics.zeroed_allocations_count   .load(Ordering::Relaxed),
            reallocations_count:         self.statistics.reallocations_count        .load(Ordering::Relaxed),
            allocated_bytes:             self.statistics.allocated_bytes            .load(Ordering::Relaxed),
            deallocated_bytes:           self.statistics.deallocated_bytes          .load(Ordering::Relaxed),
            zeroed_allocated_bytes:      self.statistics.zeroed_allocated_bytes     .load(Ordering::Relaxed),
            reallocated_originals_bytes: self.statistics.reallocated_originals_bytes.load(Ordering::Relaxed),
            reallocated_news_bytes:      self.statistics.reallocated_news_bytes     .load(Ordering::Relaxed),
        }
    }
    pub fn live_statistics(&self) -> &CustomAllocatorStatistics<AtomicUsize> {
        &self.statistics
    }
    pub fn delta_statistics(&self, save_point: &CustomAllocatorStatistics<usize>) -> CustomAllocatorStatistics<usize> {
        CustomAllocatorStatistics::<usize> {
            allocations_count:           self.statistics.allocations_count          .load(Ordering::Relaxed) - save_point.allocations_count,
            deallocations_count:         self.statistics.deallocations_count        .load(Ordering::Relaxed) - save_point.deallocations_count,
            zeroed_allocations_count:    self.statistics.zeroed_allocations_count   .load(Ordering::Relaxed) - save_point.zeroed_allocations_count,
            reallocations_count:         self.statistics.reallocations_count        .load(Ordering::Relaxed) - save_point.reallocations_count,
            allocated_bytes:             self.statistics.allocated_bytes            .load(Ordering::Relaxed) - save_point.allocated_bytes,
            deallocated_bytes:           self.statistics.deallocated_bytes          .load(Ordering::Relaxed) - save_point.deallocated_bytes,
            zeroed_allocated_bytes:      self.statistics.zeroed_allocated_bytes     .load(Ordering::Relaxed) - save_point.zeroed_allocated_bytes,
            reallocated_originals_bytes: self.statistics.reallocated_originals_bytes.load(Ordering::Relaxed) - save_point.reallocated_originals_bytes,
            reallocated_news_bytes:      self.statistics.reallocated_news_bytes     .load(Ordering::Relaxed) - save_point.reallocated_news_bytes,
        }
    }
}
unsafe impl<'a> GlobalAlloc for CustomAllocator<'a> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.statistics.allocations_count.fetch_add(1, Ordering::Relaxed);
        self.statistics.allocated_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        self.system_allocator.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.statistics.deallocations_count.fetch_add(1, Ordering::Relaxed);
        self.statistics.deallocated_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        self.system_allocator.dealloc(ptr, layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.statistics.zeroed_allocations_count.fetch_add(1, Ordering::Relaxed);
        self.statistics.zeroed_allocated_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        self.system_allocator.alloc_zeroed(layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.statistics.reallocations_count.fetch_add(1, Ordering::Relaxed);
        self.statistics.reallocated_originals_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        self.statistics.reallocated_news_bytes.fetch_add(new_size, Ordering::Relaxed);
        self.system_allocator.realloc(ptr, layout, new_size)
    }
}

//static _ALLOC:
#[global_allocator]
static ALLOC: CustomAllocator = CustomAllocator::new(&System);

fn custom_allocator() {
    let startup_statistics = ALLOC.save_point();
    let allocator_all_time_live_statistics = ALLOC.live_statistics();
    println!("Allocator Statistics (up to program start)  {}", startup_statistics);
    println!("current Allocator Statistics (before print) {}", allocator_all_time_live_statistics);
    println!("global allocator now                        {}", allocator_all_time_live_statistics);
}