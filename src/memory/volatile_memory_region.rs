use std::ops::Range;

#[derive(Clone, Debug)]
pub struct VolatileMemoryRegion {
    pub range: Range<u64>,
}
