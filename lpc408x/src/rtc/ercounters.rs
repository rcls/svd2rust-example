#[doc = "Register `ERCOUNTERS` reader"]
pub struct R(crate::R<ERCOUNTERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERCOUNTERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERCOUNTERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERCOUNTERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER0` reader - Value of the counter for event 0. If the counter reaches full count (the value 7), it remains there if additional events occur. This counter is cleared when the corresponding EVx bit in the ERSTATUS register is cleared by software."]
pub struct COUNTER0_R(crate::FieldReader<u8, u8>);
impl COUNTER0_R {
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER1` reader - Value of the counter for event 1. See description for COUNTER0."]
pub struct COUNTER1_R(crate::FieldReader<u8, u8>);
impl COUNTER1_R {
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER2` reader - Value of the counter for event 2. See description for COUNTER0."]
pub struct COUNTER2_R(crate::FieldReader<u8, u8>);
impl COUNTER2_R {
    pub(crate) fn new(bits: u8) -> Self {
        COUNTER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Value of the counter for event 0. If the counter reaches full count (the value 7), it remains there if additional events occur. This counter is cleared when the corresponding EVx bit in the ERSTATUS register is cleared by software."]
    #[inline(always)]
    pub fn counter0(&self) -> COUNTER0_R {
        COUNTER0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Value of the counter for event 1. See description for COUNTER0."]
    #[inline(always)]
    pub fn counter1(&self) -> COUNTER1_R {
        COUNTER1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Value of the counter for event 2. See description for COUNTER0."]
    #[inline(always)]
    pub fn counter2(&self) -> COUNTER2_R {
        COUNTER2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "Event Monitor/Recorder Counters register. Allows reading the counters associated with the event channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ercounters](index.html) module"]
pub struct ERCOUNTERS_SPEC;
impl crate::RegisterSpec for ERCOUNTERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ercounters::R](R) reader structure"]
impl crate::Readable for ERCOUNTERS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERCOUNTERS to value 0"]
impl crate::Resettable for ERCOUNTERS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
