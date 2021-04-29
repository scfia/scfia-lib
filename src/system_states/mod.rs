pub mod aarch64;

use std::fmt::Debug;

use crate::transformation_functions::ScfiaTransformationDefinition;

pub trait ScfiaSystemState: Clone + Debug + Send + Sync {
    type TransformationDefinition: ScfiaTransformationDefinition<Self> + Debug;
}
