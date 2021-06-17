#[doc = "Register `MIC_OUT0` reader"]
pub struct R(crate::R<MIC_OUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIC_OUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIC_OUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIC_OUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIC_OUT` reader - This is the MIC generated during CCM encryption."]
pub struct MIC_OUT_R(crate::FieldReader<u32, u32>);
impl MIC_OUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        MIC_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIC_OUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This is the MIC generated during CCM encryption."]
    #[inline(always)]
    pub fn mic_out(&self) -> MIC_OUT_R {
        MIC_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "MIC output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mic_out0](index.html) module"]
pub struct MIC_OUT0_SPEC;
impl crate::RegisterSpec for MIC_OUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mic_out0::R](R) reader structure"]
impl crate::Readable for MIC_OUT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIC_OUT0 to value 0"]
impl crate::Resettable for MIC_OUT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
