#[doc = "Register `VEL` reader"]
pub struct R(crate::R<VEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VELPC` reader - Current velocity pulse count."]
pub struct VELPC_R(crate::FieldReader<u32, u32>);
impl VELPC_R {
    pub(crate) fn new(bits: u32) -> Self {
        VELPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VELPC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VELPC_R {
        VELPC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Velocity counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vel](index.html) module"]
pub struct VEL_SPEC;
impl crate::RegisterSpec for VEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vel::R](R) reader structure"]
impl crate::Readable for VEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VEL to value 0"]
impl crate::Resettable for VEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
