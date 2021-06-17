#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register."]
    pub pwr: crate::Reg<pwr::PWR_SPEC>,
    #[doc = "0x04 - Clock control register."]
    pub clock: crate::Reg<clock::CLOCK_SPEC>,
    #[doc = "0x08 - Argument register."]
    pub argument: crate::Reg<argument::ARGUMENT_SPEC>,
    #[doc = "0x0c - Command register."]
    pub command: crate::Reg<command::COMMAND_SPEC>,
    #[doc = "0x10 - Response command register."]
    pub respcmd: crate::Reg<respcmd::RESPCMD_SPEC>,
    #[doc = "0x14..0x24 - Response register."]
    pub response: [crate::Reg<response::RESPONSE_SPEC>; 4],
    #[doc = "0x24 - Data Timer."]
    pub datatimer: crate::Reg<datatimer::DATATIMER_SPEC>,
    #[doc = "0x28 - Data length register."]
    pub datalength: crate::Reg<datalength::DATALENGTH_SPEC>,
    #[doc = "0x2c - Data control register."]
    pub datactrl: crate::Reg<datactrl::DATACTRL_SPEC>,
    #[doc = "0x30 - Data counter."]
    pub datacnt: crate::Reg<datacnt::DATACNT_SPEC>,
    #[doc = "0x34 - Status register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x38 - Clear register."]
    pub clear: crate::Reg<clear::CLEAR_SPEC>,
    #[doc = "0x3c - Interrupt 0 mask register."]
    pub mask0: crate::Reg<mask0::MASK0_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x48 - FIFO Counter."]
    pub fifocnt: crate::Reg<fifocnt::FIFOCNT_SPEC>,
    _reserved14: [u8; 0x34],
    #[doc = "0x80..0xc0 - Data FIFO Register."]
    pub fifo: [crate::Reg<fifo::FIFO_SPEC>; 16],
}
#[doc = "PWR register accessor: an alias for `Reg<PWR_SPEC>`"]
pub type PWR = crate::Reg<pwr::PWR_SPEC>;
#[doc = "Power control register."]
pub mod pwr;
#[doc = "CLOCK register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock control register."]
pub mod clock;
#[doc = "ARGUMENT register accessor: an alias for `Reg<ARGUMENT_SPEC>`"]
pub type ARGUMENT = crate::Reg<argument::ARGUMENT_SPEC>;
#[doc = "Argument register."]
pub mod argument;
#[doc = "COMMAND register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command register."]
pub mod command;
#[doc = "RESPCMD register accessor: an alias for `Reg<RESPCMD_SPEC>`"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
#[doc = "Response command register."]
pub mod respcmd;
#[doc = "RESPONSE register accessor: an alias for `Reg<RESPONSE_SPEC>`"]
pub type RESPONSE = crate::Reg<response::RESPONSE_SPEC>;
#[doc = "Response register."]
pub mod response;
#[doc = "DATATIMER register accessor: an alias for `Reg<DATATIMER_SPEC>`"]
pub type DATATIMER = crate::Reg<datatimer::DATATIMER_SPEC>;
#[doc = "Data Timer."]
pub mod datatimer;
#[doc = "DATALENGTH register accessor: an alias for `Reg<DATALENGTH_SPEC>`"]
pub type DATALENGTH = crate::Reg<datalength::DATALENGTH_SPEC>;
#[doc = "Data length register."]
pub mod datalength;
#[doc = "DATACTRL register accessor: an alias for `Reg<DATACTRL_SPEC>`"]
pub type DATACTRL = crate::Reg<datactrl::DATACTRL_SPEC>;
#[doc = "Data control register."]
pub mod datactrl;
#[doc = "DATACNT register accessor: an alias for `Reg<DATACNT_SPEC>`"]
pub type DATACNT = crate::Reg<datacnt::DATACNT_SPEC>;
#[doc = "Data counter."]
pub mod datacnt;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register."]
pub mod status;
#[doc = "CLEAR register accessor: an alias for `Reg<CLEAR_SPEC>`"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = "Clear register."]
pub mod clear;
#[doc = "MASK0 register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Interrupt 0 mask register."]
pub mod mask0;
#[doc = "FIFOCNT register accessor: an alias for `Reg<FIFOCNT_SPEC>`"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "FIFO Counter."]
pub mod fifocnt;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data FIFO Register."]
pub mod fifo;
