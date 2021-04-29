use std::{collections::HashMap, fs, path::PathBuf, sync::Arc, time::Instant};

use crossbeam::queue::SegQueue;
use isla_lib::{
    concrete::{bitvector64::B64, BV},
    config::ISAConfig,
    error::ExecError,
    executor::{self, Backtrace, LocalFrame, Task, TaskState},
    init::{self, Initialized},
    ir::{serialize, AssertionMode, Name, SharedState, Symtab, UVal, Val},
    memory::Region,
    smt::{
        self,
        smtlib::{Def, Exp, Ty},
        Checkpoint, Config, Context, Model, Solver, Sym,
    },
};
use lazy_static::lazy_static;
use log::{debug, info, trace};

use crate::{
    isla::memory::{
        isla_stable_memory_region::IslaStableMemoryRegion,
        isla_volatile_memory_region::IslaVolatileMemoryRegion,
    },
    symbols::scfia_symbol::ScfiaSymbolId,
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::aarch64::transformation_function::{
        ScfiaAarch64TransformationFunction, ScfiaAarch64TransformationFunctionSuccessorDefinition,
    },
    values::value8::Value8,
};

use super::parse_trace::aarch64_parse_trace;

lazy_static! {
    pub static ref AARCH64_ISLA_HANDLE: Aarch64IslaHandle = unsafe { Aarch64IslaHandle::new() };
}

pub struct Aarch64IslaHandle {
    pub shared_state: &'static SharedState<'static, B64>,
    pub task: Task<'static, 'static, B64>,
    pub task_state: &'static TaskState<B64>,
}

impl Aarch64IslaHandle {
    pub unsafe fn new() -> Self {
        let now = Instant::now();
        let folder =
            PathBuf::from(r"..\verification\sail-arm\1166c197b127ed30d95421dcfa5fc59716aa1368");
        let config_file = folder.join("aarch64.toml");
        let symtab_file = folder.join("aarch64.symtab");
        let ir_file = folder.join("aarch64.irx");

        let strings: &Vec<String> = Box::leak(Box::new(
            bincode::deserialize(&fs::read(&symtab_file).unwrap()).unwrap(),
        ));
        let symtab = Symtab::from_raw_table(&strings);
        let ir: &mut Vec<isla_lib::ir::Def<Name, B64>> = Box::leak(Box::new(
            serialize::deserialize(&fs::read(&ir_file).unwrap()).expect("Failed to deserialize IR"),
        ));
        let isa_config: ISAConfig<B64> =
            ISAConfig::parse(&fs::read_to_string(&config_file).unwrap(), &symtab).unwrap();
        trace!("Loaded architecture in: {}ms", now.elapsed().as_millis());

        let Initialized {
            regs,
            lets,
            shared_state,
        } = init::initialize_architecture(ir, symtab, &isa_config, AssertionMode::Optimistic);
        let shared_state = Box::leak(Box::new(shared_state));
        let task_state = Box::leak(Box::new(TaskState::new()));
        let step_cpu = shared_state.symtab.lookup("zStep_CPU");
        let take_reset = shared_state.symtab.lookup("zTakeReset");
        let (reset_args, _, reset_instrs) = shared_state.functions.get(&take_reset).unwrap();
        let (_step_args, _, step_cpu_instrs) = shared_state.functions.get(&step_cpu).unwrap();

        let vals = vec![Val::Bool(true)];
        let mut lf: LocalFrame<B64> =
            LocalFrame::new(take_reset, reset_args, Some(&vals), reset_instrs);
        lf.add_lets(&lets);
        lf.add_regs(&regs);

        let mut task = lf.task(0, task_state);
        task = Self::step_no_fork(task, task_state, &shared_state); // reset because isla doesn't work nicely otherwise
                                                                    //print_register(&task.frame, &shared_state.symtab, "zPSTATE");
        lf = executor::unfreeze_frame(&task.frame);
        lf.function_name = step_cpu;
        lf.instrs = step_cpu_instrs;
        task.frame = executor::freeze_frame(&lf);

        Aarch64IslaHandle {
            shared_state: shared_state,
            task: task,
            task_state,
        }
    }

