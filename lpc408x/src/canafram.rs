#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x800 - CAN AF ram access register"]
    pub mask: [crate::Reg<mask::MASK_SPEC>; 512],
}
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "CAN AF ram access register"]
pub mod mask;
