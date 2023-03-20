use std::fmt::Debug;

pub struct BVSymbol {
    pub width: u32,
}

pub struct RetiredBVSymbol {
    pub width: u32,
}

impl Debug for BVSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{{{}}}", self.width).as_str())
    }
}

impl Debug for RetiredBVSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{{{}}}", self.width).as_str())
    }
}
