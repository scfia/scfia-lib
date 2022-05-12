use super::{bit_vector::BitVector, expression::Expression};

pub trait BitVectorExpression: Expression + BitVector {}
