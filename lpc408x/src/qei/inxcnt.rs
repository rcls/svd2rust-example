#[doc = "Register `INXCNT` reader"]
pub struct R(crate::R<INXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENCPOS` reader - Current index counter value."]
pub struct ENCPOS_R(crate::FieldReader<u32, u32>);
impl ENCPOS_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENCPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCPOS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current index counter value."]
    #[inline(always)]
    pub fn encpos(&self) -> ENCPOS_R {
        ENCPOS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Index count register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inxcnt](index.html) module"]
pub struct INXCNT_SPEC;
impl crate::RegisterSpec for INXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inxcnt::R](R) reader structure"]
impl crate::Readable for INXCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INXCNT to value 0"]
impl crate::Resettable for INXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
