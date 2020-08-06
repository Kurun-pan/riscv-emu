pub struct Trap {
    pub factor: Traps,
    pub value: u64
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Traps {
    // Interrupts
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    /* Reserved for future standart use */
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    /* Reserved for future standart use */
    SupervisorTimerInterrupt,
    /* Reserved for future standart use */
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    /* Reserved for future standart use */
    MachineExternalInterrupt,

    // Exceptions
		InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    /* Reserved */
		EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    /* Reserved for future standart use */
		StorePageFault,
	}
