#[doc = "Register `I2C_RX` reader"]
pub struct R(crate::R<I2C_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_DATA` reader - Receive data."]
pub struct RX_DATA_R(crate::FieldReader<u8, u8>);
impl RX_DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive data."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Receive\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_rx](index.html) module"]
pub struct I2C_RX_SPEC;
impl crate::RegisterSpec for I2C_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_rx::R](R) reader structure"]
impl crate::Readable for I2C_RX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_RX to value 0"]
impl crate::Resettable for I2C_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
