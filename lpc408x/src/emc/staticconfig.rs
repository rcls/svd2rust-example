#[doc = "Register `STATICCONFIG%s` reader"]
pub struct R(crate::R<STATICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONFIG%s` writer"]
pub struct W(crate::W<STATICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MW_A {
    #[doc = "0: 8 bit (POR reset value)."]
    _8_BIT_POR_RESET_VAL = 0,
    #[doc = "1: 16 bit."]
    _16_BIT_ = 1,
    #[doc = "2: 32 bit."]
    _32_BIT_ = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<MW_A> for u8 {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MW` reader - Memory width."]
pub struct MW_R(crate::FieldReader<u8, MW_A>);
impl MW_R {
    pub(crate) fn new(bits: u8) -> Self {
        MW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MW_A {
        match self.bits {
            0 => MW_A::_8_BIT_POR_RESET_VAL,
            1 => MW_A::_16_BIT_,
            2 => MW_A::_32_BIT_,
            3 => MW_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT_POR_RESET_VAL`"]
    #[inline(always)]
    pub fn is_8_bit_por_reset_val(&self) -> bool {
        **self == MW_A::_8_BIT_POR_RESET_VAL
    }
    #[doc = "Checks if the value of the field is `_16_BIT_`"]
    #[inline(always)]
    pub fn is_16_bit_(&self) -> bool {
        **self == MW_A::_16_BIT_
    }
    #[doc = "Checks if the value of the field is `_32_BIT_`"]
    #[inline(always)]
    pub fn is_32_bit_(&self) -> bool {
        **self == MW_A::_32_BIT_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == MW_A::RESERVED_
    }
}
impl core::ops::Deref for MW_R {
    type Target = crate::FieldReader<u8, MW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MW` writer - Memory width."]
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 bit (POR reset value)."]
    #[inline(always)]
    pub fn _8_bit_por_reset_val(self) -> &'a mut W {
        self.variant(MW_A::_8_BIT_POR_RESET_VAL)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16_bit_(self) -> &'a mut W {
        self.variant(MW_A::_16_BIT_)
    }
    #[doc = "32 bit."]
    #[inline(always)]
    pub fn _32_bit_(self) -> &'a mut W {
        self.variant(MW_A::_32_BIT_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(MW_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: Disabled (POR reset value)."]
    DISABLED_POR_RESET_ = 0,
    #[doc = "1: Asynchronous page mode enabled (page length four)."]
    ASYNCHRONOUS_PAGE_MO = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
pub struct PM_R(crate::FieldReader<bool, PM_A>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::DISABLED_POR_RESET_,
            true => PM_A::ASYNCHRONOUS_PAGE_MO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_POR_RESET_`"]
    #[inline(always)]
    pub fn is_disabled_por_reset_(&self) -> bool {
        **self == PM_A::DISABLED_POR_RESET_
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_PAGE_MO`"]
    #[inline(always)]
    pub fn is_asynchronous_page_mo(&self) -> bool {
        **self == PM_A::ASYNCHRONOUS_PAGE_MO
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled (POR reset value)."]
    #[inline(always)]
    pub fn disabled_por_reset_(self) -> &'a mut W {
        self.variant(PM_A::DISABLED_POR_RESET_)
    }
    #[doc = "Asynchronous page mode enabled (page length four)."]
    #[inline(always)]
    pub fn asynchronous_page_mo(self) -> &'a mut W {
        self.variant(PM_A::ASYNCHRONOUS_PAGE_MO)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Chip select polarity. The value of the chip select polarity on power-on reset is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC_A {
    #[doc = "0: Active LOW chip select."]
    ACTIVE_LOW_CHIP_SELE = 0,
    #[doc = "1: Active HIGH chip select."]
    ACTIVE_HIGH_CHIP_SEL = 1,
}
impl From<PC_A> for bool {
    #[inline(always)]
    fn from(variant: PC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC` reader - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
pub struct PC_R(crate::FieldReader<bool, PC_A>);
impl PC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC_A {
        match self.bits {
            false => PC_A::ACTIVE_LOW_CHIP_SELE,
            true => PC_A::ACTIVE_HIGH_CHIP_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_CHIP_SELE`"]
    #[inline(always)]
    pub fn is_active_low_chip_sele(&self) -> bool {
        **self == PC_A::ACTIVE_LOW_CHIP_SELE
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_CHIP_SEL`"]
    #[inline(always)]
    pub fn is_active_high_chip_sel(&self) -> bool {
        **self == PC_A::ACTIVE_HIGH_CHIP_SEL
    }
}
impl core::ops::Deref for PC_R {
    type Target = crate::FieldReader<bool, PC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC` writer - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active LOW chip select."]
    #[inline(always)]
    pub fn active_low_chip_sele(self) -> &'a mut W {
        self.variant(PC_A::ACTIVE_LOW_CHIP_SELE)
    }
    #[doc = "Active HIGH chip select."]
    #[inline(always)]
    pub fn active_high_chip_sel(self) -> &'a mut W {
        self.variant(PC_A::ACTIVE_HIGH_CHIP_SEL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_A {
    #[doc = "0: For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    BLSHIGH = 0,
    #[doc = "1: For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    BLSLOW = 1,
}
impl From<PB_A> for bool {
    #[inline(always)]
    fn from(variant: PB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB` reader - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
pub struct PB_R(crate::FieldReader<bool, PB_A>);
impl PB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB_A {
        match self.bits {
            false => PB_A::BLSHIGH,
            true => PB_A::BLSLOW,
        }
    }
    #[doc = "Checks if the value of the field is `BLSHIGH`"]
    #[inline(always)]
    pub fn is_blshigh(&self) -> bool {
        **self == PB_A::BLSHIGH
    }
    #[doc = "Checks if the value of the field is `BLSLOW`"]
    #[inline(always)]
    pub fn is_blslow(&self) -> bool {
        **self == PB_A::BLSLOW
    }
}
impl core::ops::Deref for PB_R {
    type Target = crate::FieldReader<bool, PB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB` writer - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    #[inline(always)]
    pub fn blshigh(self) -> &'a mut W {
        self.variant(PB_A::BLSHIGH)
    }
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    #[inline(always)]
    pub fn blslow(self) -> &'a mut W {
        self.variant(PB_A::BLSLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    #[doc = "0: Extended wait disabled (POR reset value)."]
    EXTENDED_WAIT_DISABL = 0,
    #[doc = "1: Extended wait enabled."]
    EXTENDED_WAIT_ENABLE = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW` reader - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
pub struct EW_R(crate::FieldReader<bool, EW_A>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::EXTENDED_WAIT_DISABL,
            true => EW_A::EXTENDED_WAIT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_DISABL`"]
    #[inline(always)]
    pub fn is_extended_wait_disabl(&self) -> bool {
        **self == EW_A::EXTENDED_WAIT_DISABL
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_ENABLE`"]
    #[inline(always)]
    pub fn is_extended_wait_enable(&self) -> bool {
        **self == EW_A::EXTENDED_WAIT_ENABLE
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, EW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EW` writer - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Extended wait disabled (POR reset value)."]
    #[inline(always)]
    pub fn extended_wait_disabl(self) -> &'a mut W {
        self.variant(EW_A::EXTENDED_WAIT_DISABL)
    }
    #[doc = "Extended wait enabled."]
    #[inline(always)]
    pub fn extended_wait_enable(self) -> &'a mut W {
        self.variant(EW_A::EXTENDED_WAIT_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Buffer enable \\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B_A {
    #[doc = "0: Buffer disabled (POR reset value)."]
    BUFFER_DISABLED_POR = 0,
    #[doc = "1: Buffer enabled."]
    BUFFER_ENABLED_ = 1,
}
impl From<B_A> for bool {
    #[inline(always)]
    fn from(variant: B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B` reader - Buffer enable \\[2\\]"]
pub struct B_R(crate::FieldReader<bool, B_A>);
impl B_R {
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B_A {
        match self.bits {
            false => B_A::BUFFER_DISABLED_POR,
            true => B_A::BUFFER_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFER_DISABLED_POR`"]
    #[inline(always)]
    pub fn is_buffer_disabled_por(&self) -> bool {
        **self == B_A::BUFFER_DISABLED_POR
    }
    #[doc = "Checks if the value of the field is `BUFFER_ENABLED_`"]
    #[inline(always)]
    pub fn is_buffer_enabled_(&self) -> bool {
        **self == B_A::BUFFER_ENABLED_
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B` writer - Buffer enable \\[2\\]"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Buffer disabled (POR reset value)."]
    #[inline(always)]
    pub fn buffer_disabled_por(self) -> &'a mut W {
        self.variant(B_A::BUFFER_DISABLED_POR)
    }
    #[doc = "Buffer enabled."]
    #[inline(always)]
    pub fn buffer_enabled_(self) -> &'a mut W {
        self.variant(B_A::BUFFER_ENABLED_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P_A {
    #[doc = "0: Writes not protected (POR reset value)."]
    WRITES_NOT_PROTECTED = 0,
    #[doc = "1: Write protected."]
    WRITE_PROTECTED_ = 1,
}
impl From<P_A> for bool {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P` reader - Write protect"]
pub struct P_R(crate::FieldReader<bool, P_A>);
impl P_R {
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_A {
        match self.bits {
            false => P_A::WRITES_NOT_PROTECTED,
            true => P_A::WRITE_PROTECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `WRITES_NOT_PROTECTED`"]
    #[inline(always)]
    pub fn is_writes_not_protected(&self) -> bool {
        **self == P_A::WRITES_NOT_PROTECTED
    }
    #[doc = "Checks if the value of the field is `WRITE_PROTECTED_`"]
    #[inline(always)]
    pub fn is_write_protected_(&self) -> bool {
        **self == P_A::WRITE_PROTECTED_
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P` writer - Write protect"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn writes_not_protected(self) -> &'a mut W {
        self.variant(P_A::WRITES_NOT_PROTECTED)
    }
    #[doc = "Write protected."]
    #[inline(always)]
    pub fn write_protected_(self) -> &'a mut W {
        self.variant(P_A::WRITE_PROTECTED_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for EMC_CS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticconfig](index.html) module"]
pub struct STATICCONFIG_SPEC;
impl crate::RegisterSpec for STATICCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticconfig::R](R) reader structure"]
impl crate::Readable for STATICCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticconfig::W](W) writer structure"]
impl crate::Writable for STATICCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICCONFIG%s to value 0"]
impl crate::Resettable for STATICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
