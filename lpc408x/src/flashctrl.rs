#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: crate::Reg<fmsstart::FMSSTART_SPEC>,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: crate::Reg<fmsstop::FMSSTOP_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x2c - 128-bit signature Word 0"]
    pub fmsw0: crate::Reg<fmsw0::FMSW0_SPEC>,
    #[doc = "0x30 - 128-bit signature Word 1"]
    pub fmsw1: crate::Reg<fmsw1::FMSW1_SPEC>,
    #[doc = "0x34 - 128-bit signature Word 2"]
    pub fmsw2: crate::Reg<fmsw2::FMSW2_SPEC>,
    #[doc = "0x38 - 128-bit signature Word 3"]
    pub fmsw3: crate::Reg<fmsw3::FMSW3_SPEC>,
    _reserved6: [u8; 0x44],
    #[doc = "0x80 - EEPROM command register"]
    pub eecmd: crate::Reg<eecmd::EECMD_SPEC>,
    #[doc = "0x84 - EEPROM address register"]
    pub eeaddr: crate::Reg<eeaddr::EEADDR_SPEC>,
    #[doc = "0x88 - EEPROM write data register"]
    pub eewdata: crate::Reg<eewdata::EEWDATA_SPEC>,
    #[doc = "0x8c - EEPROM read data register"]
    pub eerdata: crate::Reg<eerdata::EERDATA_SPEC>,
    #[doc = "0x90 - EEPROM wait state register"]
    pub eewstate: crate::Reg<eewstate::EEWSTATE_SPEC>,
    #[doc = "0x94 - EEPROM clock divider register"]
    pub eeclkdiv: crate::Reg<eeclkdiv::EECLKDIV_SPEC>,
    #[doc = "0x98 - EEPROM power-down register"]
    pub eepwrdwn: crate::Reg<eepwrdwn::EEPWRDWN_SPEC>,
    _reserved13: [u8; 0x0f3c],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub enclr: crate::Reg<enclr::ENCLR_SPEC>,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub enset: crate::Reg<enset::ENSET_SPEC>,
    #[doc = "0xfe0 - Signature generation status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub statclr: crate::Reg<statclr::STATCLR_SPEC>,
}
#[doc = "FMSSTART register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "FMSW0 register accessor: an alias for `Reg<FMSW0_SPEC>`"]
pub type FMSW0 = crate::Reg<fmsw0::FMSW0_SPEC>;
#[doc = "128-bit signature Word 0"]
pub mod fmsw0;
#[doc = "FMSW1 register accessor: an alias for `Reg<FMSW1_SPEC>`"]
pub type FMSW1 = crate::Reg<fmsw1::FMSW1_SPEC>;
#[doc = "128-bit signature Word 1"]
pub mod fmsw1;
#[doc = "FMSW2 register accessor: an alias for `Reg<FMSW2_SPEC>`"]
pub type FMSW2 = crate::Reg<fmsw2::FMSW2_SPEC>;
#[doc = "128-bit signature Word 2"]
pub mod fmsw2;
#[doc = "FMSW3 register accessor: an alias for `Reg<FMSW3_SPEC>`"]
pub type FMSW3 = crate::Reg<fmsw3::FMSW3_SPEC>;
#[doc = "128-bit signature Word 3"]
pub mod fmsw3;
#[doc = "EECMD register accessor: an alias for `Reg<EECMD_SPEC>`"]
pub type EECMD = crate::Reg<eecmd::EECMD_SPEC>;
#[doc = "EEPROM command register"]
pub mod eecmd;
#[doc = "EEADDR register accessor: an alias for `Reg<EEADDR_SPEC>`"]
pub type EEADDR = crate::Reg<eeaddr::EEADDR_SPEC>;
#[doc = "EEPROM address register"]
pub mod eeaddr;
#[doc = "EEWDATA register accessor: an alias for `Reg<EEWDATA_SPEC>`"]
pub type EEWDATA = crate::Reg<eewdata::EEWDATA_SPEC>;
#[doc = "EEPROM write data register"]
pub mod eewdata;
#[doc = "EERDATA register accessor: an alias for `Reg<EERDATA_SPEC>`"]
pub type EERDATA = crate::Reg<eerdata::EERDATA_SPEC>;
#[doc = "EEPROM read data register"]
pub mod eerdata;
#[doc = "EEWSTATE register accessor: an alias for `Reg<EEWSTATE_SPEC>`"]
pub type EEWSTATE = crate::Reg<eewstate::EEWSTATE_SPEC>;
#[doc = "EEPROM wait state register"]
pub mod eewstate;
#[doc = "EECLKDIV register accessor: an alias for `Reg<EECLKDIV_SPEC>`"]
pub type EECLKDIV = crate::Reg<eeclkdiv::EECLKDIV_SPEC>;
#[doc = "EEPROM clock divider register"]
pub mod eeclkdiv;
#[doc = "EEPWRDWN register accessor: an alias for `Reg<EEPWRDWN_SPEC>`"]
pub type EEPWRDWN = crate::Reg<eepwrdwn::EEPWRDWN_SPEC>;
#[doc = "EEPROM power-down register"]
pub mod eepwrdwn;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Signature generation status register"]
pub mod stat;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "STATCLR register accessor: an alias for `Reg<STATCLR_SPEC>`"]
pub type STATCLR = crate::Reg<statclr::STATCLR_SPEC>;
#[doc = "Signature generation status clear register"]
pub mod statclr;
#[doc = "ENCLR register accessor: an alias for `Reg<ENCLR_SPEC>`"]
pub type ENCLR = crate::Reg<enclr::ENCLR_SPEC>;
#[doc = "EEPROM interrupt enable clear"]
pub mod enclr;
#[doc = "ENSET register accessor: an alias for `Reg<ENSET_SPEC>`"]
pub type ENSET = crate::Reg<enset::ENSET_SPEC>;
#[doc = "EEPROM interrupt enable set"]
pub mod enset;
