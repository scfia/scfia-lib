use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::rc::Rc;
use std::rc::Weak;
use std::time::Instant;

use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;

use crate::values::active_value::ActiveExpression;
use crate::values::active_value::ActiveValue;
use crate::values::active_value::ActiveValueInner;
use crate::values::active_value::ActiveValueWeak;
use crate::values::active_value::ValueComment;
use crate::values::bool_concrete::BoolConcrete;
use crate::values::bool_eq_expression::BoolEqExpression;
use crate::values::bool_not_expresssion::BoolNotExpression;
use crate::values::bool_signed_less_than_expression::BoolSignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::BoolUnsignedLessThanExpression;
use crate::values::bv_add_expression::BVAddExpression;
use crate::values::bv_and_expression::BVAndExpression;
use crate::values::bv_concat_expression::BVConcatExpression;
use crate::values::bv_concrete::BVConcrete;
use crate::GenericForkSink;
use crate::ScfiaComposition;

use crate::values::bool_concrete::RetiredBoolConcrete;
use crate::values::bool_eq_expression::RetiredBoolEqExpression;
use crate::values::bool_not_expresssion::RetiredBoolNotExpression;
use crate::values::bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression;
use crate::values::bv_add_expression::RetiredBVAddExpression;
use crate::values::bv_and_expression::RetiredBVAndExpression;
use crate::values::bv_concat_expression::RetiredBVConcatExpression;
use crate::values::bv_concrete::RetiredBVConcrete;
use crate::values::bv_multiply_expression::BVMultiplyExpression;
use crate::values::bv_multiply_expression::RetiredBVMultiplyExpression;
use crate::values::bv_or_expression::BVOrExpression;
use crate::values::bv_or_expression::RetiredBVOrExpression;
use crate::values::bv_sign_extend_expression::BVSignExtendExpression;
use crate::values::bv_sign_extend_expression::RetiredBVSignExtendExpression;
use crate::values::bv_slice_expression::BVSliceExpression;
use crate::values::bv_slice_expression::RetiredBVSliceExpression;
use crate::values::bv_sll_expression::BVSllExpression;
use crate::values::bv_sll_expression::RetiredBVSllExpression;
use crate::values::bv_srl_expression::RetiredBVSrlExpression;
use crate::values::bv_sub_expression::BVSubExpression;
use crate::values::bv_sub_expression::RetiredBVSubExpression;
use crate::values::bv_symbol::BVSymbol;
use crate::values::bv_symbol::RetiredBVSymbol;
use crate::values::bv_unsigned_remainder_expression::BVUnsignedRemainderExpression;
use crate::values::bv_xor_expression::BVXorExpression;
use crate::values::bv_xor_expression::RetiredBVXorExpression;
use crate::values::retired_value::RetiredExpression;
use crate::values::retired_value::RetiredValue;
use crate::values::retired_value::RetiredValueInner;
use crate::values::retired_value::RetiredValueWeak;
use crate::z3_handle::Z3Ast;
use crate::z3_handle::Z3Handle;

pub const PREFIX: [i8; 4] = ['p' as i8, 'r' as i8, 'e' as i8, 0];

pub struct Scfia<SC: ScfiaComposition> {
    pub(crate) z3: Rc<Z3Handle>,
    pub next_symbol_id: Cell<u64>,
    selff: OnceCell<Weak<Self>>,
    phantom: PhantomData<SC>,
}

impl<SC: ScfiaComposition> Scfia<SC> {
    pub fn new_bool_concrete(&self, value: bool, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bool_concrete(value);
        self.new_active(ActiveExpression::BoolConcrete(BoolConcrete { value }), z3_ast, id, fork_sink, None)
    }

