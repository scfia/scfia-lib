use std::fmt::Debug;

#[derive(Debug)]
pub struct BVConcrete {
    pub value: u64,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVConcrete {
    pub value: u64,
    pub width: u32,
}
