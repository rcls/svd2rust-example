#[doc = "Register `CTIME2` reader"]
pub struct R(crate::R<CTIME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub struct DOY_R(crate::FieldReader<u16, u16>);
impl DOY_R {
    pub(crate) fn new(bits: u16) -> Self {
        DOY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Consolidated Time Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime2](index.html) module"]
pub struct CTIME2_SPEC;
impl crate::RegisterSpec for CTIME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctime2::R](R) reader structure"]
impl crate::Readable for CTIME2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTIME2 to value 0"]
impl crate::Resettable for CTIME2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
