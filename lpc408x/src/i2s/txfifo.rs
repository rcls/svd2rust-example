#[doc = "Register `TXFIFO` writer"]
pub struct W(crate::W<TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFO_SPEC>;
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
impl From<crate::W<TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2STXFIFO` writer - 8 x 32-bit transmit FIFO."]
pub struct I2STXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2STXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2stxfifo(&mut self) -> I2STXFIFO_W {
        I2STXFIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](index.html) module"]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txfifo::W](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
