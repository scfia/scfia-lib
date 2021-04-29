use isla_lib::{
    concrete::BV,
    error::ExecError,
    ir::Val,
    memory::{Address, CustomRegion},
    smt::{
        smtlib::{Exp, Ty},
        Event, Solver,
    },
};
use log::info;

/// A memory region that returns fresh symbols when read and does nothing when written.
#[derive(Clone, Debug)]
pub struct IslaVolatileMemoryRegion {}
const VOLATILE_MEMORY: &str = "VolatileMemoryRegion";

impl<B: 'static + BV> CustomRegion<B> for IslaVolatileMemoryRegion {
    fn read(
        &mut self,
        read_kind: Val<B>,
        address: Address,
        bytes: u32,
        solver: &mut Solver<B>,
        tag: bool,
    ) -> Result<Val<B>, ExecError> {
        info!(
            "IslaVolatileMemoryRegion read 0x{:x} 0x{:x}",
            address, bytes
        );
        let bit_len = bytes * 8;
        assert!(!tag);

        let mut symbol = solver.define_const(Exp::Bits64(0, bit_len));
        let mut bsymbols = vec![];

        // Gather the individual bytes
        for byte in 0..bytes {
            bsymbols.push(solver.declare_const(Ty::BitVec(8)));
            solver.add_event(Event::ReadMem {
                value: Val::Symbolic(bsymbols[byte as usize]),
                read_kind: read_kind.clone(),
                address: Val::Bits(B::from_u64(address + byte as u64)),
                bytes: 1,
                tag_value: None,
                kind: VOLATILE_MEMORY,
            })
        }

        // Create the final symbol
        for i in 0..bytes {
            let bsymbol = bsymbols[i as usize];
            assert!(solver.length(bsymbol).unwrap() == 8);

            // Extend
            let extend_bsymbol = Exp::ZeroExtend(bit_len - 8, Box::new(Exp::Var(bsymbol)));

            // Shift (TODO we could also create BVs with the correct size in the first place)
            let shift_bsymbol = if i == 0 {
                extend_bsymbol
            } else {
                // z3 does not appreciate shifts with a BV with a different lengh (https://github.com/Z3Prover/z3/issues/978)
                Exp::Bvshl(
                    Box::new(extend_bsymbol),
                    Box::new(Exp::Bits64(8 * i as u64, bit_len)),
                )
            };

            // Or
            let or_symbol = Exp::Bvor(Box::new(shift_bsymbol), Box::new(Exp::Var(symbol)));
            symbol = solver.define_const(or_symbol);
        }

        Ok(Val::Symbolic(symbol))
    }

    fn write(
        &mut self,
        _write_kind: Val<B>,
        _address: Address,
        _data: Val<B>,
        _solver: &mut Solver<B>,
        _tag_value: Option<Val<B>>,
    ) -> Result<Val<B>, ExecError> {
        //TODO wtf is this return value
        Ok(Val::Unit)
    }

    fn initial_value(&self, _address: Address, _bytes: u32) -> Option<B> {
        unimplemented!()
    }

    fn clone_dyn(&self) -> Box<dyn Send + Sync + CustomRegion<B>> {
        Box::new(self.clone())
    }

    fn memory_kind(&self) -> &'static str {
        VOLATILE_MEMORY
    }
}
