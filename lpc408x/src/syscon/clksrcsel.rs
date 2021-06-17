#[doc = "Register `CLKSRCSEL` reader"]
pub struct R(crate::R<CLKSRCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSRCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSRCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSRCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSRCSEL` writer"]
pub struct W(crate::W<CLKSRCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSRCSEL_SPEC>;
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
impl From<crate::W<CLKSRCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSRCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the clock source for sysclk and PLL0 as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL = 0,
    #[doc = "1: Selects the main oscillator as the sysclk and PLL0 clock source."]
    SELECTS_THE_MAIN_OSC = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Selects the clock source for sysclk and PLL0 as follows:"]
pub struct CLKSRC_R(crate::FieldReader<bool, CLKSRC_A>);
impl CLKSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::SELECTS_THE_INTERNAL,
            true => CLKSRC_A::SELECTS_THE_MAIN_OSC,
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        **self == CLKSRC_A::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        **self == CLKSRC_A::SELECTS_THE_MAIN_OSC
    }
}
impl core::ops::Deref for CLKSRC_R {
    type Target = crate::FieldReader<bool, CLKSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSRC` writer - Selects the clock source for sysclk and PLL0 as follows:"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_MAIN_OSC)
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
impl R {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksrcsel](index.html) module"]
pub struct CLKSRCSEL_SPEC;
impl crate::RegisterSpec for CLKSRCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksrcsel::R](R) reader structure"]
impl crate::Readable for CLKSRCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksrcsel::W](W) writer structure"]
impl crate::Writable for CLKSRCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSRCSEL to value 0"]
impl crate::Resettable for CLKSRCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
