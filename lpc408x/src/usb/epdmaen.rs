#[doc = "Register `EPDMAEN` writer"]
pub struct W(crate::W<EPDMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPDMAEN_SPEC>;
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
impl From<crate::W<EPDMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPDMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_DMA_EN0` writer - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
pub struct EP_DMA_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN0_W<'a> {
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
#[doc = "Field `EP_DMA_EN1` writer - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
pub struct EP_DMA_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN1_W<'a> {
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
#[doc = "Field `EP_DMA_EN` writer - Endpoint xx(2 <= xx <= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
pub struct EP_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_en0(&mut self) -> EP_DMA_EN0_W {
        EP_DMA_EN0_W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_en1(&mut self) -> EP_DMA_EN1_W {
        EP_DMA_EN1_W { w: self }
    }
    #[doc = "Bits 2:31 - Endpoint xx(2 <= xx <= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_en(&mut self) -> EP_DMA_EN_W {
        EP_DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint DMA Enable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdmaen](index.html) module"]
pub struct EPDMAEN_SPEC;
impl crate::RegisterSpec for EPDMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [epdmaen::W](W) writer structure"]
impl crate::Writable for EPDMAEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPDMAEN to value 0"]
impl crate::Resettable for EPDMAEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
