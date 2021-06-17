#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xdc],
    #[doc = "0xdc - USB Receive Packet Length"]
    pub rxplen: crate::Reg<rxplen::RXPLEN_SPEC>,
    _reserved1: [u8; 0x20],
    #[doc = "0x100 - OTG Interrupt Status"]
    pub intst: crate::Reg<intst::INTST_SPEC>,
    #[doc = "0x104 - OTG Interrupt Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x108 - OTG Interrupt Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    #[doc = "0x10c - OTG Interrupt Clear"]
    pub inclr: crate::Reg<inclr::INCLR_SPEC>,
    #[doc = "0x110 - USB Port Select. The USBPortSel register is identical to the OTGStCtrl register (see Section 15.8.6). In device-only operations only bits 0 and 1 of this register are used to control the routing of USB pins to Port 1 or Port 2."]
    pub portsel: crate::Reg<portsel::PORTSEL_SPEC>,
    #[doc = "0x114 - OTG Timer"]
    pub tmr: crate::Reg<tmr::TMR_SPEC>,
    _reserved7: [u8; 0xe8],
    #[doc = "0x200 - USB Device Interrupt Status"]
    pub devintst: crate::Reg<devintst::DEVINTST_SPEC>,
    #[doc = "0x204 - USB Device Interrupt Enable"]
    pub devinten: crate::Reg<devinten::DEVINTEN_SPEC>,
    #[doc = "0x208 - USB Device Interrupt Clear"]
    pub devintclr: crate::Reg<devintclr::DEVINTCLR_SPEC>,
    #[doc = "0x20c - USB Device Interrupt Set"]
    pub devintset: crate::Reg<devintset::DEVINTSET_SPEC>,
    #[doc = "0x210 - USB Command Code"]
    pub cmdcode: crate::Reg<cmdcode::CMDCODE_SPEC>,
    #[doc = "0x214 - USB Command Data"]
    pub cmddata: crate::Reg<cmddata::CMDDATA_SPEC>,
    #[doc = "0x218 - USB Receive Data"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x21c - USB Transmit Data"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x224 - USB Transmit Packet Length"]
    pub txplen: crate::Reg<txplen::TXPLEN_SPEC>,
    #[doc = "0x228 - USB Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x22c - USB Device Interrupt Priority"]
    pub devintpri: crate::Reg<devintpri::DEVINTPRI_SPEC>,
    #[doc = "0x230 - USB Endpoint Interrupt Status"]
    pub epintst: crate::Reg<epintst::EPINTST_SPEC>,
    #[doc = "0x234 - USB Endpoint Interrupt Enable"]
    pub epinten: crate::Reg<epinten::EPINTEN_SPEC>,
    #[doc = "0x238 - USB Endpoint Interrupt Clear"]
    pub epintclr: crate::Reg<epintclr::EPINTCLR_SPEC>,
    #[doc = "0x23c - USB Endpoint Interrupt Set"]
    pub epintset: crate::Reg<epintset::EPINTSET_SPEC>,
    #[doc = "0x240 - USB Endpoint Priority"]
    pub epintpri: crate::Reg<epintpri::EPINTPRI_SPEC>,
    #[doc = "0x244 - USB Realize Endpoint"]
    pub reep: crate::Reg<reep::REEP_SPEC>,
    #[doc = "0x248 - USB Endpoint Index"]
    pub epin: crate::Reg<epin::EPIN_SPEC>,
    #[doc = "0x24c - USB MaxPacketSize"]
    pub maxpsize: crate::Reg<maxpsize::MAXPSIZE_SPEC>,
    #[doc = "0x250 - USB DMA Request Status"]
    pub dmarst: crate::Reg<dmarst::DMARST_SPEC>,
    #[doc = "0x254 - USB DMA Request Clear"]
    pub dmarclr: crate::Reg<dmarclr::DMARCLR_SPEC>,
    #[doc = "0x258 - USB DMA Request Set"]
    pub dmarset: crate::Reg<dmarset::DMARSET_SPEC>,
    _reserved29: [u8; 0x24],
    #[doc = "0x280 - USB UDCA Head"]
    pub udcah: crate::Reg<udcah::UDCAH_SPEC>,
    #[doc = "0x284 - USB Endpoint DMA Status"]
    pub epdmast: crate::Reg<epdmast::EPDMAST_SPEC>,
    #[doc = "0x288 - USB Endpoint DMA Enable"]
    pub epdmaen: crate::Reg<epdmaen::EPDMAEN_SPEC>,
    #[doc = "0x28c - USB Endpoint DMA Disable"]
    pub epdmadis: crate::Reg<epdmadis::EPDMADIS_SPEC>,
    #[doc = "0x290 - USB DMA Interrupt Status"]
    pub dmaintst: crate::Reg<dmaintst::DMAINTST_SPEC>,
    #[doc = "0x294 - USB DMA Interrupt Enable"]
    pub dmainten: crate::Reg<dmainten::DMAINTEN_SPEC>,
    _reserved35: [u8; 0x08],
    #[doc = "0x2a0 - USB End of Transfer Interrupt Status"]
    pub eotintst: crate::Reg<eotintst::EOTINTST_SPEC>,
    #[doc = "0x2a4 - USB End of Transfer Interrupt Clear"]
    pub eotintclr: crate::Reg<eotintclr::EOTINTCLR_SPEC>,
    #[doc = "0x2a8 - USB End of Transfer Interrupt Set"]
    pub eotintset: crate::Reg<eotintset::EOTINTSET_SPEC>,
    #[doc = "0x2ac - USB New DD Request Interrupt Status"]
    pub nddrintst: crate::Reg<nddrintst::NDDRINTST_SPEC>,
    #[doc = "0x2b0 - USB New DD Request Interrupt Clear"]
    pub nddrintclr: crate::Reg<nddrintclr::NDDRINTCLR_SPEC>,
    #[doc = "0x2b4 - USB New DD Request Interrupt Set"]
    pub nddrintset: crate::Reg<nddrintset::NDDRINTSET_SPEC>,
    #[doc = "0x2b8 - USB System Error Interrupt Status"]
    pub syserrintst: crate::Reg<syserrintst::SYSERRINTST_SPEC>,
    #[doc = "0x2bc - USB System Error Interrupt Clear"]
    pub syserrintclr: crate::Reg<syserrintclr::SYSERRINTCLR_SPEC>,
    #[doc = "0x2c0 - USB System Error Interrupt Set"]
    pub syserrintset: crate::Reg<syserrintset::SYSERRINTSET_SPEC>,
    _reserved44: [u8; 0x3c],
    _reserved_44_i2c: [u8; 0x04],
    #[doc = "0x304 - I2C Status"]
    pub i2c_sts: crate::Reg<i2c_sts::I2C_STS_SPEC>,
    #[doc = "0x308 - I2C Control"]
    pub i2c_ctl: crate::Reg<i2c_ctl::I2C_CTL_SPEC>,
    #[doc = "0x30c - I2C Clock High"]
    pub i2c_clkhi: crate::Reg<i2c_clkhi::I2C_CLKHI_SPEC>,
    #[doc = "0x310 - I2C Clock Low"]
    pub i2c_clklo: crate::Reg<i2c_clklo::I2C_CLKLO_SPEC>,
    _reserved49: [u8; 0x0ce0],
    #[doc = "0xff4 - OTG clock controller"]
    pub clkctrl: crate::Reg<clkctrl::CLKCTRL_SPEC>,
    #[doc = "0xff8 - OTG clock status"]
    pub otgclk_st: crate::Reg<otgclk_st::OTGCLKST_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x300 - I2C Transmit"]
    #[inline(always)]
    pub fn i2c_tx(&self) -> &crate::Reg<i2c_tx::I2C_TX_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(768usize)
                as *const crate::Reg<i2c_tx::I2C_TX_SPEC>)
        }
    }
    #[doc = "0x300 - I2C Receive"]
    #[inline(always)]
    pub fn i2c_rx(&self) -> &crate::Reg<i2c_rx::I2C_RX_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(768usize)
                as *const crate::Reg<i2c_rx::I2C_RX_SPEC>)
        }
    }
}
#[doc = "INTST register accessor: an alias for `Reg<INTST_SPEC>`"]
pub type INTST = crate::Reg<intst::INTST_SPEC>;
#[doc = "OTG Interrupt Status"]
pub mod intst;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "OTG Interrupt Enable"]
pub mod inten;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "OTG Interrupt Set"]
pub mod intset;
#[doc = "INCLR register accessor: an alias for `Reg<INCLR_SPEC>`"]
pub type INCLR = crate::Reg<inclr::INCLR_SPEC>;
#[doc = "OTG Interrupt Clear"]
pub mod inclr;
#[doc = "PORTSEL register accessor: an alias for `Reg<PORTSEL_SPEC>`"]
pub type PORTSEL = crate::Reg<portsel::PORTSEL_SPEC>;
#[doc = "USB Port Select. The USBPortSel register is identical to the OTGStCtrl register (see Section 15.8.6). In device-only operations only bits 0 and 1 of this register are used to control the routing of USB pins to Port 1 or Port 2."]
pub mod portsel;
#[doc = "TMR register accessor: an alias for `Reg<TMR_SPEC>`"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "OTG Timer"]
pub mod tmr;
#[doc = "DEVINTST register accessor: an alias for `Reg<DEVINTST_SPEC>`"]
pub type DEVINTST = crate::Reg<devintst::DEVINTST_SPEC>;
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "DEVINTEN register accessor: an alias for `Reg<DEVINTEN_SPEC>`"]
pub type DEVINTEN = crate::Reg<devinten::DEVINTEN_SPEC>;
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "DEVINTCLR register accessor: an alias for `Reg<DEVINTCLR_SPEC>`"]
pub type DEVINTCLR = crate::Reg<devintclr::DEVINTCLR_SPEC>;
#[doc = "USB Device Interrupt Clear"]
pub mod devintclr;
#[doc = "DEVINTSET register accessor: an alias for `Reg<DEVINTSET_SPEC>`"]
pub type DEVINTSET = crate::Reg<devintset::DEVINTSET_SPEC>;
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "DEVINTPRI register accessor: an alias for `Reg<DEVINTPRI_SPEC>`"]
pub type DEVINTPRI = crate::Reg<devintpri::DEVINTPRI_SPEC>;
#[doc = "USB Device Interrupt Priority"]
pub mod devintpri;
#[doc = "EPINTST register accessor: an alias for `Reg<EPINTST_SPEC>`"]
pub type EPINTST = crate::Reg<epintst::EPINTST_SPEC>;
#[doc = "USB Endpoint Interrupt Status"]
pub mod epintst;
#[doc = "EPINTEN register accessor: an alias for `Reg<EPINTEN_SPEC>`"]
pub type EPINTEN = crate::Reg<epinten::EPINTEN_SPEC>;
#[doc = "USB Endpoint Interrupt Enable"]
pub mod epinten;
#[doc = "EPINTCLR register accessor: an alias for `Reg<EPINTCLR_SPEC>`"]
pub type EPINTCLR = crate::Reg<epintclr::EPINTCLR_SPEC>;
#[doc = "USB Endpoint Interrupt Clear"]
pub mod epintclr;
#[doc = "EPINTSET register accessor: an alias for `Reg<EPINTSET_SPEC>`"]
pub type EPINTSET = crate::Reg<epintset::EPINTSET_SPEC>;
#[doc = "USB Endpoint Interrupt Set"]
pub mod epintset;
#[doc = "EPINTPRI register accessor: an alias for `Reg<EPINTPRI_SPEC>`"]
pub type EPINTPRI = crate::Reg<epintpri::EPINTPRI_SPEC>;
#[doc = "USB Endpoint Priority"]
pub mod epintpri;
#[doc = "REEP register accessor: an alias for `Reg<REEP_SPEC>`"]
pub type REEP = crate::Reg<reep::REEP_SPEC>;
#[doc = "USB Realize Endpoint"]
pub mod reep;
#[doc = "EPIN register accessor: an alias for `Reg<EPIN_SPEC>`"]
pub type EPIN = crate::Reg<epin::EPIN_SPEC>;
#[doc = "USB Endpoint Index"]
pub mod epin;
#[doc = "MAXPSIZE register accessor: an alias for `Reg<MAXPSIZE_SPEC>`"]
pub type MAXPSIZE = crate::Reg<maxpsize::MAXPSIZE_SPEC>;
#[doc = "USB MaxPacketSize"]
pub mod maxpsize;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "RXPLEN register accessor: an alias for `Reg<RXPLEN_SPEC>`"]
pub type RXPLEN = crate::Reg<rxplen::RXPLEN_SPEC>;
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "TXPLEN register accessor: an alias for `Reg<TXPLEN_SPEC>`"]
pub type TXPLEN = crate::Reg<txplen::TXPLEN_SPEC>;
#[doc = "USB Transmit Packet Length"]
pub mod txplen;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "CMDCODE register accessor: an alias for `Reg<CMDCODE_SPEC>`"]
pub type CMDCODE = crate::Reg<cmdcode::CMDCODE_SPEC>;
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "CMDDATA register accessor: an alias for `Reg<CMDDATA_SPEC>`"]
pub type CMDDATA = crate::Reg<cmddata::CMDDATA_SPEC>;
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "DMARST register accessor: an alias for `Reg<DMARST_SPEC>`"]
pub type DMARST = crate::Reg<dmarst::DMARST_SPEC>;
#[doc = "USB DMA Request Status"]
pub mod dmarst;
#[doc = "DMARCLR register accessor: an alias for `Reg<DMARCLR_SPEC>`"]
pub type DMARCLR = crate::Reg<dmarclr::DMARCLR_SPEC>;
#[doc = "USB DMA Request Clear"]
pub mod dmarclr;
#[doc = "DMARSET register accessor: an alias for `Reg<DMARSET_SPEC>`"]
pub type DMARSET = crate::Reg<dmarset::DMARSET_SPEC>;
#[doc = "USB DMA Request Set"]
pub mod dmarset;
#[doc = "UDCAH register accessor: an alias for `Reg<UDCAH_SPEC>`"]
pub type UDCAH = crate::Reg<udcah::UDCAH_SPEC>;
#[doc = "USB UDCA Head"]
pub mod udcah;
#[doc = "EPDMAST register accessor: an alias for `Reg<EPDMAST_SPEC>`"]
pub type EPDMAST = crate::Reg<epdmast::EPDMAST_SPEC>;
#[doc = "USB Endpoint DMA Status"]
pub mod epdmast;
#[doc = "EPDMAEN register accessor: an alias for `Reg<EPDMAEN_SPEC>`"]
pub type EPDMAEN = crate::Reg<epdmaen::EPDMAEN_SPEC>;
#[doc = "USB Endpoint DMA Enable"]
pub mod epdmaen;
#[doc = "EPDMADIS register accessor: an alias for `Reg<EPDMADIS_SPEC>`"]
pub type EPDMADIS = crate::Reg<epdmadis::EPDMADIS_SPEC>;
#[doc = "USB Endpoint DMA Disable"]
pub mod epdmadis;
#[doc = "DMAINTST register accessor: an alias for `Reg<DMAINTST_SPEC>`"]
pub type DMAINTST = crate::Reg<dmaintst::DMAINTST_SPEC>;
#[doc = "USB DMA Interrupt Status"]
pub mod dmaintst;
#[doc = "DMAINTEN register accessor: an alias for `Reg<DMAINTEN_SPEC>`"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "USB DMA Interrupt Enable"]
pub mod dmainten;
#[doc = "EOTINTST register accessor: an alias for `Reg<EOTINTST_SPEC>`"]
pub type EOTINTST = crate::Reg<eotintst::EOTINTST_SPEC>;
#[doc = "USB End of Transfer Interrupt Status"]
pub mod eotintst;
#[doc = "EOTINTCLR register accessor: an alias for `Reg<EOTINTCLR_SPEC>`"]
pub type EOTINTCLR = crate::Reg<eotintclr::EOTINTCLR_SPEC>;
#[doc = "USB End of Transfer Interrupt Clear"]
pub mod eotintclr;
#[doc = "EOTINTSET register accessor: an alias for `Reg<EOTINTSET_SPEC>`"]
pub type EOTINTSET = crate::Reg<eotintset::EOTINTSET_SPEC>;
#[doc = "USB End of Transfer Interrupt Set"]
pub mod eotintset;
#[doc = "NDDRINTST register accessor: an alias for `Reg<NDDRINTST_SPEC>`"]
pub type NDDRINTST = crate::Reg<nddrintst::NDDRINTST_SPEC>;
#[doc = "USB New DD Request Interrupt Status"]
pub mod nddrintst;
#[doc = "NDDRINTCLR register accessor: an alias for `Reg<NDDRINTCLR_SPEC>`"]
pub type NDDRINTCLR = crate::Reg<nddrintclr::NDDRINTCLR_SPEC>;
#[doc = "USB New DD Request Interrupt Clear"]
pub mod nddrintclr;
#[doc = "NDDRINTSET register accessor: an alias for `Reg<NDDRINTSET_SPEC>`"]
pub type NDDRINTSET = crate::Reg<nddrintset::NDDRINTSET_SPEC>;
#[doc = "USB New DD Request Interrupt Set"]
pub mod nddrintset;
#[doc = "SYSERRINTST register accessor: an alias for `Reg<SYSERRINTST_SPEC>`"]
pub type SYSERRINTST = crate::Reg<syserrintst::SYSERRINTST_SPEC>;
#[doc = "USB System Error Interrupt Status"]
pub mod syserrintst;
#[doc = "SYSERRINTCLR register accessor: an alias for `Reg<SYSERRINTCLR_SPEC>`"]
pub type SYSERRINTCLR = crate::Reg<syserrintclr::SYSERRINTCLR_SPEC>;
#[doc = "USB System Error Interrupt Clear"]
pub mod syserrintclr;
#[doc = "SYSERRINTSET register accessor: an alias for `Reg<SYSERRINTSET_SPEC>`"]
pub type SYSERRINTSET = crate::Reg<syserrintset::SYSERRINTSET_SPEC>;
#[doc = "USB System Error Interrupt Set"]
pub mod syserrintset;
#[doc = "I2C_RX register accessor: an alias for `Reg<I2C_RX_SPEC>`"]
pub type I2C_RX = crate::Reg<i2c_rx::I2C_RX_SPEC>;
#[doc = "I2C Receive"]
pub mod i2c_rx;
#[doc = "I2C_TX register accessor: an alias for `Reg<I2C_TX_SPEC>`"]
pub type I2C_TX = crate::Reg<i2c_tx::I2C_TX_SPEC>;
#[doc = "I2C Transmit"]
pub mod i2c_tx;
#[doc = "I2C_STS register accessor: an alias for `Reg<I2C_STS_SPEC>`"]
pub type I2C_STS = crate::Reg<i2c_sts::I2C_STS_SPEC>;
#[doc = "I2C Status"]
pub mod i2c_sts;
#[doc = "I2C_CTL register accessor: an alias for `Reg<I2C_CTL_SPEC>`"]
pub type I2C_CTL = crate::Reg<i2c_ctl::I2C_CTL_SPEC>;
#[doc = "I2C Control"]
pub mod i2c_ctl;
#[doc = "I2C_CLKHI register accessor: an alias for `Reg<I2C_CLKHI_SPEC>`"]
pub type I2C_CLKHI = crate::Reg<i2c_clkhi::I2C_CLKHI_SPEC>;
#[doc = "I2C Clock High"]
pub mod i2c_clkhi;
#[doc = "I2C_CLKLO register accessor: an alias for `Reg<I2C_CLKLO_SPEC>`"]
pub type I2C_CLKLO = crate::Reg<i2c_clklo::I2C_CLKLO_SPEC>;
#[doc = "I2C Clock Low"]
pub mod i2c_clklo;
#[doc = "CLKCTRL register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "OTG clock controller"]
pub mod clkctrl;
#[doc = "OTGClkSt register accessor: an alias for `Reg<OTGCLKST_SPEC>`"]
pub type OTGCLKST = crate::Reg<otgclk_st::OTGCLKST_SPEC>;
#[doc = "OTG clock status"]
pub mod otgclk_st;
