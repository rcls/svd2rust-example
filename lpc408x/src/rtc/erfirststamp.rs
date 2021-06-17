#[doc = "Register `ERFIRSTSTAMP%s` reader"]
pub struct R(crate::R<ERFIRSTSTAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERFIRSTSTAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERFIRSTSTAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERFIRSTSTAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC` reader - Seconds value in the range of 0 to 59."]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` reader - Minutes value in the range of 0 to 59."]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOUR` reader - Hours value in the range of 0 to 23."]
pub struct HOUR_R(crate::FieldReader<u8, u8>);
impl HOUR_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOY` reader - Day of Year value in the range of 1 to 366."]
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
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minutes value in the range of 0 to 59."]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hours value in the range of 0 to 23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:25 - Day of Year value in the range of 1 to 366."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
}
#[doc = "Event Monitor/Recorder First Stamp register for channel 0. Retains the time stamp for the first event on channel 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erfirststamp](index.html) module"]
pub struct ERFIRSTSTAMP_SPEC;
impl crate::RegisterSpec for ERFIRSTSTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erfirststamp::R](R) reader structure"]
impl crate::Readable for ERFIRSTSTAMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERFIRSTSTAMP%s to value 0"]
impl crate::Resettable for ERFIRSTSTAMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
