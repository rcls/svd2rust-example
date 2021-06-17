#[doc = "Register `DMA2` reader"]
pub struct R(crate::R<DMA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2` writer"]
pub struct W(crate::W<DMA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2_SPEC>;
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
impl From<crate::W<DMA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub struct RX_DMA2_ENABLE_R(crate::FieldReader<bool, bool>);
impl RX_DMA2_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA2_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DMA2_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub struct RX_DMA2_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA2_ENABLE_W<'a> {
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
#[doc = "Field `TX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub struct TX_DMA2_ENABLE_R(crate::FieldReader<bool, bool>);
impl TX_DMA2_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA2_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DMA2_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub struct TX_DMA2_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA2_ENABLE_W<'a> {
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
#[doc = "Field `RX_DEPTH_DMA2` reader - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub struct RX_DEPTH_DMA2_R(crate::FieldReader<u8, u8>);
impl RX_DEPTH_DMA2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_DEPTH_DMA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DEPTH_DMA2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DEPTH_DMA2` writer - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub struct RX_DEPTH_DMA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DEPTH_DMA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TX_DEPTH_DMA2` reader - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub struct TX_DEPTH_DMA2_R(crate::FieldReader<u8, u8>);
impl TX_DEPTH_DMA2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DEPTH_DMA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DEPTH_DMA2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DEPTH_DMA2` writer - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub struct TX_DEPTH_DMA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DEPTH_DMA2_W<'a> {
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
    pub fn rx_dma2_enable(&self) -> RX_DMA2_ENABLE_R {
        RX_DMA2_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma2_enable(&self) -> TX_DMA2_ENABLE_R {
        TX_DMA2_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    pub fn rx_depth_dma2(&self) -> RX_DEPTH_DMA2_R {
        RX_DEPTH_DMA2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    pub fn tx_depth_dma2(&self) -> TX_DEPTH_DMA2_R {
        TX_DEPTH_DMA2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma2_enable(&mut self) -> RX_DMA2_ENABLE_W {
        RX_DMA2_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma2_enable(&mut self) -> TX_DMA2_ENABLE_W {
        TX_DMA2_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    pub fn rx_depth_dma2(&mut self) -> RX_DEPTH_DMA2_W {
        RX_DEPTH_DMA2_W { w: self }
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    pub fn tx_depth_dma2(&mut self) -> TX_DEPTH_DMA2_W {
        TX_DEPTH_DMA2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2](index.html) module"]
pub struct DMA2_SPEC;
impl crate::RegisterSpec for DMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2::R](R) reader structure"]
impl crate::Readable for DMA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2::W](W) writer structure"]
impl crate::Writable for DMA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2 to value 0"]
impl crate::Resettable for DMA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
