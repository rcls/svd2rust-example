#[doc = "Register `CHAN_RESULT[%s]` reader"]
pub struct R(crate::R<CHAN_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_RESULT_NEWVALUE_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
pub struct CHAN_RESULT_NEWVALUE_MIR_R(crate::FieldReader<bool, bool>);
impl CHAN_RESULT_NEWVALUE_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHAN_RESULT_NEWVALUE_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_RESULT_NEWVALUE_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SATURATE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
pub struct SATURATE_INTR_MIR_R(crate::FieldReader<bool, bool>);
impl SATURATE_INTR_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SATURATE_INTR_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SATURATE_INTR_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
pub struct RANGE_INTR_MIR_R(crate::FieldReader<bool, bool>);
impl RANGE_INTR_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANGE_INTR_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANGE_INTR_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_RESULT_UPDATED_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
pub struct CHAN_RESULT_UPDATED_MIR_R(crate::FieldReader<bool, bool>);
impl CHAN_RESULT_UPDATED_MIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHAN_RESULT_UPDATED_MIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_RESULT_UPDATED_MIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_result_newvalue_mir(&self) -> CHAN_RESULT_NEWVALUE_MIR_R {
        CHAN_RESULT_NEWVALUE_MIR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn saturate_intr_mir(&self) -> SATURATE_INTR_MIR_R {
        SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RANGE_INTR_MIR_R {
        RANGE_INTR_MIR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub fn chan_result_updated_mir(&self) -> CHAN_RESULT_UPDATED_MIR_R {
        CHAN_RESULT_UPDATED_MIR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Channel result data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result](index.html) module"]
pub struct CHAN_RESULT_SPEC;
impl crate::RegisterSpec for CHAN_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_result::R](R) reader structure"]
impl crate::Readable for CHAN_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHAN_RESULT[%s]
to value 0"]
impl crate::Resettable for CHAN_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
