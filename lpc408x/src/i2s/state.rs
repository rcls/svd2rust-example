#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQ` reader - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
pub struct IRQ_R(crate::FieldReader<bool, bool>);
impl IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREQ1` reader - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
pub struct DMAREQ1_R(crate::FieldReader<bool, bool>);
impl DMAREQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREQ2` reader - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
pub struct DMAREQ2_R(crate::FieldReader<bool, bool>);
impl DMAREQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_LEVEL` reader - Reflects the current level of the Receive FIFO."]
pub struct RX_LEVEL_R(crate::FieldReader<u8, u8>);
impl RX_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LEVEL` reader - Reflects the current level of the Transmit FIFO."]
pub struct TX_LEVEL_R(crate::FieldReader<u8, u8>);
impl TX_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
    #[inline(always)]
    pub fn dmareq1(&self) -> DMAREQ1_R {
        DMAREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
    #[inline(always)]
    pub fn dmareq2(&self) -> DMAREQ2_R {
        DMAREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Reflects the current level of the Receive FIFO."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
