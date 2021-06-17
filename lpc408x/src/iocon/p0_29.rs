#[doc = "Register `P0_29` reader"]
pub struct R(crate::R<P0_29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P0_29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P0_29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P0_29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P0_29` writer"]
pub struct W(crate::W<P0_29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P0_29_SPEC>;
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
impl From<crate::W<P0_29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P0_29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function for pin P0\\[29\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin."]
    P0_29 = 0,
    #[doc = "1: USB port 1 bidirectional D+ line."]
    USB_DP1 = 1,
    #[doc = "2: External interrupt 0 input."]
    EINT0 = 2,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[29\\]"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::P0_29),
            1 => Some(FUNC_A::USB_DP1),
            2 => Some(FUNC_A::EINT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P0_29`"]
    #[inline(always)]
    pub fn is_p0_29(&self) -> bool {
        **self == FUNC_A::P0_29
    }
    #[doc = "Checks if the value of the field is `USB_DP1`"]
    #[inline(always)]
    pub fn is_usb_dp1(&self) -> bool {
        **self == FUNC_A::USB_DP1
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        **self == FUNC_A::EINT0
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[29\\]"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_29(self) -> &'a mut W {
        self.variant(FUNC_A::P0_29)
    }
    #[doc = "USB port 1 bidirectional D+ line."]
    #[inline(always)]
    pub fn usb_dp1(self) -> &'a mut W {
        self.variant(FUNC_A::USB_DP1)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut W {
        self.variant(FUNC_A::EINT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration register for pin P0\\[29\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p0_29](index.html) module"]
pub struct P0_29_SPEC;
impl crate::RegisterSpec for P0_29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p0_29::R](R) reader structure"]
impl crate::Readable for P0_29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p0_29::W](W) writer structure"]
impl crate::Writable for P0_29_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P0_29 to value 0"]
impl crate::Resettable for P0_29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
