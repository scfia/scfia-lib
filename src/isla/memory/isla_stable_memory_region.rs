use std::collections::HashMap;

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
use log::trace;

#[derive(Clone)]
pub struct IslaStableMemoryRegion<B: BV> {
    pub data: HashMap<Address, Val<B>>,
}
const STABLE_MEMORY: &str = "StableMemoryRegion";

impl<B: 'static + BV> CustomRegion<B> for IslaStableMemoryRegion<B> {
    fn read(
        &mut self,
        read_kind: Val<B>,
        address: Address,
        bytes: u32,
        solver: &mut Solver<B>,
        tag: bool,
    ) -> Result<Val<B>, ExecError> {
        trace!("read 0x{:x}", address);
        let bit_len = bytes * 8;
        let mut symbolic = false;
        let mut bytes_vec = vec![];
        assert!(!tag);
        // TODO push to trace [as long as we don't use the requirements this doesn't matter]
        for byte in (0..bytes).rev() {
            let address = address + (byte as u64);
            match self.data.get(&(address)) {
                Some(d) => {
                    match d {
                        // TODO assert bv.len == 8
                        Val::Bits(bv) => {
                            assert!(bv.len() == 8);
                            solver.add_event(Event::ReadMem {
                                value: Val::Bits(*bv),
                                read_kind: read_kind.clone(),
                                address: Val::Bits(B::from_u64(address)),
                                bytes: 1,
                                tag_value: None,
                                kind: STABLE_MEMORY,
                            });
                            bytes_vec.push(bv.lower_u64() as u8)
                        }
                        _ => symbolic = true,
                    }
                }
                None => symbolic = true,
            }
        }

        if symbolic {
            let mut symbol = solver.define_const(Exp::Bits64(0, bytes * 8));
            let mut bsymbols = vec![];

            // Gather the individual bytes
            for byte in 0..bytes {
                let address = address + (byte as u64);
                match self.data.get(&(address)) {
                    Some(d) => match d {
                        Val::Bits(bv) => {
                            assert!(bv.len() == 8);
                            bsymbols.push(solver.define_const(Exp::Bits64(bv.lower_u64(), 8)));
                        }
                        Val::Symbolic(sym) => {
                            solver.add_event(Event::ReadMem {
                                value: Val::Symbolic(*sym),
                                read_kind: read_kind.clone(),
                                address: Val::Bits(B::from_u64(address)),
                                bytes: 1,
                                tag_value: None,
                                kind: STABLE_MEMORY,
                            });
                            bsymbols.push(*sym);
                        }
                        _ => panic!(),
                    },
                    None => {
                        bsymbols.push(solver.declare_const(Ty::BitVec(8)));
                    }
                }
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
                // TODO we have to or with a zero-initialized BV
                let or_symbol = Exp::Bvor(Box::new(shift_bsymbol), Box::new(Exp::Var(symbol)));
                symbol = solver.define_const(or_symbol);
            }
            Ok(Val::Symbolic(symbol))
        } else {
            Ok(Val::Bits(B::from_bytes(&bytes_vec)))
        }
    }

    fn write(
        &mut self,
        write_kind: Val<B>,
        address: Address,
        data: Val<B>,
        solver: &mut Solver<B>,
        tag_value: Option<Val<B>>,
    ) -> Result<Val<B>, ExecError> {
        match data {
            Val::Bits(bv) => {
                let bytes_vec = bv.to_le_bytes();
                for i in 0..bytes_vec.len() {
                    let address = address + (i as u64);
                    let data = Val::Bits(B::new(bytes_vec[i] as u64, 8));
                    self.data.insert(address, data.clone());
                    let value = solver.fresh(); // TODO wtf is this
                    solver.add_event(Event::WriteMem {
                        value,
                        write_kind: write_kind.clone(),
                        address: Val::Bits(B::from_u64(address)),
                        data,
                        bytes: 1,
                        tag_value: tag_value.clone(),
                        kind: STABLE_MEMORY,
                    });
                }
                Ok(Val::Unit)
            }
            Val::Symbolic(sym) => {
                let len = solver.length(sym).unwrap();
                assert!(len % 8 == 0);
                let len = len / 8;
                for i in 0..len {
                    let address = address + (i as u64);
                    let byte_symbol = solver.define_const(Exp::Extract(
                        (i * 8) + 7,
                        i * 8,
                        Box::new(Exp::Var(sym)),
                    ));
                    assert!(solver.length(byte_symbol).unwrap() == 8);
                    let data = Val::Symbolic(byte_symbol);
                    let value = solver.fresh(); // TODO wtf is this
                    solver.add_event(Event::WriteMem {
                        value,
                        write_kind: write_kind.clone(),
                        address: Val::Bits(B::from_u64(address)),
                        data,
                        bytes: 1,
                        tag_value: tag_value.clone(),
                        kind: STABLE_MEMORY,
                    });
                }
                Ok(Val::Unit)
            }
            _ => panic!(),
        }
    }

    fn initial_value(&self, _address: Address, _bytes: u32) -> Option<B> {
        unimplemented!()
    }

    fn clone_dyn(&self) -> Box<dyn Send + Sync + CustomRegion<B>> {
        Box::new(self.clone())
    }

    fn memory_kind(&self) -> &'static str {
        STABLE_MEMORY
    }
}
