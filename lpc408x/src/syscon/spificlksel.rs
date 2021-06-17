#[doc = "Register `SPIFICLKSEL` reader"]
pub struct R(crate::R<SPIFICLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIFICLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIFICLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIFICLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIFICLKSEL` writer"]
pub struct W(crate::W<SPIFICLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIFICLKSEL_SPEC>;
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
impl From<crate::W<SPIFICLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIFICLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIFIDIV` reader - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
pub struct SPIFIDIV_R(crate::FieldReader<u8, u8>);
impl SPIFIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPIFIDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIFIDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIFIDIV` writer - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
pub struct SPIFIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the USB clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPIFISEL_A {
    #[doc = "0: Sysclk is used as the input to the SPIFI clock divider."]
    SYSCLK = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the SPIFI clock divider."]
    MAINPLLOUT = 1,
    #[doc = "2: The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    ALTPLLOUT = 2,
}
impl From<SPIFISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIFISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPIFISEL` reader - Selects the input clock for the USB clock divider."]
pub struct SPIFISEL_R(crate::FieldReader<u8, SPIFISEL_A>);
impl SPIFISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPIFISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIFISEL_A {
        match self.bits {
            0 => SPIFISEL_A::SYSCLK,
            1 => SPIFISEL_A::MAINPLLOUT,
            2 => SPIFISEL_A::ALTPLLOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == SPIFISEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `MAINPLLOUT`"]
    #[inline(always)]
    pub fn is_mainpllout(&self) -> bool {
        **self == SPIFISEL_A::MAINPLLOUT
    }
    #[doc = "Checks if the value of the field is `ALTPLLOUT`"]
    #[inline(always)]
    pub fn is_altpllout(&self) -> bool {
        **self == SPIFISEL_A::ALTPLLOUT
    }
}
impl core::ops::Deref for SPIFISEL_R {
    type Target = crate::FieldReader<u8, SPIFISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIFISEL` writer - Selects the input clock for the USB clock divider."]
pub struct SPIFISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIFISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sysclk is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SPIFISEL_A::SYSCLK)
    }
    #[doc = "The output of the Main PLL is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn mainpllout(self) -> &'a mut W {
        self.variant(SPIFISEL_A::MAINPLLOUT)
    }
    #[doc = "The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn altpllout(self) -> &'a mut W {
        self.variant(SPIFISEL_A::ALTPLLOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline(always)]
    pub fn spifidiv(&self) -> SPIFIDIV_R {
        SPIFIDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn spifisel(&self) -> SPIFISEL_R {
        SPIFISEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline(always)]
    pub fn spifidiv(&mut self) -> SPIFIDIV_W {
        SPIFIDIV_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn spifisel(&mut self) -> SPIFISEL_W {
        SPIFISEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spificlksel](index.html) module"]
pub struct SPIFICLKSEL_SPEC;
impl crate::RegisterSpec for SPIFICLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spificlksel::R](R) reader structure"]
impl crate::Readable for SPIFICLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spificlksel::W](W) writer structure"]
impl crate::Writable for SPIFICLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIFICLKSEL to value 0"]
impl crate::Resettable for SPIFICLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
