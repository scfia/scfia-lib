use std::fmt::Debug;

use crate::ScfiaComposition;

pub struct BoolConcrete<SC: ScfiaComposition> {
    pub value: bool,
}

pub struct RetiredBoolConcrete<SC: ScfiaComposition> {
    pub value: bool,
}

impl<SC: ScfiaComposition> Debug for BoolConcrete<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}

impl<SC: ScfiaComposition> Debug for RetiredBoolConcrete<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.value).as_str())
    }
}
