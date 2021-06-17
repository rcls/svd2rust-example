#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INX_INT` reader - When 1, the INX_Int interrupt is enabled."]
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
#[doc = "Field `TIM_INT` reader - When 1, the TIN_Int interrupt is enabled."]
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
#[doc = "Field `VELC_INT` reader - When 1, the VELC_Int interrupt is enabled."]
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
#[doc = "Field `DIR_INT` reader - When 1, the DIR_Int interrupt is enabled."]
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
#[doc = "Field `ERR_INT` reader - When 1, the ERR_Int interrupt is enabled."]
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
#[doc = "Field `ENCLK_INT` reader - When 1, the ENCLK_Int interrupt is enabled."]
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
#[doc = "Field `POS0_INT` reader - When 1, the POS0_Int interrupt is enabled."]
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
#[doc = "Field `POS1_INT` reader - When 1, the POS1_Int interrupt is enabled."]
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
#[doc = "Field `POS2_INT` reader - When 1, the POS2_Int interrupt is enabled."]
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
#[doc = "Field `REV0_INT` reader - When 1, the REV0_Int interrupt is enabled."]
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
#[doc = "Field `POS0REV_INT` reader - When 1, the POS0REV_Int interrupt is enabled."]
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
#[doc = "Field `POS1REV_INT` reader - When 1, the POS1REV_Int interrupt is enabled."]
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
#[doc = "Field `POS2REV_INT` reader - When 1, the POS2REV_Int interrupt is enabled."]
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
#[doc = "Field `REV1_INT` reader - When 1, the REV1_Int interrupt is enabled."]
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
#[doc = "Field `REV2_INT` reader - When 1, the REV2_Int interrupt is enabled."]
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
#[doc = "Field `MAXPOS_INT` reader - When 1, the MAXPOS_Int interrupt is enabled."]
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
    #[doc = "Bit 0 - When 1, the INX_Int interrupt is enabled."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the TIN_Int interrupt is enabled."]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, the VELC_Int interrupt is enabled."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, the DIR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, the ERR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, the ENCLK_Int interrupt is enabled."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, the POS0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, the POS1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, the POS2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, the REV0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When 1, the POS0REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When 1, the POS1REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When 1, the POS2REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When 1, the REV1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - When 1, the REV2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When 1, the MAXPOS_Int interrupt is enabled."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
