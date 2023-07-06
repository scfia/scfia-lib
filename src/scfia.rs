use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::Not;
use std::rc::Rc;
use std::rc::Weak;

use log::error;
use log::trace;

use crate::values::active_value::ActiveExpression;
use crate::values::active_value::ActiveValue;
use crate::values::active_value::ActiveValueZ3;
use crate::values::active_value::ValueComment;
use crate::values::bool_eq_expression::BoolEqExpression;
use crate::values::bool_eq_expression::RetiredBoolEqExpression;
use crate::values::bool_not_expresssion::BoolNotExpression;
use crate::values::bool_not_expresssion::RetiredBoolNotExpression;
use crate::values::bool_signed_less_than_expression::BoolSignedLessThanExpression;
use crate::values::bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::BoolUnsignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression;
use crate::values::bv_add_expression::BVAddExpression;
use crate::values::bv_add_expression::RetiredBVAddExpression;
use crate::values::bv_and_expression::BVAndExpression;
use crate::values::bv_and_expression::RetiredBVAndExpression;
use crate::values::bv_concat_expression::BVConcatExpression;
use crate::values::bv_concat_expression::RetiredBVConcatExpression;
use crate::values::bv_concrete_expression::BVConcreteExpression;
use crate::values::bv_concrete_expression::RetiredBVConcreteExpression;
use crate::values::bv_multiply_expression::BVMultiplyExpression;
use crate::values::bv_multiply_expression::RetiredBVMultiplyExpression;
use crate::values::bv_not_expression::BVNotExpression;
use crate::values::bv_not_expression::RetiredBVNotExpression;
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
use crate::values::bv_unsigned_remainder_expression::RetiredBVUnsignedRemainderExpression;
use crate::values::bv_xor_expression::BVXorExpression;
use crate::values::bv_xor_expression::RetiredBVXorExpression;
use crate::values::retired_value::ParentWeakReference;
use crate::values::retired_value::RetiredExpression;
use crate::values::retired_value::RetiredValue;
use crate::values::retired_value::RetiredValueInner;
use crate::z3_handle::Z3Ast;
use crate::z3_handle::Z3Handle;
use crate::GenericForkSink;
use crate::ScfiaComposition;

pub struct Scfia<SC: ScfiaComposition> {
    pub z3: Rc<Z3Handle<SC>>,
    pub next_symbol_id: Cell<u64>,
    pub selff: OnceCell<Weak<Self>>,
    phantom: PhantomData<SC>,
}

impl<SC: ScfiaComposition> Scfia<SC> {
    pub fn new(next_symbol_id: Option<u64>) -> Rc<Self> {
        let scfia = Rc::new(Scfia {
            z3: Z3Handle::new(),
            next_symbol_id: Cell::new(next_symbol_id.unwrap_or_default()),
            selff: OnceCell::new(),
            phantom: PhantomData,
        });
        scfia.selff.set(Rc::downgrade(&scfia)).unwrap();
        scfia
    }

