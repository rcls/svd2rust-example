#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSIOM port registers"]
    pub prt: crate::ArrayProxy<PRT, 15, 0x10>,
    _reserved1: [u8; 0x2000],
    #[doc = "0x2000..0x2100 - AMUX splitter cell control"]
    pub amux_split_ctl: [crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>; 64],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port selection 0"]
    pub port_sel0: crate::Reg<self::prt::port_sel0::PORT_SEL0_SPEC>,
    #[doc = "0x04 - Port selection 1"]
    pub port_sel1: crate::Reg<self::prt::port_sel1::PORT_SEL1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "AMUX_SPLIT_CTL register accessor: an alias for `Reg<AMUX_SPLIT_CTL_SPEC>`"]
pub type AMUX_SPLIT_CTL = crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>;
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;
