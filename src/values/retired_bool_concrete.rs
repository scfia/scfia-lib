use std::fmt::Debug;

pub struct RetiredBoolConcrete {
    pub value: bool,
}

impl Debug for RetiredBoolConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}
