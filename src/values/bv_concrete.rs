use std::fmt::Debug;

pub struct BVConcrete {
    pub value: u64,
    pub width: u32,
}

impl Debug for BVConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("0x{:x}[{}]", self.value, self.width).as_str())
    }
}
