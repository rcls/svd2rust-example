#[doc = "Register `TX_RX_SYNTH_DELAY` reader"]
pub struct R(crate::R<TX_RX_SYNTH_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RX_SYNTH_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RX_SYNTH_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RX_SYNTH_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_RX_SYNTH_DELAY` writer"]
pub struct W(crate::W<TX_RX_SYNTH_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_RX_SYNTH_DELAY_SPEC>;
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
impl From<crate::W<TX_RX_SYNTH_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_RX_SYNTH_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN_DELAY` reader - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
pub struct RX_EN_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_EN_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_EN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN_DELAY` writer - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
pub struct RX_EN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TX_EN_DELAY` reader - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
pub struct TX_EN_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_EN_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_EN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_EN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_EN_DELAY` writer - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
pub struct TX_EN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&self) -> RX_EN_DELAY_R {
        RX_EN_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&self) -> TX_EN_DELAY_R {
        TX_EN_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&mut self) -> RX_EN_DELAY_W {
        RX_EN_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&mut self) -> TX_EN_DELAY_W {
        TX_EN_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit/Receive enable delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_synth_delay](index.html) module"]
pub struct TX_RX_SYNTH_DELAY_SPEC;
impl crate::RegisterSpec for TX_RX_SYNTH_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rx_synth_delay::R](R) reader structure"]
impl crate::Readable for TX_RX_SYNTH_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_rx_synth_delay::W](W) writer structure"]
impl crate::Writable for TX_RX_SYNTH_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_RX_SYNTH_DELAY to value 0"]
impl crate::Resettable for TX_RX_SYNTH_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
