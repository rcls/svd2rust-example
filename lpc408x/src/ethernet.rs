#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register 1."]
    pub mac1: crate::Reg<mac1::MAC1_SPEC>,
    #[doc = "0x04 - MAC configuration register 2."]
    pub mac2: crate::Reg<mac2::MAC2_SPEC>,
    #[doc = "0x08 - Back-to-Back Inter-Packet-Gap register."]
    pub ipgt: crate::Reg<ipgt::IPGT_SPEC>,
    #[doc = "0x0c - Non Back-to-Back Inter-Packet-Gap register."]
    pub ipgr: crate::Reg<ipgr::IPGR_SPEC>,
    #[doc = "0x10 - Collision window / Retry register."]
    pub clrt: crate::Reg<clrt::CLRT_SPEC>,
    #[doc = "0x14 - Maximum Frame register."]
    pub maxf: crate::Reg<maxf::MAXF_SPEC>,
    #[doc = "0x18 - PHY Support register."]
    pub supp: crate::Reg<supp::SUPP_SPEC>,
    #[doc = "0x1c - Test register."]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x20 - MII Mgmt Configuration register."]
    pub mcfg: crate::Reg<mcfg::MCFG_SPEC>,
    #[doc = "0x24 - MII Mgmt Command register."]
    pub mcmd: crate::Reg<mcmd::MCMD_SPEC>,
    #[doc = "0x28 - MII Mgmt Address register."]
    pub madr: crate::Reg<madr::MADR_SPEC>,
    #[doc = "0x2c - MII Mgmt Write Data register."]
    pub mwtd: crate::Reg<mwtd::MWTD_SPEC>,
    #[doc = "0x30 - MII Mgmt Read Data register."]
    pub mrdd: crate::Reg<mrdd::MRDD_SPEC>,
    #[doc = "0x34 - MII Mgmt Indicators register."]
    pub mind: crate::Reg<mind::MIND_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x40 - Station Address 0 register."]
    pub sa0: crate::Reg<sa0::SA0_SPEC>,
    #[doc = "0x44 - Station Address 1 register."]
    pub sa1: crate::Reg<sa1::SA1_SPEC>,
    #[doc = "0x48 - Station Address 2 register."]
    pub sa2: crate::Reg<sa2::SA2_SPEC>,
    _reserved17: [u8; 0xb4],
    #[doc = "0x100 - Command register."]
    pub command: crate::Reg<command::COMMAND_SPEC>,
    #[doc = "0x104 - Status register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x108 - Receive descriptor base address register."]
    pub rxdescriptor: crate::Reg<rxdescriptor::RXDESCRIPTOR_SPEC>,
    #[doc = "0x10c - Receive status base address register."]
    pub rxstatus: crate::Reg<rxstatus::RXSTATUS_SPEC>,
    #[doc = "0x110 - Receive number of descriptors register."]
    pub rxdescriptornumber: crate::Reg<rxdescriptornumber::RXDESCRIPTORNUMBER_SPEC>,
    #[doc = "0x114 - Receive produce index register."]
    pub rxproduceindex: crate::Reg<rxproduceindex::RXPRODUCEINDEX_SPEC>,
    #[doc = "0x118 - Receive consume index register."]
    pub rxconsumeindex: crate::Reg<rxconsumeindex::RXCONSUMEINDEX_SPEC>,
    #[doc = "0x11c - Transmit descriptor base address register."]
    pub txdescriptor: crate::Reg<txdescriptor::TXDESCRIPTOR_SPEC>,
    #[doc = "0x120 - Transmit status base address register."]
    pub txstatus: crate::Reg<txstatus::TXSTATUS_SPEC>,
    #[doc = "0x124 - Transmit number of descriptors register."]
    pub txdescriptornumber: crate::Reg<txdescriptornumber::TXDESCRIPTORNUMBER_SPEC>,
    #[doc = "0x128 - Transmit produce index register."]
    pub txproduceindex: crate::Reg<txproduceindex::TXPRODUCEINDEX_SPEC>,
    #[doc = "0x12c - Transmit consume index register."]
    pub txconsumeindex: crate::Reg<txconsumeindex::TXCONSUMEINDEX_SPEC>,
    _reserved29: [u8; 0x28],
    #[doc = "0x158 - Transmit status vector 0 register."]
    pub tsv0: crate::Reg<tsv0::TSV0_SPEC>,
    #[doc = "0x15c - Transmit status vector 1 register."]
    pub tsv1: crate::Reg<tsv1::TSV1_SPEC>,
    #[doc = "0x160 - Receive status vector register."]
    pub rsv: crate::Reg<rsv::RSV_SPEC>,
    _reserved32: [u8; 0x0c],
    #[doc = "0x170 - Flow control counter register."]
    pub flowcontrolcounter: crate::Reg<flowcontrolcounter::FLOWCONTROLCOUNTER_SPEC>,
    #[doc = "0x174 - Flow control status register."]
    pub flowcontrolstatus: crate::Reg<flowcontrolstatus::FLOWCONTROLSTATUS_SPEC>,
    _reserved34: [u8; 0x88],
    #[doc = "0x200 - Receive filter control register."]
    pub rxfilterctrl: crate::Reg<rxfilterctrl::RXFILTERCTRL_SPEC>,
    #[doc = "0x204 - Receive filter WoL status register."]
    pub rxfilterwolstatus: crate::Reg<rxfilterwolstatus::RXFILTERWOLSTATUS_SPEC>,
    #[doc = "0x208 - Receive filter WoL clear register."]
    pub rxfilterwolclear: crate::Reg<rxfilterwolclear::RXFILTERWOLCLEAR_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x210 - Hash filter table LSBs register."]
    pub hashfilterl: crate::Reg<hashfilterl::HASHFILTERL_SPEC>,
    #[doc = "0x214 - Hash filter table MSBs register."]
    pub hashfilterh: crate::Reg<hashfilterh::HASHFILTERH_SPEC>,
    _reserved39: [u8; 0x0dc8],
    #[doc = "0xfe0 - Interrupt status register."]
    pub intstatus: crate::Reg<intstatus::INTSTATUS_SPEC>,
    #[doc = "0xfe4 - Interrupt enable register."]
    pub intenable: crate::Reg<intenable::INTENABLE_SPEC>,
    #[doc = "0xfe8 - Interrupt clear register."]
    pub intclear: crate::Reg<intclear::INTCLEAR_SPEC>,
    #[doc = "0xfec - Interrupt set register."]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0xff4 - Power-down register."]
    pub powerdown: crate::Reg<powerdown::POWERDOWN_SPEC>,
}
#[doc = "MAC1 register accessor: an alias for `Reg<MAC1_SPEC>`"]
pub type MAC1 = crate::Reg<mac1::MAC1_SPEC>;
#[doc = "MAC configuration register 1."]
pub mod mac1;
#[doc = "MAC2 register accessor: an alias for `Reg<MAC2_SPEC>`"]
pub type MAC2 = crate::Reg<mac2::MAC2_SPEC>;
#[doc = "MAC configuration register 2."]
pub mod mac2;
#[doc = "IPGT register accessor: an alias for `Reg<IPGT_SPEC>`"]
pub type IPGT = crate::Reg<ipgt::IPGT_SPEC>;
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub mod ipgt;
#[doc = "IPGR register accessor: an alias for `Reg<IPGR_SPEC>`"]
pub type IPGR = crate::Reg<ipgr::IPGR_SPEC>;
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub mod ipgr;
#[doc = "CLRT register accessor: an alias for `Reg<CLRT_SPEC>`"]
pub type CLRT = crate::Reg<clrt::CLRT_SPEC>;
#[doc = "Collision window / Retry register."]
pub mod clrt;
#[doc = "MAXF register accessor: an alias for `Reg<MAXF_SPEC>`"]
pub type MAXF = crate::Reg<maxf::MAXF_SPEC>;
#[doc = "Maximum Frame register."]
pub mod maxf;
#[doc = "SUPP register accessor: an alias for `Reg<SUPP_SPEC>`"]
pub type SUPP = crate::Reg<supp::SUPP_SPEC>;
#[doc = "PHY Support register."]
pub mod supp;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test register."]
pub mod test;
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "MII Mgmt Configuration register."]
pub mod mcfg;
#[doc = "MCMD register accessor: an alias for `Reg<MCMD_SPEC>`"]
pub type MCMD = crate::Reg<mcmd::MCMD_SPEC>;
#[doc = "MII Mgmt Command register."]
pub mod mcmd;
#[doc = "MADR register accessor: an alias for `Reg<MADR_SPEC>`"]
pub type MADR = crate::Reg<madr::MADR_SPEC>;
#[doc = "MII Mgmt Address register."]
pub mod madr;
#[doc = "MWTD register accessor: an alias for `Reg<MWTD_SPEC>`"]
pub type MWTD = crate::Reg<mwtd::MWTD_SPEC>;
#[doc = "MII Mgmt Write Data register."]
pub mod mwtd;
#[doc = "MRDD register accessor: an alias for `Reg<MRDD_SPEC>`"]
pub type MRDD = crate::Reg<mrdd::MRDD_SPEC>;
#[doc = "MII Mgmt Read Data register."]
pub mod mrdd;
#[doc = "MIND register accessor: an alias for `Reg<MIND_SPEC>`"]
pub type MIND = crate::Reg<mind::MIND_SPEC>;
#[doc = "MII Mgmt Indicators register."]
pub mod mind;
#[doc = "SA0 register accessor: an alias for `Reg<SA0_SPEC>`"]
pub type SA0 = crate::Reg<sa0::SA0_SPEC>;
#[doc = "Station Address 0 register."]
pub mod sa0;
#[doc = "SA1 register accessor: an alias for `Reg<SA1_SPEC>`"]
pub type SA1 = crate::Reg<sa1::SA1_SPEC>;
#[doc = "Station Address 1 register."]
pub mod sa1;
#[doc = "SA2 register accessor: an alias for `Reg<SA2_SPEC>`"]
pub type SA2 = crate::Reg<sa2::SA2_SPEC>;
#[doc = "Station Address 2 register."]
pub mod sa2;
#[doc = "COMMAND register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command register."]
pub mod command;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register."]
pub mod status;
#[doc = "RXDESCRIPTOR register accessor: an alias for `Reg<RXDESCRIPTOR_SPEC>`"]
pub type RXDESCRIPTOR = crate::Reg<rxdescriptor::RXDESCRIPTOR_SPEC>;
#[doc = "Receive descriptor base address register."]
pub mod rxdescriptor;
#[doc = "RXSTATUS register accessor: an alias for `Reg<RXSTATUS_SPEC>`"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Receive status base address register."]
pub mod rxstatus;
#[doc = "RXDESCRIPTORNUMBER register accessor: an alias for `Reg<RXDESCRIPTORNUMBER_SPEC>`"]
pub type RXDESCRIPTORNUMBER = crate::Reg<rxdescriptornumber::RXDESCRIPTORNUMBER_SPEC>;
#[doc = "Receive number of descriptors register."]
pub mod rxdescriptornumber;
#[doc = "RXPRODUCEINDEX register accessor: an alias for `Reg<RXPRODUCEINDEX_SPEC>`"]
pub type RXPRODUCEINDEX = crate::Reg<rxproduceindex::RXPRODUCEINDEX_SPEC>;
#[doc = "Receive produce index register."]
pub mod rxproduceindex;
#[doc = "RXCONSUMEINDEX register accessor: an alias for `Reg<RXCONSUMEINDEX_SPEC>`"]
pub type RXCONSUMEINDEX = crate::Reg<rxconsumeindex::RXCONSUMEINDEX_SPEC>;
#[doc = "Receive consume index register."]
pub mod rxconsumeindex;
#[doc = "TXDESCRIPTOR register accessor: an alias for `Reg<TXDESCRIPTOR_SPEC>`"]
pub type TXDESCRIPTOR = crate::Reg<txdescriptor::TXDESCRIPTOR_SPEC>;
#[doc = "Transmit descriptor base address register."]
pub mod txdescriptor;
#[doc = "TXSTATUS register accessor: an alias for `Reg<TXSTATUS_SPEC>`"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Transmit status base address register."]
pub mod txstatus;
#[doc = "TXDESCRIPTORNUMBER register accessor: an alias for `Reg<TXDESCRIPTORNUMBER_SPEC>`"]
pub type TXDESCRIPTORNUMBER = crate::Reg<txdescriptornumber::TXDESCRIPTORNUMBER_SPEC>;
#[doc = "Transmit number of descriptors register."]
pub mod txdescriptornumber;
#[doc = "TXPRODUCEINDEX register accessor: an alias for `Reg<TXPRODUCEINDEX_SPEC>`"]
pub type TXPRODUCEINDEX = crate::Reg<txproduceindex::TXPRODUCEINDEX_SPEC>;
#[doc = "Transmit produce index register."]
pub mod txproduceindex;
#[doc = "TXCONSUMEINDEX register accessor: an alias for `Reg<TXCONSUMEINDEX_SPEC>`"]
pub type TXCONSUMEINDEX = crate::Reg<txconsumeindex::TXCONSUMEINDEX_SPEC>;
#[doc = "Transmit consume index register."]
pub mod txconsumeindex;
#[doc = "TSV0 register accessor: an alias for `Reg<TSV0_SPEC>`"]
pub type TSV0 = crate::Reg<tsv0::TSV0_SPEC>;
#[doc = "Transmit status vector 0 register."]
pub mod tsv0;
#[doc = "TSV1 register accessor: an alias for `Reg<TSV1_SPEC>`"]
pub type TSV1 = crate::Reg<tsv1::TSV1_SPEC>;
#[doc = "Transmit status vector 1 register."]
pub mod tsv1;
#[doc = "RSV register accessor: an alias for `Reg<RSV_SPEC>`"]
pub type RSV = crate::Reg<rsv::RSV_SPEC>;
#[doc = "Receive status vector register."]
pub mod rsv;
#[doc = "FLOWCONTROLCOUNTER register accessor: an alias for `Reg<FLOWCONTROLCOUNTER_SPEC>`"]
pub type FLOWCONTROLCOUNTER = crate::Reg<flowcontrolcounter::FLOWCONTROLCOUNTER_SPEC>;
#[doc = "Flow control counter register."]
pub mod flowcontrolcounter;
#[doc = "FLOWCONTROLSTATUS register accessor: an alias for `Reg<FLOWCONTROLSTATUS_SPEC>`"]
pub type FLOWCONTROLSTATUS = crate::Reg<flowcontrolstatus::FLOWCONTROLSTATUS_SPEC>;
#[doc = "Flow control status register."]
pub mod flowcontrolstatus;
#[doc = "RXFILTERCTRL register accessor: an alias for `Reg<RXFILTERCTRL_SPEC>`"]
pub type RXFILTERCTRL = crate::Reg<rxfilterctrl::RXFILTERCTRL_SPEC>;
#[doc = "Receive filter control register."]
pub mod rxfilterctrl;
#[doc = "RXFILTERWOLSTATUS register accessor: an alias for `Reg<RXFILTERWOLSTATUS_SPEC>`"]
pub type RXFILTERWOLSTATUS = crate::Reg<rxfilterwolstatus::RXFILTERWOLSTATUS_SPEC>;
#[doc = "Receive filter WoL status register."]
pub mod rxfilterwolstatus;
#[doc = "RXFILTERWOLCLEAR register accessor: an alias for `Reg<RXFILTERWOLCLEAR_SPEC>`"]
pub type RXFILTERWOLCLEAR = crate::Reg<rxfilterwolclear::RXFILTERWOLCLEAR_SPEC>;
#[doc = "Receive filter WoL clear register."]
pub mod rxfilterwolclear;
#[doc = "HASHFILTERL register accessor: an alias for `Reg<HASHFILTERL_SPEC>`"]
pub type HASHFILTERL = crate::Reg<hashfilterl::HASHFILTERL_SPEC>;
#[doc = "Hash filter table LSBs register."]
pub mod hashfilterl;
#[doc = "HASHFILTERH register accessor: an alias for `Reg<HASHFILTERH_SPEC>`"]
pub type HASHFILTERH = crate::Reg<hashfilterh::HASHFILTERH_SPEC>;
#[doc = "Hash filter table MSBs register."]
pub mod hashfilterh;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt status register."]
pub mod intstatus;
#[doc = "INTENABLE register accessor: an alias for `Reg<INTENABLE_SPEC>`"]
pub type INTENABLE = crate::Reg<intenable::INTENABLE_SPEC>;
#[doc = "Interrupt enable register."]
pub mod intenable;
#[doc = "INTCLEAR register accessor: an alias for `Reg<INTCLEAR_SPEC>`"]
pub type INTCLEAR = crate::Reg<intclear::INTCLEAR_SPEC>;
#[doc = "Interrupt clear register."]
pub mod intclear;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "Interrupt set register."]
pub mod intset;
#[doc = "POWERDOWN register accessor: an alias for `Reg<POWERDOWN_SPEC>`"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "Power-down register."]
pub mod powerdown;