    fn step<'ir, 'task>(
        task: isla_lib::executor::Task<'ir, 'task, B64>,
        task_state: &'static TaskState<B64>,
        shared_state: &isla_lib::ir::SharedState<'ir, B64>,
    ) -> Vec<Task<'ir, 'task, B64>> {
        // isla_lib::log::set_flags(0xffffffff);
        //print_registers(&task.frame, &shared_state.symtab);
        let queue = Arc::new(SegQueue::new());
        executor::start_multi(
            1,
            None,
            vec![task],
            shared_state,
            queue.clone(),
            &simple_collector,
        );
        let mut successors = vec![];
        loop {
            match queue.pop() {
                Ok(Ok((mut local_frame, checkpoint))) => {
                    local_frame.pc = 0;
                    let frame = executor::freeze_frame(&local_frame);
                    successors.push(executor::Task {
                        id: 42,
                        state: task_state,
                        frame: frame,
                        checkpoint: checkpoint,
                        fork_cond: None,
                        stop_functions: None,
                    });
                }
                Ok(Err((error, backtrace))) => {
                    panic!(
                        "queue got error: {}",
                        error.to_string(&backtrace, &shared_state)
                    );
                }
                Err(_) => break,
            }
        }
        successors
    }

    fn step_no_fork<'ir, 'task>(
        task: executor::Task<'ir, 'task, B64>,
        task_state: &'static TaskState<B64>,
        shared_state: &SharedState<'ir, B64>,
    ) -> executor::Task<'ir, 'task, B64> {
        let mut succs = Self::step(task, task_state, &shared_state);
        if succs.len() > 1 {
            panic!("single_step_no_fork forked")
        }
        succs.remove(0)
    }