    pub fn new_bool_eq(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        id: Option<u64>,
        is_assert: bool,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let z3_ast = self.z3.new_bool_concrete(s1.value == s2.value);
                return self.new_active(
                    ActiveExpression::BoolConcrete(BoolConcrete { value: s1.value == s2.value }),
                    z3_ast,
                    id,
                    fork_sink,
                    comment,
                );
            }
        };

        let z3_ast = self.z3.new_eq(&s1_inner.z3_ast, &s2_inner.z3_ast, is_assert);
        self.new_active(
            ActiveExpression::BoolEqExpression(BoolEqExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                is_assert,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bool_not(
        &self,
        s1: &ActiveValue<SC>,
        id: Option<u64>,
        is_assert: bool,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BoolConcrete(s1) = &s1_inner.expression {
            let z3_ast = self.z3.new_bool_concrete(!s1.value);
            return self.new_active(
                ActiveExpression::BoolConcrete(BoolConcrete { value: !s1.value }),
                z3_ast,
                id,
                fork_sink,
                comment,
            );
        }

        let z3_ast = self.z3.new_not(&s1_inner.z3_ast, is_assert);
        self.new_active(
            ActiveExpression::BoolNotExpression(BoolNotExpression { s1: s1.clone(), is_assert }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bool_signed_less_than(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        id: Option<u64>,
        is_assert: bool,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let slt = (s1.value as i64) < (s2.value as i64);
                let z3_ast = self.z3.new_bool_concrete(slt);
                return self.new_active(ActiveExpression::BoolConcrete(BoolConcrete { value: slt }), z3_ast, id, fork_sink, comment);
            }
        }

        let z3_ast = self.z3.new_bvslt(&s1_inner.z3_ast, &s2_inner.z3_ast, is_assert);
        self.new_active(
            ActiveExpression::BoolSignedLessThanExpression(BoolSignedLessThanExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                is_assert: is_assert,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bool_unsigned_less_than(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        id: Option<u64>,
        is_assert: bool,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let ult = s1.value < s2.value;
                let z3_ast = self.z3.new_bool_concrete(ult);
                return self.new_active(ActiveExpression::BoolConcrete(BoolConcrete { value: ult }), z3_ast, id, fork_sink, comment);
            }
        }

        let z3_ast = self.z3.new_bvult(&s1_inner.z3_ast, &s2_inner.z3_ast, is_assert);
        self.new_active(
            ActiveExpression::BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                is_assert,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_add(
        &self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let one: u64 = 1;
                let mask = one.rotate_left(s2.width).overflowing_sub(1).0; // TODO is rotate_left the right choice here?
                let sum = s1.value.overflowing_add(s2.value).0;
                let value = mask & sum;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvadd(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVAddExpression(BVAddExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_and(
        &self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let value = s1.value & s2.value;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvand(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVAndExpression(BVAndExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_concat(
        &self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let e1_shifted = s1.value << s2.width;
                let value = e1_shifted | s2.value;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvconcat(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVConcatExpression(BVConcatExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_concrete(
        &self,
        value: u64,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        //TODO assert value sticking to the width mask
        let z3_ast = self.z3.new_bv_concrete(value, width);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment)
    }

    pub fn new_bv_multiply(
        &self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                // TODO check
                let one: u64 = 1;
                let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                let product = s1.value.overflowing_mul(s2.value).0;
                let value = mask & product;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvmul(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVMultiplyExpression(BVMultiplyExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_or(
        &self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let value = s1.value | s2.value;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvor(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVOrExpression(BVOrExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_sign_extend(
        &self,
        s1: ActiveValue<SC>,
        input_width: u32,
        output_width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
            let m: u64 = 1 << (s1.width - 1);
            let x = s1.value & ((1 << s1.width) - 1);
            let value = (x ^ m).overflowing_sub(m).0;
            let z3_ast = self.z3.new_bv_concrete(value, output_width);
            return self.new_active(
                ActiveExpression::BVConcrete(BVConcrete { value, width: output_width }),
                z3_ast,
                id,
                fork_sink,
                comment,
            );
        };

        let z3_ast = self.z3.new_sign_ext(output_width - input_width, &s1_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVSignExtendExpression(BVSignExtendExpression {
                s1: s1.clone(),
                width: output_width,
                input_width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_slice(
        &mut self,
        s1: ActiveValue<SC>,
        high: u32,
        low: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let width = high - low + 1;
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            let shifted = s1.value >> low;
            let mask = (1 << width) - 1;
            let value = shifted & mask;
            let z3_ast = self.z3.new_bv_concrete(value, width);
            return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
        };

        let z3_ast = self.z3.new_extract(high, low, &s1_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVSliceExpression(BVSliceExpression {
                s1: s1.clone(),
                width,
                high,
                low,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_sll(
        &mut self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let one: u64 = 1;
                let mask = one.rotate_left(s1.width).overflowing_sub(1).0;
                let value = (s1.value << s2.value) & mask;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvshl(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVSllExpression(BVSllExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_srl(
        &mut self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                let one: u64 = 1;
                let mask = one.rotate_left(s1.width).overflowing_sub(1).0;
                let value = (s1.value >> s2.value) & mask;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvlshr(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVSllExpression(BVSllExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_sub(
        &mut self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                // TODO check
                let one: u64 = 1;
                let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                let sum = s1.value.overflowing_sub(s2.value).0;
                let value = mask & sum;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvsub(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVSubExpression(BVSubExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_symbol(&mut self, width: u32, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>, comment: Option<ValueComment>) -> ActiveValue<SC> {
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_fresh_const(width);
        self.new_active(ActiveExpression::BVSymbol(BVSymbol { width }), z3_ast, id, fork_sink, comment)
    }

    pub fn new_bv_unsigned_remainder(
        &mut self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                // TODO check
                let one: u64 = 1;
                let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                let remainder = s1.value % s2.value;
                let value = mask & remainder;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvurem(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVUnsignedRemainderExpression(BVUnsignedRemainderExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_xor(
        &mut self,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let s1_inner = s1.try_borrow().unwrap();
        let s2_inner = s2.try_borrow().unwrap();
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
            if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                // TODO check
                let value = s1.value ^ s2.value;
                let z3_ast = self.z3.new_bv_concrete(value, width);
                return self.new_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, fork_sink, comment);
            }
        };

        let z3_ast = self.z3.new_bvxor(&s1_inner.z3_ast, &s2_inner.z3_ast);
        self.new_active(
            ActiveExpression::BVXorExpression(BVXorExpression {
                s1: s1.clone(),
                s2: s2.clone(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    fn next_symbol_id(&self) -> u64 {
        let id = self.next_symbol_id.get();
        self.next_symbol_id.set(id + 1);
        id
    }

    fn new_active(
        &self,
        expression: ActiveExpression<SC>,
        z3_ast: Z3Ast,
        id: u64,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let value = Rc::new(RefCell::new(ActiveValueInner {
            id,
            z3_ast,
            expression,
            inherited_asts: BTreeMap::new(),
            discovered_asts: BTreeMap::new(),
            scfia: self.selff.get().unwrap().clone(),
            comment,
        }));
        if let Some(fork_sink) = fork_sink {
            fork_sink.push_value(value.clone())
        }
        value
    }

    pub fn new_inactive(&self, expression: RetiredExpression<SC>, z3_ast: Z3Ast, id: u64) -> RetiredValue<SC> {
        let value = Rc::new(RefCell::new(RetiredValueInner {
            id,
            z3_ast,
            expression,
            scfia: self.selff.get().unwrap().clone(),
        }));
        value
    }

    pub fn drop_active(&self, value: &ActiveValueInner<SC>) -> RetiredValue<SC> {
        let expression = match &value.expression {
            ActiveExpression::BoolConcrete(v) => RetiredExpression::BoolConcrete(RetiredBoolConcrete { value: v.value }),
            _ => todo!(),
        };

        self.new_inactive(expression, value.z3_ast.clone(), value.id)
    }

    pub fn monomorphize_active(&self, value: &ActiveValueInner<SC>, candidates: &mut Vec<u64>) {
        todo!()
    }
}
