#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Central Transmit Status Register"]
    pub txsr: crate::Reg<txsr::TXSR_SPEC>,
    #[doc = "0x04 - CAN Central Receive Status Register"]
    pub rxsr: crate::Reg<rxsr::RXSR_SPEC>,
    #[doc = "0x08 - CAN Central Miscellaneous Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
}
#[doc = "TXSR register accessor: an alias for `Reg<TXSR_SPEC>`"]
pub type TXSR = crate::Reg<txsr::TXSR_SPEC>;
#[doc = "CAN Central Transmit Status Register"]
pub mod txsr;
#[doc = "RXSR register accessor: an alias for `Reg<RXSR_SPEC>`"]
pub type RXSR = crate::Reg<rxsr::RXSR_SPEC>;
#[doc = "CAN Central Receive Status Register"]
pub mod rxsr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "CAN Central Miscellaneous Register"]
pub mod msr;
