#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls the operating mode of the CAN Controller."]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x04 - Command bits that affect the state of the CAN Controller"]
    pub cmr: crate::Reg<cmr::CMR_SPEC>,
    #[doc = "0x08 - Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
    pub gsr: crate::Reg<gsr::GSR_SPEC>,
    #[doc = "0x0c - Interrupt status, Arbitration Lost Capture, Error Code Capture"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Bus Timing. Can only be written when RM in CANMOD is 1."]
    pub btr: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x18 - Error Warning Limit. Can only be written when RM in CANMOD is 1."]
    pub ewl: crate::Reg<ewl::EWL_SPEC>,
    #[doc = "0x1c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x20 - Receive frame status. Can only be written when RM in CANMOD is 1."]
    pub rfs: crate::Reg<rfs::RFS_SPEC>,
    #[doc = "0x24 - Received Identifier. Can only be written when RM in CANMOD is 1."]
    pub rid: crate::Reg<rid::RID_SPEC>,
    #[doc = "0x28 - Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
    pub rda: crate::Reg<rda::RDA_SPEC>,
    #[doc = "0x2c - Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
    pub rdb: crate::Reg<rdb::RDB_SPEC>,
    #[doc = "0x30 - Transmit frame info (Tx Buffer )"]
    pub tfi1: crate::Reg<tfi::TFI_SPEC>,
    #[doc = "0x34 - Transmit Identifier (Tx Buffer)"]
    pub tid1: crate::Reg<tid::TID_SPEC>,
    #[doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda1: crate::Reg<tda::TDA_SPEC>,
    #[doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb1: crate::Reg<tdb::TDB_SPEC>,
    #[doc = "0x40 - Transmit frame info (Tx Buffer )"]
    pub tfi2: crate::Reg<tfi::TFI_SPEC>,
    #[doc = "0x44 - Transmit Identifier (Tx Buffer)"]
    pub tid2: crate::Reg<tid::TID_SPEC>,
    #[doc = "0x48 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda2: crate::Reg<tda::TDA_SPEC>,
    #[doc = "0x4c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb2: crate::Reg<tdb::TDB_SPEC>,
    #[doc = "0x50 - Transmit frame info (Tx Buffer )"]
    pub tfi3: crate::Reg<tfi::TFI_SPEC>,
    #[doc = "0x54 - Transmit Identifier (Tx Buffer)"]
    pub tid3: crate::Reg<tid::TID_SPEC>,
    #[doc = "0x58 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda3: crate::Reg<tda::TDA_SPEC>,
    #[doc = "0x5c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb3: crate::Reg<tdb::TDB_SPEC>,
}
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Controls the operating mode of the CAN Controller."]
pub mod mod_;
#[doc = "CMR register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Command bits that affect the state of the CAN Controller"]
pub mod cmr;
#[doc = "GSR register accessor: an alias for `Reg<GSR_SPEC>`"]
pub type GSR = crate::Reg<gsr::GSR_SPEC>;
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub mod gsr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub mod icr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub mod btr;
#[doc = "EWL register accessor: an alias for `Reg<EWL_SPEC>`"]
pub type EWL = crate::Reg<ewl::EWL_SPEC>;
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub mod ewl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RFS register accessor: an alias for `Reg<RFS_SPEC>`"]
pub type RFS = crate::Reg<rfs::RFS_SPEC>;
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub mod rfs;
#[doc = "RID register accessor: an alias for `Reg<RID_SPEC>`"]
pub type RID = crate::Reg<rid::RID_SPEC>;
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub mod rid;
#[doc = "RDA register accessor: an alias for `Reg<RDA_SPEC>`"]
pub type RDA = crate::Reg<rda::RDA_SPEC>;
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub mod rda;
#[doc = "RDB register accessor: an alias for `Reg<RDB_SPEC>`"]
pub type RDB = crate::Reg<rdb::RDB_SPEC>;
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub mod rdb;
#[doc = "TFI register accessor: an alias for `Reg<TFI_SPEC>`"]
pub type TFI = crate::Reg<tfi::TFI_SPEC>;
#[doc = "Transmit frame info (Tx Buffer )"]
pub mod tfi;
#[doc = "TID register accessor: an alias for `Reg<TID_SPEC>`"]
pub type TID = crate::Reg<tid::TID_SPEC>;
#[doc = "Transmit Identifier (Tx Buffer)"]
pub mod tid;
#[doc = "TDA register accessor: an alias for `Reg<TDA_SPEC>`"]
pub type TDA = crate::Reg<tda::TDA_SPEC>;
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub mod tda;
#[doc = "TDB register accessor: an alias for `Reg<TDB_SPEC>`"]
pub type TDB = crate::Reg<tdb::TDB_SPEC>;
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub mod tdb;
