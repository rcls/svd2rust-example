#[doc = "Register `CTIME1` reader"]
pub struct R(crate::R<CTIME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub struct DOM_R(crate::FieldReader<u8, u8>);
impl DOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub struct YEAR_R(crate::FieldReader<u16, u16>);
impl YEAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "Consolidated Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime1](index.html) module"]
pub struct CTIME1_SPEC;
impl crate::RegisterSpec for CTIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctime1::R](R) reader structure"]
impl crate::Readable for CTIME1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTIME1 to value 0"]
impl crate::Resettable for CTIME1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
