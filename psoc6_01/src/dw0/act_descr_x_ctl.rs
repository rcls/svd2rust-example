#[doc = "Register `ACT_DESCR_X_CTL` reader"]
pub struct R(crate::R<ACT_DESCR_X_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_DESCR_X_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_DESCR_X_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_DESCR_X_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Copy of DESCR_X_CTL of the currently active descriptor."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_X_CTL of the currently active descriptor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Active descriptor X loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_x_ctl](index.html) module"]
pub struct ACT_DESCR_X_CTL_SPEC;
impl crate::RegisterSpec for ACT_DESCR_X_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_descr_x_ctl::R](R) reader structure"]
impl crate::Readable for ACT_DESCR_X_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_DESCR_X_CTL to value 0"]
impl crate::Resettable for ACT_DESCR_X_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
