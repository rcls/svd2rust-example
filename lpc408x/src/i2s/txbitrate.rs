#[doc = "Register `TXBITRATE` reader"]
pub struct R(crate::R<TXBITRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBITRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBITRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBITRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBITRATE` writer"]
pub struct W(crate::W<TXBITRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBITRATE_SPEC>;
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
impl From<crate::W<TXBITRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBITRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BITRATE` reader - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub struct TX_BITRATE_R(crate::FieldReader<u8, u8>);
impl TX_BITRATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_BITRATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BITRATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BITRATE` writer - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub struct TX_BITRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&self) -> TX_BITRATE_R {
        TX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&mut self) -> TX_BITRATE_W {
        TX_BITRATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbitrate](index.html) module"]
pub struct TXBITRATE_SPEC;
impl crate::RegisterSpec for TXBITRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbitrate::R](R) reader structure"]
impl crate::Readable for TXBITRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbitrate::W](W) writer structure"]
impl crate::Writable for TXBITRATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBITRATE to value 0"]
impl crate::Resettable for TXBITRATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
