#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOS_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct EOS_MASKED_R(crate::FieldReader<bool, bool>);
impl EOS_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOS_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOS_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct OVERFLOW_MASKED_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct FW_COLLISION_MASKED_R(crate::FieldReader<bool, bool>);
impl FW_COLLISION_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        FW_COLLISION_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_COLLISION_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct DSI_COLLISION_MASKED_R(crate::FieldReader<bool, bool>);
impl DSI_COLLISION_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_COLLISION_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_COLLISION_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_EOC_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct INJ_EOC_MASKED_R(crate::FieldReader<bool, bool>);
impl INJ_EOC_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_EOC_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_EOC_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct INJ_SATURATE_MASKED_R(crate::FieldReader<bool, bool>);
impl INJ_SATURATE_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_SATURATE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_SATURATE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct INJ_RANGE_MASKED_R(crate::FieldReader<bool, bool>);
impl INJ_RANGE_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_RANGE_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_RANGE_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INJ_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub struct INJ_COLLISION_MASKED_R(crate::FieldReader<bool, bool>);
impl INJ_COLLISION_MASKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        INJ_COLLISION_MASKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INJ_COLLISION_MASKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn eos_masked(&self) -> EOS_MASKED_R {
        EOS_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow_masked(&self) -> OVERFLOW_MASKED_R {
        OVERFLOW_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fw_collision_masked(&self) -> FW_COLLISION_MASKED_R {
        FW_COLLISION_MASKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dsi_collision_masked(&self) -> DSI_COLLISION_MASKED_R {
        DSI_COLLISION_MASKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_eoc_masked(&self) -> INJ_EOC_MASKED_R {
        INJ_EOC_MASKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_saturate_masked(&self) -> INJ_SATURATE_MASKED_R {
        INJ_SATURATE_MASKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_range_masked(&self) -> INJ_RANGE_MASKED_R {
        INJ_RANGE_MASKED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_collision_masked(&self) -> INJ_COLLISION_MASKED_R {
        INJ_COLLISION_MASKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