    pub fn new_bool_concrete(&self, value: bool, id: Option<u64>, _fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let _id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let _z3_ast = self.z3.new_bool_concrete(value);
        ActiveValue::BoolConcrete(value)
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
        if let ActiveValue::BVConcrete(s1_value, _) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _) = s2 {
                return self.new_bool_concrete(s1_value == s2_value, None, fork_sink);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_eq(&s1.get_z3_ast(), &s2.get_z3_ast(), is_assert);
        self.new_active(
            ActiveExpression::BoolEqExpression(BoolEqExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        if let ActiveValue::BoolConcrete(value_s1) = s1 {
            return self.new_bool_concrete(!value_s1, None, fork_sink);
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_not(&s1.get_z3_ast(), is_assert);
        self.new_active(
            ActiveExpression::BoolNotExpression(BoolNotExpression {
                s1: s1.get_z3_value(),
                is_assert,
            }),
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
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let slt = (*s1_value as i64) < (*s2_value as i64); // TODO this probably breaks for 64bit BVs
                return self.new_bool_concrete(slt, None, fork_sink);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvslt(&s1.get_z3_ast(), &s2.get_z3_ast(), is_assert);
        self.new_active(
            ActiveExpression::BoolSignedLessThanExpression(BoolSignedLessThanExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                is_assert,
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
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let ult = s1_value < s2_value;
                return self.new_bool_concrete(ult, None, fork_sink);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvult(&s1.get_z3_ast(), &s2.get_z3_ast(), is_assert);
        self.new_active(
            ActiveExpression::BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s2_width).overflowing_sub(1).0; // TODO is rotate_left the right choice here?
                let sum = s1_value.overflowing_add(*s2_value).0;
                let value = mask & sum;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvadd(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVAddExpression(BVAddExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let value = s1_value & s2_value;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvand(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVAndExpression(BVAndExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, s2_width) = s2 {
                let e1_shifted = s1_value << s2_width;
                let value = e1_shifted | s2_value;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvconcat(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVConcatExpression(BVConcatExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_concrete(&self, value: u64, width: u32) -> ActiveValue<SC> {
        ActiveValue::BVConcrete(value, width)
    }

    pub fn new_bv_concrete_z3(
        &self,
        value: u64,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bv_concrete(value, width);
        self.new_active(
            ActiveExpression::BVConcreteExpression(BVConcreteExpression {
                value,
                width,
                phantom: PhantomData,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_multiply(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s2_width).overflowing_sub(1).0;
                let product = s1_value.overflowing_mul(*s2_value).0;
                let value = mask & product;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvmul(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVMultiplyExpression(BVMultiplyExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_not(
        &self,
        s1: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            let one: u64 = 1;
            let mask = one.rotate_left(width).overflowing_sub(1).0;
            let not = s1_value.not();
            let value = mask & not;
            return ActiveValue::BVConcrete(value, width);
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvnot(&s1.get_z3_ast());
        self.new_active(
            ActiveExpression::BVNotExpression(BVNotExpression {
                s1: s1.get_z3_value(),
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
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let value = s1_value | s2_value;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvor(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVOrExpression(BVOrExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        s1: &ActiveValue<SC>,
        input_width: u32,
        output_width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, s1_width) = s1 {
            // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
            let m: u64 = 1 << (s1_width - 1);
            let x = s1_value & ((1 << s1_width) - 1);
            let value = (x ^ m).overflowing_sub(m).0;
            return ActiveValue::BVConcrete(value, output_width);
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_sign_ext(output_width - input_width, &s1.get_z3_ast());
        self.new_active(
            ActiveExpression::BVSignExtendExpression(BVSignExtendExpression {
                s1: s1.get_z3_value(),
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
        &self,
        s1: &ActiveValue<SC>,
        high: u32,
        low: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let width = high - low + 1;
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
            let shifted = s1_value >> low;
            let mask = (1 << width) - 1;
            let value = shifted & mask;
            return ActiveValue::BVConcrete(value, width);
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let width = high - low + 1;
        let z3_ast = self.z3.new_extract(high, low, &s1.get_z3_ast());
        self.new_active(
            ActiveExpression::BVSliceExpression(BVSliceExpression {
                s1: s1.get_z3_value(),
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
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s1_width).overflowing_sub(1).0;
                let value = (s1_value << s2_value) & mask;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvshl(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVSllExpression(BVSllExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_srl(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s1_width).overflowing_sub(1).0;
                let value = (s1_value >> s2_value) & mask;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvlshr(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVSllExpression(BVSllExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_sub(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s2_width).overflowing_sub(1).0;
                let sum = s1_value.overflowing_sub(*s2_value).0;
                let value = mask & sum;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvsub(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVSubExpression(BVSubExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_symbol(&self, width: u32, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>, comment: Option<ValueComment>) -> ActiveValue<SC> {
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_fresh_const(width);
        self.new_active(ActiveExpression::BVSymbol(BVSymbol { width }), z3_ast, id, fork_sink, comment)
    }

    pub fn new_bv_unsigned_remainder(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, s2_width) = s2 {
                let one: u64 = 1;
                let mask = one.rotate_left(*s2_width).overflowing_sub(1).0;
                let remainder = s1_value % s2_value;
                let value = mask & remainder;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvurem(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVUnsignedRemainderExpression(BVUnsignedRemainderExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
                width,
            }),
            z3_ast,
            id,
            fork_sink,
            comment,
        )
    }

    pub fn new_bv_xor(
        &self,
        s1: &ActiveValue<SC>,
        s2: &ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        if let ActiveValue::BVConcrete(s1_value, _s1_width) = s1 {
            if let ActiveValue::BVConcrete(s2_value, _s2_width) = s2 {
                let value = s1_value ^ s2_value;
                return ActiveValue::BVConcrete(value, width);
            }
        };

        let s1 = s1.into_z3_value(self, fork_sink);
        let s2 = s2.into_z3_value(self, fork_sink);
        let id = if let Some(id) = id { id } else { self.next_symbol_id() };
        let z3_ast = self.z3.new_bvxor(&s1.get_z3_ast(), &s2.get_z3_ast());
        self.new_active(
            ActiveExpression::BVXorExpression(BVXorExpression {
                s1: s1.get_z3_value(),
                s2: s2.get_z3_value(),
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
        z3_ast: Z3Ast<SC>,
        id: u64,
        fork_sink: &mut Option<SC::ForkSink>,
        comment: Option<ValueComment>,
    ) -> ActiveValue<SC> {
        let value = ActiveValue::Expression(Rc::new(RefCell::new(ActiveValueZ3 {
            id,
            z3_ast,
            expression,
            inherited_asts: BTreeMap::new(),
            discovered_asts: BTreeMap::new(),
            scfia: self.selff.get().unwrap().clone(),
            comment,
            can_inherit: true,
        })));

        if let Some(fork_sink) = fork_sink {
            fork_sink.push_value(value.clone())
        }
        value
    }

    pub fn new_inactive(&self, expression: RetiredExpression<SC>, z3_ast: Z3Ast<SC>, id: u64) -> RetiredValue<SC> {
        let value = Rc::new(RefCell::new(RetiredValueInner {
            id,
            z3_ast,
            expression,
            scfia: self.selff.get().unwrap().clone(),
            comment: None,
        }));
        value
    }

    pub fn drop_active_expression(&self, value: &ActiveValueZ3<SC>) -> RetiredValue<SC> {
        trace!("Dropping {} ({:?})", value.id, value);
        let expression = match &value.expression {
            ActiveExpression::BoolEqExpression(e) => RetiredExpression::BoolEqExpression(RetiredBoolEqExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                is_assert: e.is_assert,
                phantom: PhantomData,
            }),
            ActiveExpression::BoolNotExpression(e) => RetiredExpression::BoolNotExpression(RetiredBoolNotExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                is_assert: e.is_assert,
                phantom: PhantomData,
            }),
            ActiveExpression::BoolSignedLessThanExpression(e) => RetiredExpression::BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                is_assert: e.is_assert,
                phantom: PhantomData,
            }),
            ActiveExpression::BoolUnsignedLessThanExpression(e) => RetiredExpression::BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                is_assert: e.is_assert,
                phantom: PhantomData,
            }),
            ActiveExpression::BVAddExpression(e) => RetiredExpression::BVAddExpression(RetiredBVAddExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVAndExpression(e) => RetiredExpression::BVAndExpression(RetiredBVAndExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVConcatExpression(e) => RetiredExpression::BVConcatExpression(RetiredBVConcatExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVMultiplyExpression(e) => RetiredExpression::BVMultiplyExpression(RetiredBVMultiplyExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVNotExpression(e) => RetiredExpression::BVNotExpression(RetiredBVNotExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVOrExpression(e) => RetiredExpression::BVOrExpression(RetiredBVOrExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSignExtendExpression(e) => RetiredExpression::BVSignExtendExpression(RetiredBVSignExtendExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                input_width: e.width,
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSliceExpression(e) => RetiredExpression::BVSliceExpression(RetiredBVSliceExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                high: e.high,
                low: e.low,
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSllExpression(e) => RetiredExpression::BVSllExpression(RetiredBVSllExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSrlExpression(e) => RetiredExpression::BVSrlExpression(RetiredBVSrlExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                shamt: e.shamt,
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSubExpression(e) => RetiredExpression::BVSubExpression(RetiredBVSubExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVSymbol(e) => RetiredExpression::BVSymbol(RetiredBVSymbol { width: e.width }),
            ActiveExpression::BVUnsignedRemainderExpression(e) => RetiredExpression::BVUnsignedRemainderExpression(RetiredBVUnsignedRemainderExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVXorExpression(e) => RetiredExpression::BVXorExpression(RetiredBVXorExpression {
                s1: ParentWeakReference {
                    id: e.s1.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s1),
                },
                s2: ParentWeakReference {
                    id: e.s2.try_borrow().unwrap().id,
                    weak: Rc::downgrade(&e.s2),
                },
                width: e.width,
                phantom: PhantomData,
            }),
            ActiveExpression::BVConcreteExpression(e) => RetiredExpression::BVConcreteExpression(RetiredBVConcreteExpression {
                value: e.value,
                width: e.width,
            }),
        };

        let inactive = self.new_inactive(expression, value.z3_ast.clone(), value.id);

        // Heirs are parents and discovered symbols
        let mut heirs = vec![];
        value.get_parents(&mut heirs);
        for discovered_ast in value.discovered_asts.values() {
            let acquaintance = discovered_ast.upgrade().unwrap();
            let mut acquaintance_mut = acquaintance.try_borrow_mut().unwrap();
            acquaintance_mut.discovered_asts.remove(&value.id);
            heirs.push(acquaintance.clone())
        }

        // For each heir...
        for heir in &heirs {
            let mut heir_mut = heir.try_borrow_mut().unwrap();
            trace!("{:?} attempting to inherit {:?}", heir_mut.id, value.id);

            // Inherit
            if !heir_mut.is_concrete() && heir_mut.can_inherit {
                heir_mut.inherited_asts.insert(value.id, inactive.clone());

                // Pass on inherited symbols
                for (inherited_value_id, inherited_value) in &value.inherited_asts {
                    let old = heir_mut.inherited_asts.insert(*inherited_value_id, inherited_value.clone());
                    if let Some(old) = old {
                        if !Rc::ptr_eq(&old, inherited_value) {
                            error!(
                                "{:?} tried to pass on {:?}, but {:?} already had {:?} (same id, different thing)",
                                value, inherited_value, heir_mut, old
                            );
                            panic!();
                        }
                    }
                }
            }

            // Acquaint all heirs
            for other_heir in &heirs {
                if !Rc::ptr_eq(heir, other_heir) {
                    let mut other_heir_mut = other_heir.try_borrow_mut().unwrap();
                    other_heir_mut.discovered_asts.insert(heir_mut.id, Rc::downgrade(heir));
                    heir_mut.discovered_asts.insert(other_heir_mut.id, Rc::downgrade(other_heir));
                }
            }
        }

        inactive
    }

    pub fn check_condition(&self, condition: &ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> bool {
        self.z3.check_condition(self, condition, fork_sink)
    }

    pub fn monomorphize_active(&self, value: &ActiveValue<SC>, candidates: &mut Vec<u64>) {
        self.z3.monomorphize(&value.get_z3_ast(), candidates);
    }

    pub fn new_bv_constrained(&self, width: u32, align: u64, limit: u64) -> ActiveValue<SC> {
        let base_symbol = {
            let base_symbol = self.new_bv_symbol(
                width,
                None,
                &mut None,
                Some(ValueComment {
                    message: "constrained symbol".to_string(),
                }),
            );

            // Assert base_symbol & align == 0
            let align_bv = ActiveValue::BVConcrete(align, width);
            let align_and = self.new_bv_and(&base_symbol, &align_bv, width, None, &mut None, None);
            let zero = ActiveValue::BVConcrete(0, width);
            // I swear to whichever deity the Drop behaviour is a nightmare
            self.new_bool_eq(&align_and, &zero, None, true, &mut None, None);

            // Assert base_symbol < max
            let limit_bv = ActiveValue::BVConcrete(limit, width);
            self.new_bool_unsigned_less_than(&base_symbol, &limit_bv, None, true, &mut None, None);

            base_symbol
        };
        base_symbol.set_can_inherit(false);
        base_symbol
    }
}

/*TODO
#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, rc::Rc};

    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    pub fn test_scfia() {
        let scfia: Rc<Scfia<RV32iScfiaComposition>> = Scfia::new(None);
        let s1 = scfia.new_bv_symbol(32, None, &mut None, None);
        let s2 = scfia.new_bv_concrete(0x0f, 32, None, &mut None, None);
        let s3 = scfia.new_bv_symbol(32, None, &mut None, None);
        let s4 = scfia.new_bv_multiply(&s3, &s3, 32, None, &mut None, None);
        let s5 = scfia.new_bool_eq(&s4, &s3, None, true, &mut None, None);
        {
            let _s5 = s5.try_borrow().unwrap().get_z3_ast().clone();
        }
        assert_eq!(scfia.z3.ast_refs.get(), 5);
        let and = scfia.new_bv_and(&s1, &s2, 32, None, &mut None, None);
        let mut candidates = vec![];
        scfia.monomorphize_active(&and, &mut candidates);
        assert_eq!(scfia.z3.ast_refs.get(), 6);

        let cloned_scfia: Rc<Scfia<RV32iScfiaComposition>> = Scfia::new(Some(scfia.next_symbol_id.get()));
        let mut cloned_actives = BTreeMap::new();
        let mut cloned_inactives = BTreeMap::new();
        let _cloned_add = and
            .try_borrow()
            .unwrap()
            .clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_inactives);
        let _cloned_s5 = s5
            .try_borrow()
            .unwrap()
            .clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_inactives);
        assert_eq!(cloned_scfia.z3.ast_refs.get(), 6);
    }
}
*/
