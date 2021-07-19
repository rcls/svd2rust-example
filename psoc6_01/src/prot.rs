#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x23e8 - SMPU"]
    pub smpu: SMPU,
    _reserved1: [u8; 0x1c18],
    #[doc = "0x4000 - MPU"]
    pub mpu: crate::ArrayProxy<MPU, 16, 0x0400>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU {
    #[doc = "0x00 - Master 0 protection context control"]
    pub ms0_ctl: crate::Reg<self::smpu::ms0_ctl::MS0_CTL_SPEC>,
    #[doc = "0x04 - Master 1 protection context control"]
    pub ms1_ctl: crate::Reg<self::smpu::ms1_ctl::MS1_CTL_SPEC>,
    #[doc = "0x08 - Master 2 protection context control"]
    pub ms2_ctl: crate::Reg<self::smpu::ms2_ctl::MS2_CTL_SPEC>,
    #[doc = "0x0c - Master 3 protection context control"]
    pub ms3_ctl: crate::Reg<self::smpu::ms3_ctl::MS3_CTL_SPEC>,
    #[doc = "0x10 - Master 4 protection context control"]
    pub ms4_ctl: crate::Reg<self::smpu::ms4_ctl::MS4_CTL_SPEC>,
    #[doc = "0x14 - Master 5 protection context control"]
    pub ms5_ctl: crate::Reg<self::smpu::ms5_ctl::MS5_CTL_SPEC>,
    #[doc = "0x18 - Master 6 protection context control"]
    pub ms6_ctl: crate::Reg<self::smpu::ms6_ctl::MS6_CTL_SPEC>,
    #[doc = "0x1c - Master 7 protection context control"]
    pub ms7_ctl: crate::Reg<self::smpu::ms7_ctl::MS7_CTL_SPEC>,
    #[doc = "0x20 - Master 8 protection context control"]
    pub ms8_ctl: crate::Reg<self::smpu::ms8_ctl::MS8_CTL_SPEC>,
    #[doc = "0x24 - Master 9 protection context control"]
    pub ms9_ctl: crate::Reg<self::smpu::ms9_ctl::MS9_CTL_SPEC>,
    #[doc = "0x28 - Master 10 protection context control"]
    pub ms10_ctl: crate::Reg<self::smpu::ms10_ctl::MS10_CTL_SPEC>,
    #[doc = "0x2c - Master 11 protection context control"]
    pub ms11_ctl: crate::Reg<self::smpu::ms11_ctl::MS11_CTL_SPEC>,
    #[doc = "0x30 - Master 12 protection context control"]
    pub ms12_ctl: crate::Reg<self::smpu::ms12_ctl::MS12_CTL_SPEC>,
    #[doc = "0x34 - Master 13 protection context control"]
    pub ms13_ctl: crate::Reg<self::smpu::ms13_ctl::MS13_CTL_SPEC>,
    #[doc = "0x38 - Master 14 protection context control"]
    pub ms14_ctl: crate::Reg<self::smpu::ms14_ctl::MS14_CTL_SPEC>,
    #[doc = "0x3c - Master 15 protection context control"]
    pub ms15_ctl: crate::Reg<self::smpu::ms15_ctl::MS15_CTL_SPEC>,
    _reserved16: [u8; 0x1fc0],
    #[doc = "0x2000 - SMPU structure"]
    pub smpu_struct: crate::ArrayProxy<self::smpu::SMPU_STRUCT, 16, 0x40>,
}
#[doc = r"Register block"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    #[doc = "0x00 - Master control"]
    pub ms_ctl: crate::Reg<self::mpu::ms_ctl::MS_CTL_SPEC>,
    #[doc = "0x04..0x200 - Master control read mirror"]
    pub ms_ctl_read_mir: [crate::Reg<self::mpu::ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>; 127],
    #[doc = "0x200 - MPU structure"]
    pub mpu_struct: crate::ArrayProxy<self::mpu::MPU_STRUCT, 8, 0x20>,
}
#[doc = r"Register block"]
#[doc = "MPU"]
pub mod mpu;