    fn apply_aarch64_state_to_local_frame(
        &self,
        state: &ScfiaAarch64SystemState,
        local_frame: &mut LocalFrame<B64>,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) {
        trace!("apply_aarch64_state_to_local_frame()");
        {
            // TODO remove those dirty hacks
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zDBGEN"),
                UVal::Init(isla_lib::ir::Val::Enum(isla_lib::ir::EnumMember {
                    // DBGEN = LOW
                    enum_id: 3,
                    member: 0,
                })),
            );

            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("z_IRQPending"),
                isla_lib::ir::UVal::Init(isla_lib::ir::Val::Bool(false)),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("z_FIQPending"),
                isla_lib::ir::UVal::Init(isla_lib::ir::Val::Bool(false)),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zCPTR_EL2"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::from_u64(0))),
            ); // Architectural Feature Trap Register
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zCPTR_EL3"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::from_u64(0))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("z__defaultRAM"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::new(4096, 56))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zDBGCLAIMCLR_EL1"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::new(0, 32))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zDBGCLAIMSET_EL1"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::new(0, 32))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zID_AA64PFR0_EL1"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::from_u64(0))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zRMR_EL3"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::from_u64(1))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zCFG_ID_AA64PFR0_EL1_EL0"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::new(2, 4))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("z__highest_el_aarch32"),
                isla_lib::ir::UVal::Init(isla_lib::ir::Val::Bool(false)),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("z__v84_implemented"),
                isla_lib::ir::UVal::Init(isla_lib::ir::Val::Bool(false)),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zCNTCR"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::new(0, 32))),
            );
            local_frame.regs_mut().insert(
                self.shared_state.symtab.lookup("zSCR_EL3"),
                isla_lib::ir::UVal::Init(Val::Bits(B64::from_u64(0))),
            );
        }

        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR0"),
            UVal::Init(state.r0.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR1"),
            UVal::Init(state.r1.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR2"),
            UVal::Init(state.r2.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR3"),
            UVal::Init(state.r3.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR4"),
            UVal::Init(state.r4.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR5"),
            UVal::Init(state.r5.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR6"),
            UVal::Init(state.r6.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR7"),
            UVal::Init(state.r7.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR8"),
            UVal::Init(state.r8.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR9"),
            UVal::Init(state.r9.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR10"),
            UVal::Init(state.r10.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR11"),
            UVal::Init(state.r11.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR12"),
            UVal::Init(state.r12.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR13"),
            UVal::Init(state.r13.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR14"),
            UVal::Init(state.r14.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR15"),
            UVal::Init(state.r15.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR16"),
            UVal::Init(state.r16.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR17"),
            UVal::Init(state.r17.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR18"),
            UVal::Init(state.r18.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR19"),
            UVal::Init(state.r19.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR20"),
            UVal::Init(state.r20.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR21"),
            UVal::Init(state.r21.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR22"),
            UVal::Init(state.r22.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR23"),
            UVal::Init(state.r23.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR24"),
            UVal::Init(state.r24.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR25"),
            UVal::Init(state.r25.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR26"),
            UVal::Init(state.r26.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR27"),
            UVal::Init(state.r7.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR28"),
            UVal::Init(state.r28.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR29"),
            UVal::Init(state.fp.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );
        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zR30"),
            UVal::Init(state.lr.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );

        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("z_PC"),
            UVal::Init(Val::Bits(B64::new(state.pc, 64))),
        );

        let mut isla_pstate = HashMap::new();
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zN"),
            state.pstate.n.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            ),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zZ"),
            state.pstate.z.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            ),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zC"),
            state.pstate.c.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            ),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zV"),
            state.pstate.v.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            ),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zPAN"),
            Val::Bits(B64::new(state.pstate.pan as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zSSBS"),
            Val::Bits(B64::new(state.pstate.ssbs as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zTCO"),
            Val::Bits(B64::new(state.pstate.tco as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zUAO"),
            Val::Bits(B64::new(state.pstate.uao as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zI"),
            Val::Bits(B64::new(state.pstate.i as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zDIT"),
            Val::Bits(B64::new(state.pstate.dit as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zSP"),
            Val::Bits(B64::new(state.pstate.sp as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zIT"),
            Val::Bits(B64::new(state.pstate.it as u64, 8)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zT"),
            Val::Bits(B64::new(state.pstate.t as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zF"),
            Val::Bits(B64::new(state.pstate.f as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zD"),
            Val::Bits(B64::new(state.pstate.d as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("znRW"),
            Val::Bits(B64::new(state.pstate.nrw as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zBTYPE"),
            Val::Bits(B64::new(state.pstate.btype as u64, 2)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zJ"),
            Val::Bits(B64::new(state.pstate.j as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zE"),
            Val::Bits(B64::new(state.pstate.e as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zEL"),
            Val::Bits(B64::new(3, 2)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zQ"),
            Val::Bits(B64::new(state.pstate.q as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zSS"),
            Val::Bits(B64::new(state.pstate.ss as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zM"),
            Val::Bits(B64::new(state.pstate.m as u64, 5)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zA"),
            Val::Bits(B64::new(state.pstate.a as u64, 1)),
        );
        isla_pstate.insert(
            self.shared_state.symtab.lookup("zIL"),
            Val::Bits(B64::new(state.pstate.il as u64, 1)),
        );

        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zPSTATE"),
            UVal::Init(isla_lib::ir::Val::Struct(isla_pstate)),
        );

        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zSCTLR_EL3"),
            UVal::Init(Val::Bits(B64::new(state.sctlr_el3, 64))),
        );

        local_frame.regs_mut().insert(
            self.shared_state.symtab.lookup("zSP_EL3"),
            UVal::Init(state.sp_el3.to_isla_val(
                &state.symbol_manager,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        );

        for region in &state.memory.stable_regions {
            let mut isla_custom_region = IslaStableMemoryRegion {
                data: HashMap::new(),
            };
            for address in region.range.start..region.range.end {
                isla_custom_region.data.insert(
                    address,
                    region.data.get(&address).unwrap().to_isla_val(
                        &state.symbol_manager,
                        solver,
                        scfia_symbol_to_isla_sym_mapping,
                    ),
                );
            }
            local_frame.memory_mut().add_region(Region::Custom(
                region.range.clone(),
                Box::new(isla_custom_region),
            ))
        }

        for region in &state.memory.volatile_regions {
            local_frame.memory_mut().add_region(Region::Custom(
                region.range.clone(),
                Box::new(IslaVolatileMemoryRegion {}),
            ))
        }

        trace!("apply_aarch64_state_to_local_frame() finished");
    }

    pub fn monomorphize_u64_scfia_symbol(
        &self,
        state: &ScfiaAarch64SystemState,
        symbol_id: ScfiaSymbolId,
    ) -> Vec<u64> {
        trace!("monomorphize_u64_scfia_symbol {} {:?}", symbol_id, state.r4);
        let mut local_frame = executor::unfreeze_frame(&self.task.frame);
        let cfg = Config::new();
        let ctx = Context::new(cfg);

        let mut solver = Solver::from_checkpoint(&ctx, self.task.checkpoint.clone());
        let mut scfia_symbol_to_isla_sym_mapping: HashMap<ScfiaSymbolId, Sym> = HashMap::new();
        self.apply_aarch64_state_to_local_frame(
            state,
            &mut local_frame,
            &mut solver,
            &mut scfia_symbol_to_isla_sym_mapping,
        );

        let mut candidates = vec![];
        trace!("{:?}", scfia_symbol_to_isla_sym_mapping.keys());
        let mut exp = Exp::Bool(true);
        let sym = solver.declare_const(Ty::BitVec(64));
        solver.add(Def::Assert(Exp::Eq(
            Box::new(Exp::Var(
                *scfia_symbol_to_isla_sym_mapping.get(&symbol_id).unwrap(),
            )),
            Box::new(Exp::Var(sym)),
        )));
        while solver.check_sat_with(&exp).is_sat().unwrap() {
            let mut model = Model::new(&solver);
            match model.get_var(sym) {
                Ok(Some(Exp::Bits64(model_u64, size))) => {
                    assert_eq!(size, 64);
                    trace!("found candidate {}", model_u64);
                    candidates.push(model_u64);
                    exp = Exp::And(
                        Box::new(exp),
                        Box::new(Exp::Neq(
                            Box::new(Exp::Var(sym)),
                            Box::new(Exp::Bits64(model_u64, 64)),
                        )),
                    )
                }
                x => panic!("{:?}", &x),
            }
        }
        candidates
    }

    pub fn monomorphize_u1_scfia_symbol(
        &self,
        state: &ScfiaAarch64SystemState,
        symbol_id: ScfiaSymbolId,
    ) -> Vec<u64> {
        trace!("monomorphize_u1_scfia_symbol {} {:?}", symbol_id, state.r4);
        let mut local_frame = executor::unfreeze_frame(&self.task.frame);
        let cfg = Config::new();
        let ctx = Context::new(cfg);

        let mut solver = Solver::from_checkpoint(&ctx, self.task.checkpoint.clone());
        let mut scfia_symbol_to_isla_sym_mapping: HashMap<ScfiaSymbolId, Sym> = HashMap::new();
        self.apply_aarch64_state_to_local_frame(
            state,
            &mut local_frame,
            &mut solver,
            &mut scfia_symbol_to_isla_sym_mapping,
        );

        let mut candidates = vec![];
        trace!("{:?}", scfia_symbol_to_isla_sym_mapping.keys());
        let mut exp = Exp::Bool(true);
        let sym = solver.declare_const(Ty::BitVec(1));
        solver.add(Def::Assert(Exp::Eq(
            Box::new(Exp::Var(
                *scfia_symbol_to_isla_sym_mapping.get(&symbol_id).unwrap(),
            )),
            Box::new(Exp::Var(sym)),
        )));
        while solver.check_sat_with(&exp).is_sat().unwrap() {
            let mut model = Model::new(&solver);
            match model.get_var(sym) {
                Ok(Some(Exp::Bits64(model_u64, size))) => {
                    assert_eq!(size, 1);
                    trace!("found candidate {}", model_u64);
                    candidates.push(model_u64);
                    exp = Exp::And(
                        Box::new(exp),
                        Box::new(Exp::Neq(
                            Box::new(Exp::Var(sym)),
                            Box::new(Exp::Bits64(model_u64, 1)),
                        )),
                    )
                }
                x => panic!("{:?}", &x),
            }
        }
        candidates
    }

    pub fn determine_symbol_offset(&self, state: &ScfiaAarch64SystemState, base_symbol: ScfiaSymbolId, symbol: ScfiaSymbolId) -> u64 {
        let mut local_frame = executor::unfreeze_frame(&self.task.frame);
        let cfg = Config::new();
        let ctx = Context::new(cfg);

        let mut solver = Solver::from_checkpoint(&ctx, self.task.checkpoint.clone());
        let mut scfia_symbol_to_isla_sym_mapping: HashMap<ScfiaSymbolId, Sym> = HashMap::new();
        self.apply_aarch64_state_to_local_frame(
            state,
            &mut local_frame,
            &mut solver,
            &mut scfia_symbol_to_isla_sym_mapping,
        );

        let mut candidates = vec![];
        trace!("{:?}", scfia_symbol_to_isla_sym_mapping.keys());
        let mut exp = Exp::Bool(true);
        let sym = solver.declare_const(Ty::BitVec(64));
        // sym == symbol - base_symbol
        solver.add(Def::Assert(Exp::Eq(
            Box::new(Exp::Bvsub(
                Box::new(Exp::Var(
                    *scfia_symbol_to_isla_sym_mapping.get(&symbol).unwrap(),
                )),
                Box::new(Exp::Var(
                    *scfia_symbol_to_isla_sym_mapping.get(&base_symbol).unwrap(),
                ))
            )),
            Box::new(Exp::Var(sym)),
        )));
        while solver.check_sat_with(&exp).is_sat().unwrap() {
            let mut model = Model::new(&solver);
            match model.get_var(sym) {
                Ok(Some(Exp::Bits64(model_u64, size))) => {
                    assert_eq!(size, 64);
                    trace!("found candidate {}", model_u64);
                    candidates.push(model_u64);
                    exp = Exp::And(
                        Box::new(exp),
                        Box::new(Exp::Neq(
                            Box::new(Exp::Var(sym)),
                            Box::new(Exp::Bits64(model_u64, 64)),
                        )),
                    )
                }
                x => panic!("{:?}", &x),
            }
        }
        assert_eq!(candidates.len(), 1);
        debug!("found offset {}", candidates[0]);
        candidates[0]
    }

    pub fn derive_ttf(
        &self,
        state: &ScfiaAarch64SystemState,
    ) -> ScfiaAarch64TransformationFunction {
        trace!("derive_ttf");
        let mut local_frame = executor::unfreeze_frame(&self.task.frame);

        let cfg = Config::new();
        let ctx = Context::new(cfg);

        let mut solver = Solver::from_checkpoint(&ctx, self.task.checkpoint.clone());
        let mut scfia_symbol_to_isla_sym_mapping: HashMap<ScfiaSymbolId, Sym> = HashMap::new();
        self.apply_aarch64_state_to_local_frame(
            state,
            &mut local_frame,
            &mut solver,
            &mut scfia_symbol_to_isla_sym_mapping,
        );

        // Let isla-lib do its thing
        let checkpoint = smt::checkpoint(&mut solver);
        let now = Instant::now();
        let isla_successors = Self::step(
            local_frame.task_with_checkpoint(0, &self.task_state, checkpoint.clone()),
            self.task_state,
            &self.shared_state,
        );
        info!(
            "isla step took {} with {} translated scfia symbols",
            now.elapsed().as_millis(),
            scfia_symbol_to_isla_sym_mapping.len()
        );

        // Examine the isla successors
        let mut successor_definitions = vec![];

        for isla_successor in isla_successors {
            let cfg = Config::new();
            let ctx = Context::new(cfg);
            successor_definitions.push(aarch64_parse_trace(
                &state,
                &scfia_symbol_to_isla_sym_mapping,
                &(*isla_successor.checkpoint.trace).as_ref().unwrap(),
                self.shared_state,
                (*checkpoint.trace).as_ref().unwrap(),
                &mut Solver::from_checkpoint(&ctx, isla_successor.checkpoint.clone()),
            ));
        }

        ScfiaAarch64TransformationFunction {
            successors: successor_definitions,
        }
    }
}

pub type SimpleResultQueue<'ir, B> =
    SegQueue<Result<(LocalFrame<'ir, B>, Checkpoint<B>), (ExecError, Backtrace)>>;

fn simple_collector<'ir, B: BV>(
    _: usize,
    _task_id: usize,
    result: Result<(Val<B>, LocalFrame<'ir, B>), (ExecError, Backtrace)>,
    _shared_state: &SharedState<'ir, B>,
    mut solver: Solver<B>,
    collected: &SimpleResultQueue<'ir, B>,
) {
    match result {
        Ok((_, frame)) => collected.push(Ok((frame, smt::checkpoint(&mut solver)))),
        Err(e) => collected.push(Err(e)),
    }
}
