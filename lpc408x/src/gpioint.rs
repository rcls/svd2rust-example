#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO overall Interrupt Status."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr0: crate::Reg<statr0::STATR0_SPEC>,
    #[doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf0: crate::Reg<statf0::STATF0_SPEC>,
    #[doc = "0x0c - GPIO Interrupt Clear."]
    pub clr0: crate::Reg<clr0::CLR0_SPEC>,
    #[doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr0: crate::Reg<enr0::ENR0_SPEC>,
    #[doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf0: crate::Reg<enf0::ENF0_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr2: crate::Reg<statr2::STATR2_SPEC>,
    #[doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf2: crate::Reg<statf2::STATF2_SPEC>,
    #[doc = "0x2c - GPIO Interrupt Clear."]
    pub clr2: crate::Reg<clr2::CLR2_SPEC>,
    #[doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr2: crate::Reg<enr2::ENR2_SPEC>,
    #[doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf2: crate::Reg<enf2::ENF2_SPEC>,
}
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO overall Interrupt Status."]
pub mod status;
#[doc = "STATR0 register accessor: an alias for `Reg<STATR0_SPEC>`"]
pub type STATR0 = crate::Reg<statr0::STATR0_SPEC>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr0;
#[doc = "STATF0 register accessor: an alias for `Reg<STATF0_SPEC>`"]
pub type STATF0 = crate::Reg<statf0::STATF0_SPEC>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf0;
#[doc = "CLR0 register accessor: an alias for `Reg<CLR0_SPEC>`"]
pub type CLR0 = crate::Reg<clr0::CLR0_SPEC>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr0;
#[doc = "ENR0 register accessor: an alias for `Reg<ENR0_SPEC>`"]
pub type ENR0 = crate::Reg<enr0::ENR0_SPEC>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr0;
#[doc = "ENF0 register accessor: an alias for `Reg<ENF0_SPEC>`"]
pub type ENF0 = crate::Reg<enf0::ENF0_SPEC>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf0;
#[doc = "STATR2 register accessor: an alias for `Reg<STATR2_SPEC>`"]
pub type STATR2 = crate::Reg<statr2::STATR2_SPEC>;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr2;
#[doc = "STATF2 register accessor: an alias for `Reg<STATF2_SPEC>`"]
pub type STATF2 = crate::Reg<statf2::STATF2_SPEC>;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf2;
#[doc = "CLR2 register accessor: an alias for `Reg<CLR2_SPEC>`"]
pub type CLR2 = crate::Reg<clr2::CLR2_SPEC>;
#[doc = "GPIO Interrupt Clear."]
pub mod clr2;
#[doc = "ENR2 register accessor: an alias for `Reg<ENR2_SPEC>`"]
pub type ENR2 = crate::Reg<enr2::ENR2_SPEC>;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr2;
#[doc = "ENF2 register accessor: an alias for `Reg<ENF2_SPEC>`"]
pub type ENF2 = crate::Reg<enf2::ENF2_SPEC>;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf2;
