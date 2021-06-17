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
#[doc = "Field `RXSTATUS` reader - If 1, the receive channel is active. If 0, the receive channel is inactive."]
pub struct RXSTATUS_R(crate::FieldReader<bool, bool>);
impl RXSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTATUS` reader - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
pub struct TXSTATUS_R(crate::FieldReader<bool, bool>);
impl TXSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - If 1, the receive channel is active. If 0, the receive channel is inactive."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
