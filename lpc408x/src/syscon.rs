#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: crate::Reg<flashcfg::FLASHCFG_SPEC>,
    _reserved1: [u8; 0x7c],
    #[doc = "0x80 - PLL0 Control register"]
    pub pllcon0: crate::Reg<pllcon::PLLCON_SPEC>,
    #[doc = "0x84 - PLL0 Configuration register"]
    pub pllcfg0: crate::Reg<pllcfg::PLLCFG_SPEC>,
    #[doc = "0x88 - PLL0 Status register"]
    pub pllstat0: crate::Reg<pllstat::PLLSTAT_SPEC>,
    #[doc = "0x8c - PLL0 Feed register"]
    pub pllfeed0: crate::Reg<pllfeed::PLLFEED_SPEC>,
    _reserved5: [u8; 0x10],
    #[doc = "0xa0 - PLL0 Control register"]
    pub pllcon1: crate::Reg<pllcon::PLLCON_SPEC>,
    #[doc = "0xa4 - PLL0 Configuration register"]
    pub pllcfg1: crate::Reg<pllcfg::PLLCFG_SPEC>,
    #[doc = "0xa8 - PLL0 Status register"]
    pub pllstat1: crate::Reg<pllstat::PLLSTAT_SPEC>,
    #[doc = "0xac - PLL0 Feed register"]
    pub pllfeed1: crate::Reg<pllfeed::PLLFEED_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0xc0 - Power Control register"]
    pub pcon: crate::Reg<pcon::PCON_SPEC>,
    #[doc = "0xc4 - Power Control for Peripherals"]
    pub pconp0: crate::Reg<pconp0::PCONP0_SPEC>,
    #[doc = "0xc8 - Power Control for Peripherals"]
    pub pconp1: crate::Reg<pconp1::PCONP1_SPEC>,
    _reserved12: [u8; 0x34],
    #[doc = "0x100 - External Memory Controller Clock Selection register"]
    pub emcclksel: crate::Reg<emcclksel::EMCCLKSEL_SPEC>,
    #[doc = "0x104 - CPU Clock Selection register"]
    pub cclksel: crate::Reg<cclksel::CCLKSEL_SPEC>,
    #[doc = "0x108 - USB Clock Selection register"]
    pub usbclksel: crate::Reg<usbclksel::USBCLKSEL_SPEC>,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: crate::Reg<clksrcsel::CLKSRCSEL_SPEC>,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: crate::Reg<cansleepclr::CANSLEEPCLR_SPEC>,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: crate::Reg<canwakeflags::CANWAKEFLAGS_SPEC>,
    _reserved18: [u8; 0x28],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: crate::Reg<extint::EXTINT_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: crate::Reg<extmode::EXTMODE_SPEC>,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: crate::Reg<extpolar::EXTPOLAR_SPEC>,
    _reserved21: [u8; 0x30],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: crate::Reg<rsid::RSID_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x188 - Matrix arbitration register"]
    pub matrixarb: crate::Reg<matrixarb::MATRIXARB_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0x1a0 - System Control and Status"]
    pub scs: crate::Reg<scs::SCS_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x1a8 - Peripheral Clock Selection register"]
    pub pclksel: crate::Reg<pclksel::PCLKSEL_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x1b0 - Power boost register"]
    pub pboost: crate::Reg<pboost::PBOOST_SPEC>,
    #[doc = "0x1b4 - SPIFI Clock Selection register"]
    pub spificlksel: crate::Reg<spificlksel::SPIFICLKSEL_SPEC>,
    #[doc = "0x1b8 - LCD Clock configuration register"]
    pub lcd_cfg: crate::Reg<lcd_cfg::LCD_CFG_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: crate::Reg<usbintst::USBINTST_SPEC>,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: crate::Reg<dmacreqsel::DMACREQSEL_SPEC>,
    #[doc = "0x1c8 - Clock Output Configuration register"]
    pub clkoutcfg: crate::Reg<clkoutcfg::CLKOUTCFG_SPEC>,
    #[doc = "0x1cc - Individual peripheral reset control bits"]
    pub rstcon0: crate::Reg<rstcon0::RSTCON0_SPEC>,
    #[doc = "0x1d0 - Individual peripheral reset control bits"]
    pub rstcon1: crate::Reg<rstcon1::RSTCON1_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x1dc - Values for the 4 programmable delays associated with SDRAM operation."]
    pub emcdlyctl: crate::Reg<emcdlyctl::EMCDLYCTL_SPEC>,
    #[doc = "0x1e0 - Controls the calibration counter for programmable delays and returns the result value."]
    pub emccal: crate::Reg<emccal::EMCCAL_SPEC>,
}
#[doc = "FLASHCFG register accessor: an alias for `Reg<FLASHCFG_SPEC>`"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLLCON register accessor: an alias for `Reg<PLLCON_SPEC>`"]
pub type PLLCON = crate::Reg<pllcon::PLLCON_SPEC>;
#[doc = "PLL0 Control register"]
pub mod pllcon;
#[doc = "PLLCFG register accessor: an alias for `Reg<PLLCFG_SPEC>`"]
pub type PLLCFG = crate::Reg<pllcfg::PLLCFG_SPEC>;
#[doc = "PLL0 Configuration register"]
pub mod pllcfg;
#[doc = "PLLSTAT register accessor: an alias for `Reg<PLLSTAT_SPEC>`"]
pub type PLLSTAT = crate::Reg<pllstat::PLLSTAT_SPEC>;
#[doc = "PLL0 Status register"]
pub mod pllstat;
#[doc = "PLLFEED register accessor: an alias for `Reg<PLLFEED_SPEC>`"]
pub type PLLFEED = crate::Reg<pllfeed::PLLFEED_SPEC>;
#[doc = "PLL0 Feed register"]
pub mod pllfeed;
#[doc = "PCON register accessor: an alias for `Reg<PCON_SPEC>`"]
pub type PCON = crate::Reg<pcon::PCON_SPEC>;
#[doc = "Power Control register"]
pub mod pcon;
#[doc = "PCONP0 register accessor: an alias for `Reg<PCONP0_SPEC>`"]
pub type PCONP0 = crate::Reg<pconp0::PCONP0_SPEC>;
#[doc = "Power Control for Peripherals"]
pub mod pconp0;
#[doc = "PCONP1 register accessor: an alias for `Reg<PCONP1_SPEC>`"]
pub type PCONP1 = crate::Reg<pconp1::PCONP1_SPEC>;
#[doc = "Power Control for Peripherals"]
pub mod pconp1;
#[doc = "EMCCLKSEL register accessor: an alias for `Reg<EMCCLKSEL_SPEC>`"]
pub type EMCCLKSEL = crate::Reg<emcclksel::EMCCLKSEL_SPEC>;
#[doc = "External Memory Controller Clock Selection register"]
pub mod emcclksel;
#[doc = "CCLKSEL register accessor: an alias for `Reg<CCLKSEL_SPEC>`"]
pub type CCLKSEL = crate::Reg<cclksel::CCLKSEL_SPEC>;
#[doc = "CPU Clock Selection register"]
pub mod cclksel;
#[doc = "USBCLKSEL register accessor: an alias for `Reg<USBCLKSEL_SPEC>`"]
pub type USBCLKSEL = crate::Reg<usbclksel::USBCLKSEL_SPEC>;
#[doc = "USB Clock Selection register"]
pub mod usbclksel;
#[doc = "CLKSRCSEL register accessor: an alias for `Reg<CLKSRCSEL_SPEC>`"]
pub type CLKSRCSEL = crate::Reg<clksrcsel::CLKSRCSEL_SPEC>;
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "CANSLEEPCLR register accessor: an alias for `Reg<CANSLEEPCLR_SPEC>`"]
pub type CANSLEEPCLR = crate::Reg<cansleepclr::CANSLEEPCLR_SPEC>;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "CANWAKEFLAGS register accessor: an alias for `Reg<CANWAKEFLAGS_SPEC>`"]
pub type CANWAKEFLAGS = crate::Reg<canwakeflags::CANWAKEFLAGS_SPEC>;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "EXTINT register accessor: an alias for `Reg<EXTINT_SPEC>`"]
pub type EXTINT = crate::Reg<extint::EXTINT_SPEC>;
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "EXTMODE register accessor: an alias for `Reg<EXTMODE_SPEC>`"]
pub type EXTMODE = crate::Reg<extmode::EXTMODE_SPEC>;
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "EXTPOLAR register accessor: an alias for `Reg<EXTPOLAR_SPEC>`"]
pub type EXTPOLAR = crate::Reg<extpolar::EXTPOLAR_SPEC>;
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "RSID register accessor: an alias for `Reg<RSID_SPEC>`"]
pub type RSID = crate::Reg<rsid::RSID_SPEC>;
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "MATRIXARB register accessor: an alias for `Reg<MATRIXARB_SPEC>`"]
pub type MATRIXARB = crate::Reg<matrixarb::MATRIXARB_SPEC>;
#[doc = "Matrix arbitration register"]
pub mod matrixarb;
#[doc = "SCS register accessor: an alias for `Reg<SCS_SPEC>`"]
pub type SCS = crate::Reg<scs::SCS_SPEC>;
#[doc = "System Control and Status"]
pub mod scs;
#[doc = "PCLKSEL register accessor: an alias for `Reg<PCLKSEL_SPEC>`"]
pub type PCLKSEL = crate::Reg<pclksel::PCLKSEL_SPEC>;
#[doc = "Peripheral Clock Selection register"]
pub mod pclksel;
#[doc = "PBOOST register accessor: an alias for `Reg<PBOOST_SPEC>`"]
pub type PBOOST = crate::Reg<pboost::PBOOST_SPEC>;
#[doc = "Power boost register"]
pub mod pboost;
#[doc = "SPIFICLKSEL register accessor: an alias for `Reg<SPIFICLKSEL_SPEC>`"]
pub type SPIFICLKSEL = crate::Reg<spificlksel::SPIFICLKSEL_SPEC>;
#[doc = "SPIFI Clock Selection register"]
pub mod spificlksel;
#[doc = "LCD_CFG register accessor: an alias for `Reg<LCD_CFG_SPEC>`"]
pub type LCD_CFG = crate::Reg<lcd_cfg::LCD_CFG_SPEC>;
#[doc = "LCD Clock configuration register"]
pub mod lcd_cfg;
#[doc = "USBINTST register accessor: an alias for `Reg<USBINTST_SPEC>`"]
pub type USBINTST = crate::Reg<usbintst::USBINTST_SPEC>;
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "DMACREQSEL register accessor: an alias for `Reg<DMACREQSEL_SPEC>`"]
pub type DMACREQSEL = crate::Reg<dmacreqsel::DMACREQSEL_SPEC>;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "CLKOUTCFG register accessor: an alias for `Reg<CLKOUTCFG_SPEC>`"]
pub type CLKOUTCFG = crate::Reg<clkoutcfg::CLKOUTCFG_SPEC>;
#[doc = "Clock Output Configuration register"]
pub mod clkoutcfg;
#[doc = "RSTCON0 register accessor: an alias for `Reg<RSTCON0_SPEC>`"]
pub type RSTCON0 = crate::Reg<rstcon0::RSTCON0_SPEC>;
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon0;
#[doc = "RSTCON1 register accessor: an alias for `Reg<RSTCON1_SPEC>`"]
pub type RSTCON1 = crate::Reg<rstcon1::RSTCON1_SPEC>;
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon1;
#[doc = "EMCDLYCTL register accessor: an alias for `Reg<EMCDLYCTL_SPEC>`"]
pub type EMCDLYCTL = crate::Reg<emcdlyctl::EMCDLYCTL_SPEC>;
#[doc = "Values for the 4 programmable delays associated with SDRAM operation."]
pub mod emcdlyctl;
#[doc = "EMCCAL register accessor: an alias for `Reg<EMCCAL_SPEC>`"]
pub type EMCCAL = crate::Reg<emccal::EMCCAL_SPEC>;
#[doc = "Controls the calibration counter for programmable delays and returns the result value."]
pub mod emccal;
