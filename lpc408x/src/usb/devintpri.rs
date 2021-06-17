#[doc = "Register `DEVINTPRI` writer"]
pub struct W(crate::W<DEVINTPRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINTPRI_SPEC>;
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
impl From<crate::W<DEVINTPRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINTPRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frame interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_AW {
    #[doc = "0: FRAME interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: FRAME interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - Frame interrupt routing"]
pub struct FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(FRAME_AW::LP)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(FRAME_AW::HP)
    }
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
#[doc = "Fast endpoint interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_FAST_AW {
    #[doc = "0: EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<EP_FAST_AW> for bool {
    #[inline(always)]
    fn from(variant: EP_FAST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP_FAST` writer - Fast endpoint interrupt routing"]
pub struct EP_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_FAST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(EP_FAST_AW::LP)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(EP_FAST_AW::HP)
    }
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
impl W {
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W {
        FRAME_W { w: self }
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline(always)]
    pub fn ep_fast(&mut self) -> EP_FAST_W {
        EP_FAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Priority\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintpri](index.html) module"]
pub struct DEVINTPRI_SPEC;
impl crate::RegisterSpec for DEVINTPRI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [devintpri::W](W) writer structure"]
impl crate::Writable for DEVINTPRI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINTPRI to value 0"]
impl crate::Resettable for DEVINTPRI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
