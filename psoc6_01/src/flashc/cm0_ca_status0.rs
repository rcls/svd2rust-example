#[doc = "Register `CM0_CA_STATUS0` reader"]
pub struct R(crate::R<CM0_CA_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CA_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CA_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CA_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID16` reader - Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
pub struct VALID16_R(crate::FieldReader<u16, u16>);
impl VALID16_R {
    pub(crate) fn new(bits: u16) -> Self {
        VALID16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn valid16(&self) -> VALID16_R {
        VALID16_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CM0+ cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status0](index.html) module"]
pub struct CM0_CA_STATUS0_SPEC;
impl crate::RegisterSpec for CM0_CA_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_ca_status0::R](R) reader structure"]
impl crate::Readable for CM0_CA_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM0_CA_STATUS0 to value 0"]
impl crate::Resettable for CM0_CA_STATUS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
