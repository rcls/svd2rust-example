#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WIN_ACTIVE` reader - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
pub struct WIN_ACTIVE_R(crate::FieldReader<bool, bool>);
impl WIN_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIN_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIN_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub fn win_active(&self) -> WIN_ACTIVE_R {
        WIN_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Profile status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
