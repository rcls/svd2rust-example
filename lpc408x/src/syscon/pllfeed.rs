#[doc = "Register `PLLFEED%s` writer"]
pub struct W(crate::W<PLLFEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLFEED_SPEC>;
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
impl From<crate::W<PLLFEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLFEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLFEED` writer - The PLL feed sequence must be written to this register in order for the related PLL's configuration and control register changes to take effect."]
pub struct PLLFEED_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - The PLL feed sequence must be written to this register in order for the related PLL's configuration and control register changes to take effect."]
    #[inline(always)]
    pub fn pllfeed(&mut self) -> PLLFEED_W {
        PLLFEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Feed register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfeed](index.html) module"]
pub struct PLLFEED_SPEC;
impl crate::RegisterSpec for PLLFEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pllfeed::W](W) writer structure"]
impl crate::Writable for PLLFEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLFEED%s to value 0"]
impl crate::Resettable for PLLFEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
