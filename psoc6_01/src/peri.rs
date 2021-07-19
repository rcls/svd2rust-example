#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral group structure"]
    pub gr: crate::ArrayProxy<GR, 11, 0x40>,
    _reserved1: [u8; 0x0400],
    #[doc = "0x400 - Divider command register"]
    pub div_cmd: crate::Reg<div_cmd::DIV_CMD_SPEC>,
    _reserved2: [u8; 0x03fc],
    #[doc = "0x800..0x900 - Divider control register (for 8.0 divider)"]
    pub div_8_ctl: [crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>; 64],
    #[doc = "0x900..0xa00 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl: [crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>; 64],
    #[doc = "0xa00..0xb00 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl: [crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>; 64],
    #[doc = "0xb00..0xbfc - Divider control register (for 24.5 divider)"]
    pub div_24_5_ctl: [crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>; 63],
    _reserved6: [u8; 0x04],
    #[doc = "0xc00..0xe00 - Clock control register"]
    pub clock_ctl: [crate::Reg<clock_ctl::CLOCK_CTL_SPEC>; 128],
    _reserved7: [u8; 0x0200],
    #[doc = "0x1000 - Trigger command register"]
    pub tr_cmd: crate::Reg<tr_cmd::TR_CMD_SPEC>,
    _reserved8: [u8; 0x0ffc],
    #[doc = "0x2000..0x3e00 - Trigger group"]
    pub tr_gr: [TR_GR; 15],
    _reserved9: [u8; 0x0200],
    #[doc = "0x4000 - PPU structure with programmable address"]
    pub ppu_pr: crate::ArrayProxy<PPU_PR, 16, 0x40>,
    _reserved10: [u8; 0x1000],
    #[doc = "0x5000 - PPU structure with fixed/constant address for a peripheral group"]
    pub ppu_gr: crate::ArrayProxy<PPU_GR, 11, 0x40>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Clock control"]
    pub clock_ctl: crate::Reg<self::gr::clock_ctl::CLOCK_CTL_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Slave control"]
    pub sl_ctl: crate::Reg<self::gr::sl_ctl::SL_CTL_SPEC>,
    #[doc = "0x24 - Timeout control"]
    pub timeout_ctl: crate::Reg<self::gr::timeout_ctl::TIMEOUT_CTL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GR {
    #[doc = "0x00..0x200 - Trigger control register"]
    pub tr_out_ctl: [crate::Reg<self::tr_gr::tr_out_ctl::TR_OUT_CTL_SPEC>; 128],
}
#[doc = r"Register block"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_PR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: crate::Reg<self::ppu_pr::addr0::ADDR0_SPEC>,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: crate::Reg<self::ppu_pr::att0::ATT0_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: crate::Reg<self::ppu_pr::addr1::ADDR1_SPEC>,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: crate::Reg<self::ppu_pr::att1::ATT1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPU structure with programmable address"]
pub mod ppu_pr;
#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_GR {
    #[doc = "0x00 - PPU region address 0 (slave structure)"]
    pub addr0: crate::Reg<self::ppu_gr::addr0::ADDR0_SPEC>,
    #[doc = "0x04 - PPU region attributes 0 (slave structure)"]
    pub att0: crate::Reg<self::ppu_gr::att0::ATT0_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - PPU region address 1 (master structure)"]
    pub addr1: crate::Reg<self::ppu_gr::addr1::ADDR1_SPEC>,
    #[doc = "0x24 - PPU region attributes 1 (master structure)"]
    pub att1: crate::Reg<self::ppu_gr::att1::ATT1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPU structure with fixed/constant address for a peripheral group"]
pub mod ppu_gr;
#[doc = "DIV_CMD register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command register"]
pub mod div_cmd;
#[doc = "DIV_8_CTL register accessor: an alias for `Reg<DIV_8_CTL_SPEC>`"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control register (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL register accessor: an alias for `Reg<DIV_16_CTL_SPEC>`"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL register accessor: an alias for `Reg<DIV_16_5_CTL_SPEC>`"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL register accessor: an alias for `Reg<DIV_24_5_CTL_SPEC>`"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control register (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "CLOCK_CTL register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control register"]
pub mod clock_ctl;
#[doc = "TR_CMD register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Trigger command register"]
pub mod tr_cmd;
