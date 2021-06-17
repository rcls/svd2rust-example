#[doc = "Register `CCLKSEL` reader"]
pub struct R(crate::R<CCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCLKSEL` writer"]
pub struct W(crate::W<CCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCLKSEL_SPEC>;
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
impl From<crate::W<CCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLKDIV` reader - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
pub struct CCLKDIV_R(crate::FieldReader<u8, u8>);
impl CCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLKDIV` writer - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
pub struct CCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the CPU clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLKSEL_A {
    #[doc = "0: Sysclk is used as the input to the CPU clock divider."]
    SYSCLK_IS_USED_AS_TH = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the CPU clock divider."]
    THE_OUTPUT_OF_THE_MA = 1,
}
impl From<CCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLKSEL` reader - Selects the input clock for the CPU clock divider."]
pub struct CCLKSEL_R(crate::FieldReader<bool, CCLKSEL_A>);
impl CCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLKSEL_A {
        match self.bits {
            false => CCLKSEL_A::SYSCLK_IS_USED_AS_TH,
            true => CCLKSEL_A::THE_OUTPUT_OF_THE_MA,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_IS_USED_AS_TH`"]
    #[inline(always)]
    pub fn is_sysclk_is_used_as_th(&self) -> bool {
        **self == CCLKSEL_A::SYSCLK_IS_USED_AS_TH
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_MA`"]
    #[inline(always)]
    pub fn is_the_output_of_the_ma(&self) -> bool {
        **self == CCLKSEL_A::THE_OUTPUT_OF_THE_MA
    }
}
impl core::ops::Deref for CCLKSEL_R {
    type Target = crate::FieldReader<bool, CCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLKSEL` writer - Selects the input clock for the CPU clock divider."]
pub struct CCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn sysclk_is_used_as_th(self) -> &'a mut W {
        self.variant(CCLKSEL_A::SYSCLK_IS_USED_AS_TH)
    }
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_ma(self) -> &'a mut W {
        self.variant(CCLKSEL_A::THE_OUTPUT_OF_THE_MA)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&self) -> CCLKDIV_R {
        CCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&self) -> CCLKSEL_R {
        CCLKSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&mut self) -> CCLKDIV_W {
        CCLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&mut self) -> CCLKSEL_W {
        CCLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cclksel](index.html) module"]
pub struct CCLKSEL_SPEC;
impl crate::RegisterSpec for CCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cclksel::R](R) reader structure"]
impl crate::Readable for CCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cclksel::W](W) writer structure"]
impl crate::Writable for CCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCLKSEL to value 0"]
impl crate::Resettable for CCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
