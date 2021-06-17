#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOS_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct EOS_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl EOS_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOS_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOS_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct OVERFLOW_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct FW_COLLISION_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl FW_COLLISION_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FW_COLLISION_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_COLLISION_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct DSI_COLLISION_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl DSI_COLLISION_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_COLLISION_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_COLLISION_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_EOC_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct INJ_EOC_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl INJ_EOC_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_EOC_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_EOC_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_SATURATE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct INJ_SATURATE_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl INJ_SATURATE_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_SATURATE_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_SATURATE_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_RANGE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct INJ_RANGE_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl INJ_RANGE_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_RANGE_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_RANGE_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub struct INJ_COLLISION_MASKED_MIR_R(crate::FieldReader<bool, bool>);
impl INJ_COLLISION_MASKED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_COLLISION_MASKED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_COLLISION_MASKED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SATURATE_MASKED_RED` reader - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
pub struct SATURATE_MASKED_RED_R(crate::FieldReader<bool, bool>);
impl SATURATE_MASKED_RED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SATURATE_MASKED_RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SATURATE_MASKED_RED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE_MASKED_RED` reader - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
pub struct RANGE_MASKED_RED_R(crate::FieldReader<bool, bool>);
impl RANGE_MASKED_RED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANGE_MASKED_RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANGE_MASKED_RED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn eos_masked_mir(&self) -> EOS_MASKED_MIR_R {
        EOS_MASKED_MIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn overflow_masked_mir(&self) -> OVERFLOW_MASKED_MIR_R {
        OVERFLOW_MASKED_MIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn fw_collision_masked_mir(&self) -> FW_COLLISION_MASKED_MIR_R {
        FW_COLLISION_MASKED_MIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn dsi_collision_masked_mir(&self) -> DSI_COLLISION_MASKED_MIR_R {
        DSI_COLLISION_MASKED_MIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_eoc_masked_mir(&self) -> INJ_EOC_MASKED_MIR_R {
        INJ_EOC_MASKED_MIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_saturate_masked_mir(&self) -> INJ_SATURATE_MASKED_MIR_R {
        INJ_SATURATE_MASKED_MIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_range_masked_mir(&self) -> INJ_RANGE_MASKED_MIR_R {
        INJ_RANGE_MASKED_MIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_collision_masked_mir(&self) -> INJ_COLLISION_MASKED_MIR_R {
        INJ_COLLISION_MASKED_MIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn saturate_masked_red(&self) -> SATURATE_MASKED_RED_R {
        SATURATE_MASKED_RED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn range_masked_red(&self) -> RANGE_MASKED_RED_R {
        RANGE_MASKED_RED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
