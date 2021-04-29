use crate::symbols::{
    scfia_symbol::{ScfiaSymbol, SymbolInnerVariant},
    scfia_symbol_expression::ScfiaSymbolExpression,
    scfia_symbol_manager::ScfiaSymbolManager,
};

pub trait DebugIndented {
    fn debug_indented(
        &self,
        symbol_manager: &ScfiaSymbolManager,
        current_indentation: u64,
        indentation_width: u64,
    ) -> String;
    fn indent(string: &mut String, indentation: u64) {
        for _ in 0..indentation {
            string.push_str(" ");
        }
    }
}

impl DebugIndented for u64 {
    fn debug_indented(
        &self,
        _symbol_manager: &ScfiaSymbolManager,
        current_indentation: u64,
        _indentation_width: u64,
    ) -> String {
        let mut string = String::new();
        Self::indent(&mut string, current_indentation);
        string.push_str(&format!("0x{:x}", self));
        string
    }
}

impl DebugIndented for u32 {
    fn debug_indented(
        &self,
        _symbol_manager: &ScfiaSymbolManager,
        current_indentation: u64,
        _indentation_width: u64,
    ) -> String {
        let mut string = String::new();
        Self::indent(&mut string, current_indentation);
        string.push_str(&format!("0x{:x}", self));
        string
    }
}

impl DebugIndented for ScfiaSymbol {
    fn debug_indented(
        &self,
        symbol_manager: &ScfiaSymbolManager,
        current_indentation: u64,
        indentation_width: u64,
    ) -> String {
        let mut string = String::new();
        Self::indent(&mut string, current_indentation);
        string.push_str(&format!(
            "Symbol(id={}, refcount={}, variant=",
            self.id,
            symbol_manager
                .active_symbols
                .get(&self.id)
                .unwrap()
                .references
        ));
        match &self.variant {
            SymbolInnerVariant::Bitvector(width, _) => {
                string.push_str(&format!("Bitvector(width={}))", width));
            }
            SymbolInnerVariant::Bool => {
                string.push_str(&format!("Bool)"));
            }
            SymbolInnerVariant::Defined(expression) => {
                string.push_str(&format!("Defined(\n"));
                string.push_str(&format!(
                    "{}\n",
                    expression.debug_indented(
                        symbol_manager,
                        current_indentation + indentation_width,
                        indentation_width
                    )
                ));
                Self::indent(&mut string, current_indentation);
                string.push_str("))")
            }
        }
        string
    }
}

impl DebugIndented for ScfiaSymbolExpression {
    fn debug_indented(
        &self,
        symbol_manager: &ScfiaSymbolManager,
        current_indentation: u64,
        indentation_width: u64,
    ) -> String {
        let mut string = String::new();
        Self::indent(&mut string, current_indentation);
        match self {
            ScfiaSymbolExpression::Var(s) => string.push_str(&format!(
                "ScfiaSymbolExpression::Var(\n{}\n",
                symbol_manager.get_symbol(s).debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bits(bits) => {
                string.push_str(&format!("ScfiaSymbolExpression::Bits({:?}\n", bits))
            }
            ScfiaSymbolExpression::Bits64(lower_u64, width) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bits64(\n{},\n{}\n",
                lower_u64.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                width.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Enum(a, b) => string.push_str("Enum"),
            ScfiaSymbolExpression::Bool(b) => string.push_str(&format!("Bool({})\n", b)),
            ScfiaSymbolExpression::Eq(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Eq(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Neq(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Neq(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::And(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::And(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Or(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Or(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Not(a) => string.push_str(&format!(
                "ScfiaSymbolExpression::Not(\n{}\n)",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvnot(a) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvnot(\n{}\n)",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvand(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvand(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvor(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvor(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvxor(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvxor(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvnand(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvnand(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvnor(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvnor(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvxnor(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvxnor(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvneg(a) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvneg(\n{}\n)",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvadd(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvadd(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsub(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsub(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvmul(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvmul(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvudiv(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvudiv(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsdiv(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsdiv(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvurem(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvurem(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsrem(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsrem(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsmod(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsmod(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvult(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvult(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvslt(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvslt(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvule(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvule(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsle(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsle(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvuge(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvuge(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsge(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsge(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvugt(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvugt(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvsgt(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvsgt(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Extract(a, b, c) => string.push_str(&format!(
                "ScfiaSymbolExpression::Extract(\n{},\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                c.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::ZeroExtend(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::ZeroExtend(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::SignExtend(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::SignExtend(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvshl(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvshl(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvlshr(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvlshr(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Bvashr(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Bvashr(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Concat(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Concat(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Ite(a, b, c) => string.push_str(&format!(
                "ScfiaSymbolExpression::Ite(\n{},\n{}\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                c.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Select(a, b) => string.push_str(&format!(
                "ScfiaSymbolExpression::Eq(\n{},\n{}\n",
                a.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                ),
                b.debug_indented(
                    symbol_manager,
                    current_indentation + indentation_width,
                    indentation_width
                )
            )),
            ScfiaSymbolExpression::Store(_, _, _) => unimplemented!(),
        }
        Self::indent(&mut string, current_indentation);
        string.push_str(")");
        string
    }
}
