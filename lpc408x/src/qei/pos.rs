#[doc = "Register `POS` reader"]
pub struct R(crate::R<POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POS` reader - Current position value."]
pub struct POS_R(crate::FieldReader<u32, u32>);
impl POS_R {
    pub(crate) fn new(bits: u32) -> Self {
        POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Current position value."]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Position register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pos](index.html) module"]
pub struct POS_SPEC;
impl crate::RegisterSpec for POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pos::R](R) reader structure"]
impl crate::Readable for POS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for POS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
