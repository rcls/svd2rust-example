#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Status Register"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x04 - DMA Interrupt Terminal Count Request Status Register"]
    pub inttcstat: crate::Reg<inttcstat::INTTCSTAT_SPEC>,
    #[doc = "0x08 - DMA Interrupt Terminal Count Request Clear Register"]
    pub inttcclear: crate::Reg<inttcclear::INTTCCLEAR_SPEC>,
    #[doc = "0x0c - DMA Interrupt Error Status Register"]
    pub interrstat: crate::Reg<interrstat::INTERRSTAT_SPEC>,
    #[doc = "0x10 - DMA Interrupt Error Clear Register"]
    pub interrclr: crate::Reg<interrclr::INTERRCLR_SPEC>,
    #[doc = "0x14 - DMA Raw Interrupt Terminal Count Status Register"]
    pub rawinttcstat: crate::Reg<rawinttcstat::RAWINTTCSTAT_SPEC>,
    #[doc = "0x18 - DMA Raw Error Interrupt Status Register"]
    pub rawinterrstat: crate::Reg<rawinterrstat::RAWINTERRSTAT_SPEC>,
    #[doc = "0x1c - DMA Enabled Channel Register"]
    pub enbldchns: crate::Reg<enbldchns::ENBLDCHNS_SPEC>,
    #[doc = "0x20 - DMA Software Burst Request Register"]
    pub softbreq: crate::Reg<softbreq::SOFTBREQ_SPEC>,
    #[doc = "0x24 - DMA Software Single Request Register"]
    pub softsreq: crate::Reg<softsreq::SOFTSREQ_SPEC>,
    #[doc = "0x28 - DMA Software Last Burst Request Register"]
    pub softlbreq: crate::Reg<softlbreq::SOFTLBREQ_SPEC>,
    #[doc = "0x2c - DMA Software Last Single Request Register"]
    pub softlsreq: crate::Reg<softlsreq::SOFTLSREQ_SPEC>,
    #[doc = "0x30 - DMA Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x34 - DMA Synchronization Register"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100 - DMA Channel 0 Source Address Register"]
    pub srcaddr0: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x104 - DMA Channel 0 Destination Address Register"]
    pub destaddr0: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x108 - DMA Channel 0 Linked List Item Register"]
    pub lli0: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x10c - DMA Channel 0 Control Register"]
    pub control0: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x110 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config0: crate::Reg<config::CONFIG_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x120 - DMA Channel 0 Source Address Register"]
    pub srcaddr1: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x124 - DMA Channel 0 Destination Address Register"]
    pub destaddr1: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x128 - DMA Channel 0 Linked List Item Register"]
    pub lli1: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x12c - DMA Channel 0 Control Register"]
    pub control1: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x130 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config1: crate::Reg<config::CONFIG_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0x140 - DMA Channel 0 Source Address Register"]
    pub srcaddr2: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x144 - DMA Channel 0 Destination Address Register"]
    pub destaddr2: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x148 - DMA Channel 0 Linked List Item Register"]
    pub lli2: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x14c - DMA Channel 0 Control Register"]
    pub control2: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x150 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config2: crate::Reg<config::CONFIG_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x160 - DMA Channel 0 Source Address Register"]
    pub srcaddr3: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x164 - DMA Channel 0 Destination Address Register"]
    pub destaddr3: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x168 - DMA Channel 0 Linked List Item Register"]
    pub lli3: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x16c - DMA Channel 0 Control Register"]
    pub control3: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x170 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config3: crate::Reg<config::CONFIG_SPEC>,
    _reserved34: [u8; 0x0c],
    #[doc = "0x180 - DMA Channel 0 Source Address Register"]
    pub srcaddr4: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x184 - DMA Channel 0 Destination Address Register"]
    pub destaddr4: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x188 - DMA Channel 0 Linked List Item Register"]
    pub lli4: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x18c - DMA Channel 0 Control Register"]
    pub control4: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x190 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config4: crate::Reg<config::CONFIG_SPEC>,
    _reserved39: [u8; 0x0c],
    #[doc = "0x1a0 - DMA Channel 0 Source Address Register"]
    pub srcaddr5: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x1a4 - DMA Channel 0 Destination Address Register"]
    pub destaddr5: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x1a8 - DMA Channel 0 Linked List Item Register"]
    pub lli5: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x1ac - DMA Channel 0 Control Register"]
    pub control5: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x1b0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config5: crate::Reg<config::CONFIG_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0x1c0 - DMA Channel 0 Source Address Register"]
    pub srcaddr6: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x1c4 - DMA Channel 0 Destination Address Register"]
    pub destaddr6: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x1c8 - DMA Channel 0 Linked List Item Register"]
    pub lli6: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x1cc - DMA Channel 0 Control Register"]
    pub control6: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x1d0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config6: crate::Reg<config::CONFIG_SPEC>,
    _reserved49: [u8; 0x0c],
    #[doc = "0x1e0 - DMA Channel 0 Source Address Register"]
    pub srcaddr7: crate::Reg<srcaddr::SRCADDR_SPEC>,
    #[doc = "0x1e4 - DMA Channel 0 Destination Address Register"]
    pub destaddr7: crate::Reg<destaddr::DESTADDR_SPEC>,
    #[doc = "0x1e8 - DMA Channel 0 Linked List Item Register"]
    pub lli7: crate::Reg<lli::LLI_SPEC>,
    #[doc = "0x1ec - DMA Channel 0 Control Register"]
    pub control7: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x1f0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config7: crate::Reg<config::CONFIG_SPEC>,
}
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "INTTCSTAT register accessor: an alias for `Reg<INTTCSTAT_SPEC>`"]
pub type INTTCSTAT = crate::Reg<inttcstat::INTTCSTAT_SPEC>;
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "INTTCCLEAR register accessor: an alias for `Reg<INTTCCLEAR_SPEC>`"]
pub type INTTCCLEAR = crate::Reg<inttcclear::INTTCCLEAR_SPEC>;
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "INTERRSTAT register accessor: an alias for `Reg<INTERRSTAT_SPEC>`"]
pub type INTERRSTAT = crate::Reg<interrstat::INTERRSTAT_SPEC>;
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "INTERRCLR register accessor: an alias for `Reg<INTERRCLR_SPEC>`"]
pub type INTERRCLR = crate::Reg<interrclr::INTERRCLR_SPEC>;
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "RAWINTTCSTAT register accessor: an alias for `Reg<RAWINTTCSTAT_SPEC>`"]
pub type RAWINTTCSTAT = crate::Reg<rawinttcstat::RAWINTTCSTAT_SPEC>;
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "RAWINTERRSTAT register accessor: an alias for `Reg<RAWINTERRSTAT_SPEC>`"]
pub type RAWINTERRSTAT = crate::Reg<rawinterrstat::RAWINTERRSTAT_SPEC>;
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "ENBLDCHNS register accessor: an alias for `Reg<ENBLDCHNS_SPEC>`"]
pub type ENBLDCHNS = crate::Reg<enbldchns::ENBLDCHNS_SPEC>;
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "SOFTBREQ register accessor: an alias for `Reg<SOFTBREQ_SPEC>`"]
pub type SOFTBREQ = crate::Reg<softbreq::SOFTBREQ_SPEC>;
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "SOFTSREQ register accessor: an alias for `Reg<SOFTSREQ_SPEC>`"]
pub type SOFTSREQ = crate::Reg<softsreq::SOFTSREQ_SPEC>;
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "SOFTLBREQ register accessor: an alias for `Reg<SOFTLBREQ_SPEC>`"]
pub type SOFTLBREQ = crate::Reg<softlbreq::SOFTLBREQ_SPEC>;
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "SOFTLSREQ register accessor: an alias for `Reg<SOFTLSREQ_SPEC>`"]
pub type SOFTLSREQ = crate::Reg<softlsreq::SOFTLSREQ_SPEC>;
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "SRCADDR register accessor: an alias for `Reg<SRCADDR_SPEC>`"]
pub type SRCADDR = crate::Reg<srcaddr::SRCADDR_SPEC>;
#[doc = "DMA Channel 0 Source Address Register"]
pub mod srcaddr;
#[doc = "DESTADDR register accessor: an alias for `Reg<DESTADDR_SPEC>`"]
pub type DESTADDR = crate::Reg<destaddr::DESTADDR_SPEC>;
#[doc = "DMA Channel 0 Destination Address Register"]
pub mod destaddr;
#[doc = "LLI register accessor: an alias for `Reg<LLI_SPEC>`"]
pub type LLI = crate::Reg<lli::LLI_SPEC>;
#[doc = "DMA Channel 0 Linked List Item Register"]
pub mod lli;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "DMA Channel 0 Control Register"]
pub mod control;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub mod config;
