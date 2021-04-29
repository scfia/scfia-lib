pub mod handle_read_mem;
pub mod handle_read_reg;
pub mod handle_smt;
pub mod handle_write_mem;
pub mod handle_write_reg;
pub mod isla_handle;
pub mod parse_trace;

const IGNORED_REGISTERS: &'static [&'static str] = &[
    "z__unconditional",
    "zSEE",
    "zShouldAdvanceIT",
    "z__PC_changed",
    "z__currentInstrLength",
    "z__currentInstr",
    "z__highest_el_aarch32",
    "z__trickbox_enabled",
    "z__v82_implemented",
    "z__v83_implemented",
    "z__v84_implemented",
    "z__v85_implemented",
    "z_PendingPhysicalSE",
    "z_IRQPending",
    "z_FIQPending",
    "zDBGEN",
    "zOSLSR_EL1", // OS Lock Status Register
    "zEDSCR",     // External Debug Status and Control Register
    "zOSDLR_EL1", // OS Double Lock Register
    "zMPIDR_EL1",
    "zBTypeNext",
];
