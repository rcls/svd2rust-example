#[doc = "Register `CAP` reader"]
pub struct R(crate::R<CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VELCAP` reader - Last velocity capture."]
pub struct VELCAP_R(crate::FieldReader<u32, u32>);
impl VELCAP_R {
    pub(crate) fn new(bits: u32) -> Self {
        VELCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VELCAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Last velocity capture."]
    #[inline(always)]
    pub fn velcap(&self) -> VELCAP_R {
        VELCAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Velocity capture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](index.html) module"]
pub struct CAP_SPEC;
impl crate::RegisterSpec for CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap::R](R) reader structure"]
impl crate::Readable for CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP to value 0xffff_ffff"]
impl crate::Resettable for CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
