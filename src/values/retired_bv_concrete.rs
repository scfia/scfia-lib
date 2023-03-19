use std::fmt::Debug;

pub struct RetiredBVConcrete {
    pub value: u64,
    pub width: u32,
}

impl Debug for RetiredBVConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("0x{:x}[{}]", self.value, self.width).as_str())
    }
}
