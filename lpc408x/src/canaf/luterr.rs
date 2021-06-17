#[doc = "Register `LUTERR` reader"]
pub struct R(crate::R<LUTERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LUTERR` reader - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
pub struct LUTERR_R(crate::FieldReader<bool, bool>);
impl LUTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LUTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
    #[inline(always)]
    pub fn luterr(&self) -> LUTERR_R {
        LUTERR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "LUT Error Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [luterr](index.html) module"]
pub struct LUTERR_SPEC;
impl crate::RegisterSpec for LUTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [luterr::R](R) reader structure"]
impl crate::Readable for LUTERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LUTERR to value 0"]
impl crate::Resettable for LUTERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
