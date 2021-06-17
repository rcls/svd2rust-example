#[doc = "Register `RXBITRATE` reader"]
pub struct R(crate::R<RXBITRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBITRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBITRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBITRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBITRATE` writer"]
pub struct W(crate::W<RXBITRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBITRATE_SPEC>;
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
impl From<crate::W<RXBITRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBITRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_BITRATE` reader - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub struct RX_BITRATE_R(crate::FieldReader<u8, u8>);
impl RX_BITRATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_BITRATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BITRATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BITRATE` writer - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub struct RX_BITRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BITRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&self) -> RX_BITRATE_R {
        RX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&mut self) -> RX_BITRATE_W {
        RX_BITRATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbitrate](index.html) module"]
pub struct RXBITRATE_SPEC;
impl crate::RegisterSpec for RXBITRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbitrate::R](R) reader structure"]
impl crate::Readable for RXBITRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbitrate::W](W) writer structure"]
impl crate::Writable for RXBITRATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBITRATE to value 0"]
impl crate::Resettable for RXBITRATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
