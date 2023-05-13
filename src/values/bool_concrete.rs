use std::fmt::Debug;

pub struct BoolConcrete {
    pub value: bool,
}

pub struct RetiredBoolConcrete {
    pub value: bool,
}

impl Debug for BoolConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}

impl Debug for RetiredBoolConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}
