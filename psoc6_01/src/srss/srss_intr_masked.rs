#[doc = "Register `SRSS_INTR_MASKED` reader"]
pub struct R(crate::R<SRSS_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDT_MATCH` reader - Logical and of corresponding request and mask bits."]
pub struct WDT_MATCH_R(crate::FieldReader<bool, bool>);
impl WDT_MATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLVD1` reader - Logical and of corresponding request and mask bits."]
pub struct HVLVD1_R(crate::FieldReader<bool, bool>);
impl HVLVD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLVD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLVD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_CAL` reader - Logical and of corresponding request and mask bits."]
pub struct CLK_CAL_R(crate::FieldReader<bool, bool>);
impl CLK_CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "SRSS Interrupt Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_masked](index.html) module"]
pub struct SRSS_INTR_MASKED_SPEC;
impl crate::RegisterSpec for SRSS_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_intr_masked::R](R) reader structure"]
impl crate::Readable for SRSS_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRSS_INTR_MASKED to value 0"]
impl crate::Resettable for SRSS_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
