#[doc = "Register `EPIN` writer"]
pub struct W(crate::W<EPIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPIN_SPEC>;
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
impl From<crate::W<EPIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_EP` writer - Physical endpoint number (0-31)"]
pub struct PHY_EP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:4 - Physical endpoint number (0-31)"]
    #[inline(always)]
    pub fn phy_ep(&mut self) -> PHY_EP_W {
        PHY_EP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Index\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epin](index.html) module"]
pub struct EPIN_SPEC;
impl crate::RegisterSpec for EPIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [epin::W](W) writer structure"]
impl crate::Writable for EPIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPIN to value 0"]
impl crate::Resettable for EPIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
