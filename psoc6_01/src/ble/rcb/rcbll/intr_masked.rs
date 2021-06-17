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
#[doc = "Field `RCB_LL_DONE` reader - Logical and of corresponding request and mask bits."]
pub struct RCB_LL_DONE_R(crate::FieldReader<bool, bool>);
impl RCB_LL_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB_LL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB_LL_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_WRITE_DONE` reader - N/A"]
pub struct SINGLE_WRITE_DONE_R(crate::FieldReader<bool, bool>);
impl SINGLE_WRITE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_WRITE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_WRITE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_READ_DONE` reader - N/A"]
pub struct SINGLE_READ_DONE_R(crate::FieldReader<bool, bool>);
impl SINGLE_READ_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_READ_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_READ_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rcb_ll_done(&self) -> RCB_LL_DONE_R {
        RCB_LL_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&self) -> SINGLE_WRITE_DONE_R {
        SINGLE_WRITE_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&self) -> SINGLE_READ_DONE_R {
        SINGLE_READ_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Master interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
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
