#[doc = "Register `CM4_CA_STATUS1` reader"]
pub struct R(crate::R<CM4_CA_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_CA_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_CA_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_CA_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAG` reader - See CM0_CA_STATUS1."]
pub struct TAG_R(crate::FieldReader<u32, u32>);
impl TAG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS1."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "CM4 cache status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status1](index.html) module"]
pub struct CM4_CA_STATUS1_SPEC;
impl crate::RegisterSpec for CM4_CA_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_ca_status1::R](R) reader structure"]
impl crate::Readable for CM4_CA_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_CA_STATUS1 to value 0"]
impl crate::Resettable for CM4_CA_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
