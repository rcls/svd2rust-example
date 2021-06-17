#[doc = "Register `RXDATA` reader"]
pub struct R(crate::R<RXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_DATA` reader - Data received."]
pub struct RX_DATA_R(crate::FieldReader<u32, u32>);
impl RX_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data received."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "USB Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](index.html) module"]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdata::R](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
