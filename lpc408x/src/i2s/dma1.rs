#[doc = "Register `DMA1` reader"]
pub struct R(crate::R<DMA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1` writer"]
pub struct W(crate::W<DMA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_SPEC>;
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
impl From<crate::W<DMA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub struct RX_DMA1_ENABLE_R(crate::FieldReader<bool, bool>);
impl RX_DMA1_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DMA1_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub struct RX_DMA1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA1_ENABLE_W<'a> {
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
#[doc = "Field `TX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub struct TX_DMA1_ENABLE_R(crate::FieldReader<bool, bool>);
impl TX_DMA1_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DMA1_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub struct TX_DMA1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA1_ENABLE_W<'a> {
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
#[doc = "Field `RX_DEPTH_DMA1` reader - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub struct RX_DEPTH_DMA1_R(crate::FieldReader<u8, u8>);
impl RX_DEPTH_DMA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_DEPTH_DMA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DEPTH_DMA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DEPTH_DMA1` writer - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub struct RX_DEPTH_DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DEPTH_DMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TX_DEPTH_DMA1` reader - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub struct TX_DEPTH_DMA1_R(crate::FieldReader<u8, u8>);
impl TX_DEPTH_DMA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DEPTH_DMA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DEPTH_DMA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DEPTH_DMA1` writer - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub struct TX_DEPTH_DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DEPTH_DMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&self) -> RX_DMA1_ENABLE_R {
        RX_DMA1_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&self) -> TX_DMA1_ENABLE_R {
        TX_DMA1_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&self) -> RX_DEPTH_DMA1_R {
        RX_DEPTH_DMA1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&self) -> TX_DEPTH_DMA1_R {
        TX_DEPTH_DMA1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&mut self) -> RX_DMA1_ENABLE_W {
        RX_DMA1_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&mut self) -> TX_DMA1_ENABLE_W {
        TX_DMA1_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&mut self) -> RX_DEPTH_DMA1_W {
        RX_DEPTH_DMA1_W { w: self }
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&mut self) -> TX_DEPTH_DMA1_W {
        TX_DEPTH_DMA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1](index.html) module"]
pub struct DMA1_SPEC;
impl crate::RegisterSpec for DMA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1::R](R) reader structure"]
impl crate::Readable for DMA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1::W](W) writer structure"]
impl crate::Writable for DMA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1 to value 0"]
impl crate::Resettable for DMA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
