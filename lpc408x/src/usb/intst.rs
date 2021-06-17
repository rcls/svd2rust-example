#[doc = "Register `INTST` reader"]
pub struct R(crate::R<INTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TMR` reader - Timer time-out."]
pub struct TMR_R(crate::FieldReader<bool, bool>);
impl TMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMOVE_PU` reader - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
pub struct REMOVE_PU_R(crate::FieldReader<bool, bool>);
impl REMOVE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REMOVE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMOVE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNP_FAILURE` reader - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
pub struct HNP_FAILURE_R(crate::FieldReader<bool, bool>);
impl HNP_FAILURE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNP_FAILURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNP_FAILURE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNP_SUCCESS` reader - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
pub struct HNP_SUCCESS_R(crate::FieldReader<bool, bool>);
impl HNP_SUCCESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNP_SUCCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNP_SUCCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Timer time-out."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
    #[inline(always)]
    pub fn remove_pu(&self) -> REMOVE_PU_R {
        REMOVE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
    #[inline(always)]
    pub fn hnp_failure(&self) -> HNP_FAILURE_R {
        HNP_FAILURE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
    #[inline(always)]
    pub fn hnp_success(&self) -> HNP_SUCCESS_R {
        HNP_SUCCESS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "OTG Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intst](index.html) module"]
pub struct INTST_SPEC;
impl crate::RegisterSpec for INTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intst::R](R) reader structure"]
impl crate::Readable for INTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTST to value 0"]
impl crate::Resettable for INTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
