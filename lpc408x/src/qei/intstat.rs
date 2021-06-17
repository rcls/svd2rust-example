#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INX_INT` reader - Indicates that an index pulse was detected."]
pub struct INX_INT_R(crate::FieldReader<bool, bool>);
impl INX_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INX_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INX_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM_INT` reader - Indicates that a velocity timer overflow occurred"]
pub struct TIM_INT_R(crate::FieldReader<bool, bool>);
impl TIM_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VELC_INT` reader - Indicates that captured velocity is less than compare velocity."]
pub struct VELC_INT_R(crate::FieldReader<bool, bool>);
impl VELC_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        VELC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VELC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR_INT` reader - Indicates that a change of direction was detected."]
pub struct DIR_INT_R(crate::FieldReader<bool, bool>);
impl DIR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_INT` reader - Indicates that an encoder phase error was detected."]
pub struct ERR_INT_R(crate::FieldReader<bool, bool>);
impl ERR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCLK_INT` reader - Indicates that and encoder clock pulse was detected."]
pub struct ENCLK_INT_R(crate::FieldReader<bool, bool>);
impl ENCLK_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENCLK_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCLK_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS0_INT` reader - Indicates that the position 0 compare value is equal to the current position."]
pub struct POS0_INT_R(crate::FieldReader<bool, bool>);
impl POS0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS1_INT` reader - Indicates that the position 1compare value is equal to the current position."]
pub struct POS1_INT_R(crate::FieldReader<bool, bool>);
impl POS1_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS1_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS1_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS2_INT` reader - Indicates that the position 2 compare value is equal to the current position."]
pub struct POS2_INT_R(crate::FieldReader<bool, bool>);
impl POS2_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS2_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS2_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV0_INT` reader - Indicates that the index compare 0 value is equal to the current index count."]
pub struct REV0_INT_R(crate::FieldReader<bool, bool>);
impl REV0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS0REV_INT` reader - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
pub struct POS0REV_INT_R(crate::FieldReader<bool, bool>);
impl POS0REV_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS0REV_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS0REV_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS1REV_INT` reader - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
pub struct POS1REV_INT_R(crate::FieldReader<bool, bool>);
impl POS1REV_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS1REV_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS1REV_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS2REV_INT` reader - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
pub struct POS2REV_INT_R(crate::FieldReader<bool, bool>);
impl POS2REV_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        POS2REV_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS2REV_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV1_INT` reader - Indicates that the index compare 1value is equal to the current index count."]
pub struct REV1_INT_R(crate::FieldReader<bool, bool>);
impl REV1_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV1_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV1_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV2_INT` reader - Indicates that the index compare 2 value is equal to the current index count."]
pub struct REV2_INT_R(crate::FieldReader<bool, bool>);
impl REV2_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV2_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV2_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXPOS_INT` reader - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
pub struct MAXPOS_INT_R(crate::FieldReader<bool, bool>);
impl MAXPOS_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAXPOS_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXPOS_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that an index pulse was detected."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that a velocity timer overflow occurred"]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that captured velocity is less than compare velocity."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that a change of direction was detected."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that an encoder phase error was detected."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that and encoder clock pulse was detected."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that the position 0 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that the position 1compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates that the position 2 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates that the index compare 0 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicates that the index compare 1value is equal to the current index count."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Indicates that the index compare 2 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
