#[doc = "Register `INTR_M` reader"]
pub struct R(crate::R<INTR_M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_M` writer"]
pub struct W(crate::W<INTR_M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTR_M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub struct I2C_ARB_LOST_R(crate::FieldReader<bool, bool>);
impl I2C_ARB_LOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ARB_LOST` writer - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub struct I2C_ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARB_LOST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `I2C_NACK` reader - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub struct I2C_NACK_R(crate::FieldReader<bool, bool>);
impl I2C_NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_NACK` writer - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub struct I2C_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `I2C_ACK` reader - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub struct I2C_ACK_R(crate::FieldReader<bool, bool>);
impl I2C_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ACK` writer - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub struct I2C_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `I2C_STOP` reader - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub struct I2C_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_STOP` writer - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub struct I2C_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_STOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `I2C_BUS_ERROR` reader - I2C master bus error (unexpected detection of START or STOP condition)."]
pub struct I2C_BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl I2C_BUS_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_BUS_ERROR` writer - I2C master bus error (unexpected detection of START or STOP condition)."]
pub struct I2C_BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_BUS_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SPI_DONE` reader - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
pub struct SPI_DONE_R(crate::FieldReader<bool, bool>);
impl SPI_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DONE` writer - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
pub struct SPI_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
    #[inline(always)]
    pub fn spi_done(&self) -> SPI_DONE_R {
        SPI_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W { w: self }
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W { w: self }
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W { w: self }
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W { w: self }
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W {
        I2C_BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
    #[inline(always)]
    pub fn spi_done(&mut self) -> SPI_DONE_W {
        SPI_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_m](index.html) module"]
pub struct INTR_M_SPEC;
impl crate::RegisterSpec for INTR_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_m::R](R) reader structure"]
impl crate::Readable for INTR_M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_m::W](W) writer structure"]
impl crate::Writable for INTR_M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_M to value 0"]
impl crate::Resettable for INTR_M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
