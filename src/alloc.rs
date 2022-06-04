use std::collections::BTreeMap;
use std::fmt;
use std::sync::Mutex;

#[derive(Default)]
struct Stat {
    current: isize,
    peak: isize,
    total: isize,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "current: {}, peak: {}, total: {}", self.current, self.peak, self.total)
    }
}

impl Stat {
    fn record(&mut self, size: isize) {
        if self.current > self.peak {
            self.peak = self.current;
        }

        self.current += size;

        if self.current > self.peak {
            self.peak = self.current;
        }

        if size > 0 {
            self.total += size;
        }
    }
}

lazy_static::lazy_static! {
    static ref RESULTS: Mutex<BTreeMap<Allocation, Stat>> = Mutex::new(BTreeMap::new());
}

memento::usecase! {
    pub enum Allocation {
        default None,
        Link,
        Document,
    }

    impl memento::UseCase for Allocation {
        fn on_alloc(&self, size: usize) {
            if let Ok(mut map) = RESULTS.try_lock() {
                map.entry(*self).or_insert_with(Default::default).record(size as isize);
            }
        }

        fn on_dealloc(&self, size: usize) {
            if let Ok(mut map) = RESULTS.try_lock() {
                map.entry(*self).or_insert_with(Default::default).record(-(size as isize));
            }
        }
    }
}

impl fmt::Display for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Allocation::None => write!(f, "none"),
            Allocation::Link => write!(f, "link"),
            Allocation::Document => write!(f, "document"),
        }
    }
}

pub fn print_alloc_stats() {
    println!("allocation stats:");
    let guard = RESULTS.lock().unwrap();
    for (usecase, size) in guard.iter() {
        println!("  {}: {}", usecase, size);
    }
}

pub type Allocator = memento::Alloc<Allocation>;

#[global_allocator]
static ALLOCATOR: Allocator = memento::new!();
