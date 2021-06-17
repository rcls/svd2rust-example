#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions."]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x0c - Prescale Register. Determines how often the PWM counter is incremented."]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x10 - Prescale Counter. Prescaler for the main PWM counter."]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0x14 - Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x18..0x28 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr: [crate::Reg<mr::MR_SPEC>; 4],
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x2c..0x34 - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub cr: [crate::Reg<cr::CR_SPEC>; 2],
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr4: crate::Reg<mr4::MR4_SPEC>,
    #[doc = "0x44 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr5: crate::Reg<mr5::MR5_SPEC>,
    #[doc = "0x48 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr6: crate::Reg<mr6::MR6_SPEC>,
    #[doc = "0x4c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x50 - Load Enable Register. Enables use of updated PWM match values."]
    pub ler: crate::Reg<ler::LER_SPEC>,
    _reserved14: [u8; 0x1c],
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub ctcr: crate::Reg<ctcr::CTCR_SPEC>,
}
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub mod ir;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub mod tcr;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub mod pr;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub mod pc;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub mod mcr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub mod ccr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod cr;
#[doc = "MR4 register accessor: an alias for `Reg<MR4_SPEC>`"]
pub type MR4 = crate::Reg<mr4::MR4_SPEC>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr4;
#[doc = "MR5 register accessor: an alias for `Reg<MR5_SPEC>`"]
pub type MR5 = crate::Reg<mr5::MR5_SPEC>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr5;
#[doc = "MR6 register accessor: an alias for `Reg<MR6_SPEC>`"]
pub type MR6 = crate::Reg<mr6::MR6_SPEC>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr6;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod pcr;
#[doc = "LER register accessor: an alias for `Reg<LER_SPEC>`"]
pub type LER = crate::Reg<ler::LER_SPEC>;
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub mod ler;
#[doc = "CTCR register accessor: an alias for `Reg<CTCR_SPEC>`"]
pub type CTCR = crate::Reg<ctcr::CTCR_SPEC>;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
