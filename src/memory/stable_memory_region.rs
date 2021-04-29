use std::{collections::HashMap, ops::Range};

use crate::values::value8::Value8;

#[derive(Clone, Debug)]
pub struct StableMemoryRegion {
    pub data: HashMap<u64, Value8>,
    pub range: Range<u64>,
}
