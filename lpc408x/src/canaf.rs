#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Acceptance Filter Register"]
    pub afmr: crate::Reg<afmr::AFMR_SPEC>,
    #[doc = "0x04 - Standard Frame Individual Start Address Register"]
    pub sff_sa: crate::Reg<sff_sa::SFF_SA_SPEC>,
    #[doc = "0x08 - Standard Frame Group Start Address Register"]
    pub sff_grp_sa: crate::Reg<sff_grp_sa::SFF_GRP_SA_SPEC>,
    #[doc = "0x0c - Extended Frame Start Address Register"]
    pub eff_sa: crate::Reg<eff_sa::EFF_SA_SPEC>,
    #[doc = "0x10 - Extended Frame Group Start Address Register"]
    pub eff_grp_sa: crate::Reg<eff_grp_sa::EFF_GRP_SA_SPEC>,
    #[doc = "0x14 - End of AF Tables register"]
    pub endoftable: crate::Reg<endoftable::ENDOFTABLE_SPEC>,
    #[doc = "0x18 - LUT Error Address register"]
    pub luterrad: crate::Reg<luterrad::LUTERRAD_SPEC>,
    #[doc = "0x1c - LUT Error Register"]
    pub luterr: crate::Reg<luterr::LUTERR_SPEC>,
    #[doc = "0x20 - FullCAN interrupt enable register"]
    pub fcanie: crate::Reg<fcanie::FCANIE_SPEC>,
    #[doc = "0x24 - FullCAN interrupt and capture register0"]
    pub fcanic0: crate::Reg<fcanic0::FCANIC0_SPEC>,
    #[doc = "0x28 - FullCAN interrupt and capture register1"]
    pub fcanic1: crate::Reg<fcanic1::FCANIC1_SPEC>,
}
#[doc = "AFMR register accessor: an alias for `Reg<AFMR_SPEC>`"]
pub type AFMR = crate::Reg<afmr::AFMR_SPEC>;
#[doc = "Acceptance Filter Register"]
pub mod afmr;
#[doc = "SFF_SA register accessor: an alias for `Reg<SFF_SA_SPEC>`"]
pub type SFF_SA = crate::Reg<sff_sa::SFF_SA_SPEC>;
#[doc = "Standard Frame Individual Start Address Register"]
pub mod sff_sa;
#[doc = "SFF_GRP_SA register accessor: an alias for `Reg<SFF_GRP_SA_SPEC>`"]
pub type SFF_GRP_SA = crate::Reg<sff_grp_sa::SFF_GRP_SA_SPEC>;
#[doc = "Standard Frame Group Start Address Register"]
pub mod sff_grp_sa;
#[doc = "EFF_SA register accessor: an alias for `Reg<EFF_SA_SPEC>`"]
pub type EFF_SA = crate::Reg<eff_sa::EFF_SA_SPEC>;
#[doc = "Extended Frame Start Address Register"]
pub mod eff_sa;
#[doc = "EFF_GRP_SA register accessor: an alias for `Reg<EFF_GRP_SA_SPEC>`"]
pub type EFF_GRP_SA = crate::Reg<eff_grp_sa::EFF_GRP_SA_SPEC>;
#[doc = "Extended Frame Group Start Address Register"]
pub mod eff_grp_sa;
#[doc = "ENDOFTABLE register accessor: an alias for `Reg<ENDOFTABLE_SPEC>`"]
pub type ENDOFTABLE = crate::Reg<endoftable::ENDOFTABLE_SPEC>;
#[doc = "End of AF Tables register"]
pub mod endoftable;
#[doc = "LUTERRAD register accessor: an alias for `Reg<LUTERRAD_SPEC>`"]
pub type LUTERRAD = crate::Reg<luterrad::LUTERRAD_SPEC>;
#[doc = "LUT Error Address register"]
pub mod luterrad;
#[doc = "LUTERR register accessor: an alias for `Reg<LUTERR_SPEC>`"]
pub type LUTERR = crate::Reg<luterr::LUTERR_SPEC>;
#[doc = "LUT Error Register"]
pub mod luterr;
#[doc = "FCANIE register accessor: an alias for `Reg<FCANIE_SPEC>`"]
pub type FCANIE = crate::Reg<fcanie::FCANIE_SPEC>;
#[doc = "FullCAN interrupt enable register"]
pub mod fcanie;
#[doc = "FCANIC0 register accessor: an alias for `Reg<FCANIC0_SPEC>`"]
pub type FCANIC0 = crate::Reg<fcanic0::FCANIC0_SPEC>;
#[doc = "FullCAN interrupt and capture register0"]
pub mod fcanic0;
#[doc = "FCANIC1 register accessor: an alias for `Reg<FCANIC1_SPEC>`"]
pub type FCANIC1 = crate::Reg<fcanic1::FCANIC1_SPEC>;
#[doc = "FullCAN interrupt and capture register1"]
pub mod fcanic1;
