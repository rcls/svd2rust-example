#[doc = "Register `DATACNT` reader"]
pub struct R(crate::R<DATACNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATACNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATACNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATACNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATACOUNT` reader - Remaining data"]
pub struct DATACOUNT_R(crate::FieldReader<u16, u16>);
impl DATACOUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATACOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATACOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Remaining data"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Data counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datacnt](index.html) module"]
pub struct DATACNT_SPEC;
impl crate::RegisterSpec for DATACNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datacnt::R](R) reader structure"]
impl crate::Readable for DATACNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATACNT to value 0"]
impl crate::Resettable for DATACNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
