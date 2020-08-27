enum MIPSArguments {
    MemoryAddr(),
    Register(MIPSRegister)
}

pub struct MIPSCodeInstruction {
    args: Vec<MIPSArguments>,
    run_func: fn(Vec<MIPSArguments>, MIPSRuntime) -> MIPSRuntime
}
