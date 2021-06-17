#[doc = "Register `CTIME0` reader"]
pub struct R(crate::R<CTIME0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIME0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIME0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIME0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECONDS` reader - Seconds value in the range of 0 to 59"]
pub struct SECONDS_R(crate::FieldReader<u8, u8>);
impl SECONDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECONDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECONDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub struct MINUTES_R(crate::FieldReader<u8, u8>);
impl MINUTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINUTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub struct HOURS_R(crate::FieldReader<u8, u8>);
impl HOURS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOURS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOURS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6"]
pub struct DOW_R(crate::FieldReader<u8, u8>);
impl DOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of week value in the range of 0 to 6"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
#[doc = "Consolidated Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime0](index.html) module"]
pub struct CTIME0_SPEC;
impl crate::RegisterSpec for CTIME0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctime0::R](R) reader structure"]
impl crate::Readable for CTIME0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTIME0 to value 0"]
impl crate::Resettable for CTIME0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
