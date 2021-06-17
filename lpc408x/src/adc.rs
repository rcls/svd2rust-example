#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion."]
    pub gdr: crate::Reg<gdr::GDR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x10..0x30 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
    pub dr: [crate::Reg<dr::DR_SPEC>; 8],
    #[doc = "0x30 - A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag."]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x34 - ADC trim register."]
    pub trm: crate::Reg<trm::TRM_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur."]
pub mod cr;
#[doc = "GDR register accessor: an alias for `Reg<GDR_SPEC>`"]
pub type GDR = crate::Reg<gdr::GDR_SPEC>;
#[doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion."]
pub mod gdr;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub mod inten;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
pub mod dr;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag."]
pub mod stat;
#[doc = "TRM register accessor: an alias for `Reg<TRM_SPEC>`"]
pub type TRM = crate::Reg<trm::TRM_SPEC>;
#[doc = "ADC trim register."]
pub mod trm;
