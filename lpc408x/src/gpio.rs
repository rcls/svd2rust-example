#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Direction control register."]
    pub dir0: crate::Reg<dir::DIR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Mask register for Port."]
    pub mask0: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x14 - Port Pin value register using MASK."]
    pub pin0: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x18 - Port Output Set register using MASK."]
    pub set0: crate::Reg<set::SET_SPEC>,
    #[doc = "0x1c - Port Output Clear register using MASK."]
    pub clr0: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0x20 - GPIO Port Direction control register."]
    pub dir1: crate::Reg<dir::DIR_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - Mask register for Port."]
    pub mask1: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x34 - Port Pin value register using MASK."]
    pub pin1: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x38 - Port Output Set register using MASK."]
    pub set1: crate::Reg<set::SET_SPEC>,
    #[doc = "0x3c - Port Output Clear register using MASK."]
    pub clr1: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0x40 - GPIO Port Direction control register."]
    pub dir2: crate::Reg<dir::DIR_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x50 - Mask register for Port."]
    pub mask2: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x54 - Port Pin value register using MASK."]
    pub pin2: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x58 - Port Output Set register using MASK."]
    pub set2: crate::Reg<set::SET_SPEC>,
    #[doc = "0x5c - Port Output Clear register using MASK."]
    pub clr2: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0x60 - GPIO Port Direction control register."]
    pub dir3: crate::Reg<dir::DIR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x70 - Mask register for Port."]
    pub mask3: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x74 - Port Pin value register using MASK."]
    pub pin3: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x78 - Port Output Set register using MASK."]
    pub set3: crate::Reg<set::SET_SPEC>,
    #[doc = "0x7c - Port Output Clear register using MASK."]
    pub clr3: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0x80 - GPIO Port Direction control register."]
    pub dir4: crate::Reg<dir::DIR_SPEC>,
    _reserved21: [u8; 0x0c],
    #[doc = "0x90 - Mask register for Port."]
    pub mask4: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x94 - Port Pin value register using MASK."]
    pub pin4: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x98 - Port Output Set register using MASK."]
    pub set4: crate::Reg<set::SET_SPEC>,
    #[doc = "0x9c - Port Output Clear register using MASK."]
    pub clr4: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0xa0 - GPIO Port Direction control register."]
    pub dir5: crate::Reg<dir::DIR_SPEC>,
    _reserved26: [u8; 0x0c],
    #[doc = "0xb0 - Mask register for Port."]
    pub mask5: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0xb4 - Port Pin value register using MASK."]
    pub pin5: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0xb8 - Port Output Set register using MASK."]
    pub set5: crate::Reg<set::SET_SPEC>,
    #[doc = "0xbc - Port Output Clear register using MASK."]
    pub clr5: crate::Reg<clr::CLR_SPEC>,
}
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO Port Direction control register."]
pub mod dir;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register for Port."]
pub mod mask;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port Pin value register using MASK."]
pub mod pin;
#[doc = "SET register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Port Output Set register using MASK."]
pub mod set;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Port Output Clear register using MASK."]
pub mod clr;
