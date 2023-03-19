use std::fmt::Debug;

pub struct BoolConcrete {
    pub value: bool,
}

impl Debug for BoolConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}
