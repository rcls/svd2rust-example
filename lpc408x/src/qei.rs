#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: crate::Reg<con::CON_SPEC>,
    #[doc = "0x04 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Configuration register"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0x0c - Position register"]
    pub pos: crate::Reg<pos::POS_SPEC>,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: crate::Reg<maxpos::MAXPOS_SPEC>,
    #[doc = "0x14 - Position compare register 0"]
    pub cmpos0: crate::Reg<cmpos0::CMPOS0_SPEC>,
    #[doc = "0x18 - Position compare register 1"]
    pub cmpos1: crate::Reg<cmpos1::CMPOS1_SPEC>,
    #[doc = "0x1c - Position compare register 2"]
    pub cmpos2: crate::Reg<cmpos2::CMPOS2_SPEC>,
    #[doc = "0x20 - Index count register 0"]
    pub inxcnt: crate::Reg<inxcnt::INXCNT_SPEC>,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: crate::Reg<inxcmp0::INXCMP0_SPEC>,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x2c - Velocity timer register"]
    pub time: crate::Reg<time::TIME_SPEC>,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: crate::Reg<vel::VEL_SPEC>,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: crate::Reg<cap::CAP_SPEC>,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: crate::Reg<velcomp::VELCOMP_SPEC>,
    #[doc = "0x3c - Digital filter register on PHA"]
    pub filterpha: crate::Reg<filterpha::FILTERPHA_SPEC>,
    #[doc = "0x40 - Digital filter register on PHB"]
    pub filterphb: crate::Reg<filterphb::FILTERPHB_SPEC>,
    #[doc = "0x44 - Digital filter register on IDX"]
    pub filterinx: crate::Reg<filterinx::FILTERINX_SPEC>,
    #[doc = "0x48 - Index acceptance window register"]
    pub window: crate::Reg<window::WINDOW_SPEC>,
    #[doc = "0x4c - Index compare register 1"]
    pub inxcmp1: crate::Reg<inxcmp1::INXCMP1_SPEC>,
    #[doc = "0x50 - Index compare register 2"]
    pub inxcmp2: crate::Reg<inxcmp2::INXCMP2_SPEC>,
    _reserved21: [u8; 0x0f84],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: crate::Reg<iec::IEC_SPEC>,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: crate::Reg<ies::IES_SPEC>,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: crate::Reg<clr::CLR_SPEC>,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: crate::Reg<set::SET_SPEC>,
}
#[doc = "CON register accessor: an alias for `Reg<CON_SPEC>`"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "Control register"]
pub mod con;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration register"]
pub mod conf;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "POS register accessor: an alias for `Reg<POS_SPEC>`"]
pub type POS = crate::Reg<pos::POS_SPEC>;
#[doc = "Position register"]
pub mod pos;
#[doc = "MAXPOS register accessor: an alias for `Reg<MAXPOS_SPEC>`"]
pub type MAXPOS = crate::Reg<maxpos::MAXPOS_SPEC>;
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "CMPOS0 register accessor: an alias for `Reg<CMPOS0_SPEC>`"]
pub type CMPOS0 = crate::Reg<cmpos0::CMPOS0_SPEC>;
#[doc = "Position compare register 0"]
pub mod cmpos0;
#[doc = "CMPOS1 register accessor: an alias for `Reg<CMPOS1_SPEC>`"]
pub type CMPOS1 = crate::Reg<cmpos1::CMPOS1_SPEC>;
#[doc = "Position compare register 1"]
pub mod cmpos1;
#[doc = "CMPOS2 register accessor: an alias for `Reg<CMPOS2_SPEC>`"]
pub type CMPOS2 = crate::Reg<cmpos2::CMPOS2_SPEC>;
#[doc = "Position compare register 2"]
pub mod cmpos2;
#[doc = "INXCNT register accessor: an alias for `Reg<INXCNT_SPEC>`"]
pub type INXCNT = crate::Reg<inxcnt::INXCNT_SPEC>;
#[doc = "Index count register 0"]
pub mod inxcnt;
#[doc = "INXCMP0 register accessor: an alias for `Reg<INXCMP0_SPEC>`"]
pub type INXCMP0 = crate::Reg<inxcmp0::INXCMP0_SPEC>;
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "TIME register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "VEL register accessor: an alias for `Reg<VEL_SPEC>`"]
pub type VEL = crate::Reg<vel::VEL_SPEC>;
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "CAP register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "VELCOMP register accessor: an alias for `Reg<VELCOMP_SPEC>`"]
pub type VELCOMP = crate::Reg<velcomp::VELCOMP_SPEC>;
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "FILTERPHA register accessor: an alias for `Reg<FILTERPHA_SPEC>`"]
pub type FILTERPHA = crate::Reg<filterpha::FILTERPHA_SPEC>;
#[doc = "Digital filter register on PHA"]
pub mod filterpha;
#[doc = "FILTERPHB register accessor: an alias for `Reg<FILTERPHB_SPEC>`"]
pub type FILTERPHB = crate::Reg<filterphb::FILTERPHB_SPEC>;
#[doc = "Digital filter register on PHB"]
pub mod filterphb;
#[doc = "FILTERINX register accessor: an alias for `Reg<FILTERINX_SPEC>`"]
pub type FILTERINX = crate::Reg<filterinx::FILTERINX_SPEC>;
#[doc = "Digital filter register on IDX"]
pub mod filterinx;
#[doc = "WINDOW register accessor: an alias for `Reg<WINDOW_SPEC>`"]
pub type WINDOW = crate::Reg<window::WINDOW_SPEC>;
#[doc = "Index acceptance window register"]
pub mod window;
#[doc = "INXCMP1 register accessor: an alias for `Reg<INXCMP1_SPEC>`"]
pub type INXCMP1 = crate::Reg<inxcmp1::INXCMP1_SPEC>;
#[doc = "Index compare register 1"]
pub mod inxcmp1;
#[doc = "INXCMP2 register accessor: an alias for `Reg<INXCMP2_SPEC>`"]
pub type INXCMP2 = crate::Reg<inxcmp2::INXCMP2_SPEC>;
#[doc = "Index compare register 2"]
pub mod inxcmp2;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "SET register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Interrupt status set register"]
pub mod set;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "IES register accessor: an alias for `Reg<IES_SPEC>`"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "IEC register accessor: an alias for `Reg<IEC_SPEC>`"]
pub type IEC = crate::Reg<iec::IEC_SPEC>;
#[doc = "Interrupt enable clear register"]
pub mod iec;
