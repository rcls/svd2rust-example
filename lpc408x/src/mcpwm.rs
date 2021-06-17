#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control read address"]
    pub con: crate::Reg<con::CON_SPEC>,
    #[doc = "0x04 - PWM Control set address"]
    pub con_set: crate::Reg<con_set::CON_SET_SPEC>,
    #[doc = "0x08 - PWM Control clear address"]
    pub con_clr: crate::Reg<con_clr::CON_CLR_SPEC>,
    #[doc = "0x0c - Capture Control read address"]
    pub capcon: crate::Reg<capcon::CAPCON_SPEC>,
    #[doc = "0x10 - Capture Control set address"]
    pub capcon_set: crate::Reg<capcon_set::CAPCON_SET_SPEC>,
    #[doc = "0x14 - Event Control clear address"]
    pub capcon_clr: crate::Reg<capcon_clr::CAPCON_CLR_SPEC>,
    #[doc = "0x18..0x24 - Timer Counter register"]
    pub tc: [crate::Reg<tc::TC_SPEC>; 3],
    #[doc = "0x24..0x30 - Limit register"]
    pub lim: [crate::Reg<lim::LIM_SPEC>; 3],
    #[doc = "0x30..0x3c - Match register"]
    pub mat: [crate::Reg<mat::MAT_SPEC>; 3],
    #[doc = "0x3c - Dead time register"]
    pub dt: crate::Reg<dt::DT_SPEC>,
    #[doc = "0x40 - Communication Pattern register"]
    pub ccp: crate::Reg<ccp::CCP_SPEC>,
    #[doc = "0x44..0x50 - Capture register"]
    pub cap: [crate::Reg<cap::CAP_SPEC>; 3],
    #[doc = "0x50 - Interrupt Enable read address"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x54 - Interrupt Enable set address"]
    pub inten_set: crate::Reg<inten_set::INTEN_SET_SPEC>,
    #[doc = "0x58 - Interrupt Enable clear address"]
    pub inten_clr: crate::Reg<inten_clr::INTEN_CLR_SPEC>,
    #[doc = "0x5c - Count Control read address"]
    pub cntcon: crate::Reg<cntcon::CNTCON_SPEC>,
    #[doc = "0x60 - Count Control set address"]
    pub cntcon_set: crate::Reg<cntcon_set::CNTCON_SET_SPEC>,
    #[doc = "0x64 - Count Control clear address"]
    pub cntcon_clr: crate::Reg<cntcon_clr::CNTCON_CLR_SPEC>,
    #[doc = "0x68 - Interrupt flags read address"]
    pub intf: crate::Reg<intf::INTF_SPEC>,
    #[doc = "0x6c - Interrupt flags set address"]
    pub intf_set: crate::Reg<intf_set::INTF_SET_SPEC>,
    #[doc = "0x70 - Interrupt flags clear address"]
    pub intf_clr: crate::Reg<intf_clr::INTF_CLR_SPEC>,
    #[doc = "0x74 - Capture clear address"]
    pub cap_clr: crate::Reg<cap_clr::CAP_CLR_SPEC>,
}
#[doc = "CON register accessor: an alias for `Reg<CON_SPEC>`"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "PWM Control read address"]
pub mod con;
#[doc = "CON_SET register accessor: an alias for `Reg<CON_SET_SPEC>`"]
pub type CON_SET = crate::Reg<con_set::CON_SET_SPEC>;
#[doc = "PWM Control set address"]
pub mod con_set;
#[doc = "CON_CLR register accessor: an alias for `Reg<CON_CLR_SPEC>`"]
pub type CON_CLR = crate::Reg<con_clr::CON_CLR_SPEC>;
#[doc = "PWM Control clear address"]
pub mod con_clr;
#[doc = "CAPCON register accessor: an alias for `Reg<CAPCON_SPEC>`"]
pub type CAPCON = crate::Reg<capcon::CAPCON_SPEC>;
#[doc = "Capture Control read address"]
pub mod capcon;
#[doc = "CAPCON_SET register accessor: an alias for `Reg<CAPCON_SET_SPEC>`"]
pub type CAPCON_SET = crate::Reg<capcon_set::CAPCON_SET_SPEC>;
#[doc = "Capture Control set address"]
pub mod capcon_set;
#[doc = "CAPCON_CLR register accessor: an alias for `Reg<CAPCON_CLR_SPEC>`"]
pub type CAPCON_CLR = crate::Reg<capcon_clr::CAPCON_CLR_SPEC>;
#[doc = "Event Control clear address"]
pub mod capcon_clr;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter register"]
pub mod tc;
#[doc = "LIM register accessor: an alias for `Reg<LIM_SPEC>`"]
pub type LIM = crate::Reg<lim::LIM_SPEC>;
#[doc = "Limit register"]
pub mod lim;
#[doc = "MAT register accessor: an alias for `Reg<MAT_SPEC>`"]
pub type MAT = crate::Reg<mat::MAT_SPEC>;
#[doc = "Match register"]
pub mod mat;
#[doc = "DT register accessor: an alias for `Reg<DT_SPEC>`"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Dead time register"]
pub mod dt;
#[doc = "CCP register accessor: an alias for `Reg<CCP_SPEC>`"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Communication Pattern register"]
pub mod ccp;
#[doc = "CAP register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Capture register"]
pub mod cap;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable read address"]
pub mod inten;
#[doc = "INTEN_SET register accessor: an alias for `Reg<INTEN_SET_SPEC>`"]
pub type INTEN_SET = crate::Reg<inten_set::INTEN_SET_SPEC>;
#[doc = "Interrupt Enable set address"]
pub mod inten_set;
#[doc = "INTEN_CLR register accessor: an alias for `Reg<INTEN_CLR_SPEC>`"]
pub type INTEN_CLR = crate::Reg<inten_clr::INTEN_CLR_SPEC>;
#[doc = "Interrupt Enable clear address"]
pub mod inten_clr;
#[doc = "INTF register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flags read address"]
pub mod intf;
#[doc = "INTF_SET register accessor: an alias for `Reg<INTF_SET_SPEC>`"]
pub type INTF_SET = crate::Reg<intf_set::INTF_SET_SPEC>;
#[doc = "Interrupt flags set address"]
pub mod intf_set;
#[doc = "INTF_CLR register accessor: an alias for `Reg<INTF_CLR_SPEC>`"]
pub type INTF_CLR = crate::Reg<intf_clr::INTF_CLR_SPEC>;
#[doc = "Interrupt flags clear address"]
pub mod intf_clr;
#[doc = "CNTCON register accessor: an alias for `Reg<CNTCON_SPEC>`"]
pub type CNTCON = crate::Reg<cntcon::CNTCON_SPEC>;
#[doc = "Count Control read address"]
pub mod cntcon;
#[doc = "CNTCON_SET register accessor: an alias for `Reg<CNTCON_SET_SPEC>`"]
pub type CNTCON_SET = crate::Reg<cntcon_set::CNTCON_SET_SPEC>;
#[doc = "Count Control set address"]
pub mod cntcon_set;
#[doc = "CNTCON_CLR register accessor: an alias for `Reg<CNTCON_CLR_SPEC>`"]
pub type CNTCON_CLR = crate::Reg<cntcon_clr::CNTCON_CLR_SPEC>;
#[doc = "Count Control clear address"]
pub mod cntcon_clr;
#[doc = "CAP_CLR register accessor: an alias for `Reg<CAP_CLR_SPEC>`"]
pub type CAP_CLR = crate::Reg<cap_clr::CAP_CLR_SPEC>;
#[doc = "Capture clear address"]
pub mod cap_clr;
