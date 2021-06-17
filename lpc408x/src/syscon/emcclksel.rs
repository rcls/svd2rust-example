#[doc = "Register `EMCCLKSEL` reader"]
pub struct R(crate::R<EMCCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCCLKSEL` writer"]
pub struct W(crate::W<EMCCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCCLKSEL_SPEC>;
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
impl From<crate::W<EMCCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the EMC clock rate relative to the CPU clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCDIV_A {
    #[doc = "0: The EMC uses the same clock as the CPU."]
    THE_EMC_USES_THE_SAM = 0,
    #[doc = "1: The EMC uses a clock at half the rate of the CPU."]
    THE_EMC_USES_A_CLOCK = 1,
}
impl From<EMCDIV_A> for bool {
    #[inline(always)]
    fn from(variant: EMCDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCDIV` reader - Selects the EMC clock rate relative to the CPU clock."]
pub struct EMCDIV_R(crate::FieldReader<bool, EMCDIV_A>);
impl EMCDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMCDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCDIV_A {
        match self.bits {
            false => EMCDIV_A::THE_EMC_USES_THE_SAM,
            true => EMCDIV_A::THE_EMC_USES_A_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `THE_EMC_USES_THE_SAM`"]
    #[inline(always)]
    pub fn is_the_emc_uses_the_sam(&self) -> bool {
        **self == EMCDIV_A::THE_EMC_USES_THE_SAM
    }
    #[doc = "Checks if the value of the field is `THE_EMC_USES_A_CLOCK`"]
    #[inline(always)]
    pub fn is_the_emc_uses_a_clock(&self) -> bool {
        **self == EMCDIV_A::THE_EMC_USES_A_CLOCK
    }
}
impl core::ops::Deref for EMCDIV_R {
    type Target = crate::FieldReader<bool, EMCDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCDIV` writer - Selects the EMC clock rate relative to the CPU clock."]
pub struct EMCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCDIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The EMC uses the same clock as the CPU."]
    #[inline(always)]
    pub fn the_emc_uses_the_sam(self) -> &'a mut W {
        self.variant(EMCDIV_A::THE_EMC_USES_THE_SAM)
    }
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    #[inline(always)]
    pub fn the_emc_uses_a_clock(self) -> &'a mut W {
        self.variant(EMCDIV_A::THE_EMC_USES_A_CLOCK)
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
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&self) -> EMCDIV_R {
        EMCDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&mut self) -> EMCDIV_W {
        EMCDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Memory Controller Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcclksel](index.html) module"]
pub struct EMCCLKSEL_SPEC;
impl crate::RegisterSpec for EMCCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcclksel::R](R) reader structure"]
impl crate::Readable for EMCCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcclksel::W](W) writer structure"]
impl crate::Writable for EMCCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCCLKSEL to value 0"]
impl crate::Resettable for EMCCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
