pub struct Trap {
    pub exception: Exception,
    pub value: u64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    /* Reserved */
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    /* Reserved for future standart use */
    StorePageFault = 15,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    // Interrupts
    UserSoftware = 0,
    SupervisorSoftware = 1,
    /* Reserved for future standart use */
    MachineSoftware = 3,
    UserTimer = 4,
    /* Reserved for future standart use */
    SupervisorTimer = 5,
    /* Reserved for future standart use */
    MachineTimer = 7,
    UserExternal = 8,
    SupervisorExternal = 9,
    /* Reserved for future standart use */
    MachineExternal = 11,
}
