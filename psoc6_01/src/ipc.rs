#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPC structure"]
    pub struct_: crate::ArrayProxy<STRUCT, 16, 0x20>,
    _reserved1: [u8; 0x1000],
    #[doc = "0x1000 - IPC interrupt structure"]
    pub intr_struct: crate::ArrayProxy<INTR_STRUCT, 16, 0x20>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - IPC acquire"]
    pub acquire: crate::Reg<self::struct_::acquire::ACQUIRE_SPEC>,
    #[doc = "0x04 - IPC release"]
    pub release: crate::Reg<self::struct_::release::RELEASE_SPEC>,
    #[doc = "0x08 - IPC notification"]
    pub notify: crate::Reg<self::struct_::notify::NOTIFY_SPEC>,
    #[doc = "0x0c - IPC data"]
    pub data: crate::Reg<self::struct_::data::DATA_SPEC>,
    #[doc = "0x10 - IPC lock status"]
    pub lock_status: crate::Reg<self::struct_::lock_status::LOCK_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTR_STRUCT {
    #[doc = "0x00 - Interrupt"]
    pub intr: crate::Reg<self::intr_struct::intr::INTR_SPEC>,
    #[doc = "0x04 - Interrupt set"]
    pub intr_set: crate::Reg<self::intr_struct::intr_set::INTR_SET_SPEC>,
    #[doc = "0x08 - Interrupt mask"]
    pub intr_mask: crate::Reg<self::intr_struct::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x0c - Interrupt masked"]
    pub intr_masked: crate::Reg<self::intr_struct::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;
