use crate::{
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::transformation_function_expression::ScfiaTransformationFunctionExpression,
};

#[derive(Debug)]
pub enum ScfiaAarch64TransformationFunctionMemoryWrite {
    WriteMemoryConcrete(u64, u8),
    WriteMemorySymbolic(
        u64,
        ScfiaTransformationFunctionExpression<ScfiaAarch64SystemState>,
    ),
}
