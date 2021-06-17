#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "1: The time counters are enabled."]
    THE_TIME_COUNTERS_AR = 1,
    #[doc = "0: The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_AR = 0,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable."]
pub struct CLKEN_R(crate::FieldReader<bool, CLKEN_A>);
impl CLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            true => CLKEN_A::THE_TIME_COUNTERS_AR,
            false => CLKEN_A::THE_TIME_COUNTERS_AR,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline(always)]
    pub fn is_the_time_counters_ar(&self) -> bool {
        **self == CLKEN_A::THE_TIME_COUNTERS_AR
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline(always)]
    pub fn is_the_time_counters_ar(&self) -> bool {
        **self == CLKEN_A::THE_TIME_COUNTERS_AR
    }
}
impl core::ops::Deref for CLKEN_R {
    type Target = crate::FieldReader<bool, CLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEN` writer - Clock Enable."]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKEN_A::THE_TIME_COUNTERS_AR)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKEN_A::THE_TIME_COUNTERS_AR)
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
#[doc = "CTC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCRST_A {
    #[doc = "1: When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET = 1,
    #[doc = "0: No effect."]
    NO_EFFECT_ = 0,
}
impl From<CTCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CTCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCRST` reader - CTC Reset."]
pub struct CTCRST_R(crate::FieldReader<bool, CTCRST_A>);
impl CTCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCRST_A {
        match self.bits {
            true => CTCRST_A::RESET,
            false => CTCRST_A::NO_EFFECT_,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == CTCRST_A::RESET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline(always)]
    pub fn is_no_effect_(&self) -> bool {
        **self == CTCRST_A::NO_EFFECT_
    }
}
impl core::ops::Deref for CTCRST_R {
    type Target = crate::FieldReader<bool, CTCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCRST` writer - CTC Reset."]
pub struct CTCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTCRST_A::RESET)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(CTCRST_A::NO_EFFECT_)
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
#[doc = "Calibration counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCALEN_A {
    #[doc = "1: The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUN = 1,
    #[doc = "0: The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUN = 0,
}
impl From<CCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCALEN` reader - Calibration counter enable."]
pub struct CCALEN_R(crate::FieldReader<bool, CCALEN_A>);
impl CCALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCALEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCALEN_A {
        match self.bits {
            true => CCALEN_A::THE_CALIBRATION_COUN,
            false => CCALEN_A::THE_CALIBRATION_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline(always)]
    pub fn is_the_calibration_coun(&self) -> bool {
        **self == CCALEN_A::THE_CALIBRATION_COUN
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline(always)]
    pub fn is_the_calibration_coun(&self) -> bool {
        **self == CCALEN_A::THE_CALIBRATION_COUN
    }
}
impl core::ops::Deref for CCALEN_R {
    type Target = crate::FieldReader<bool, CCALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCALEN` writer - Calibration counter enable."]
pub struct CCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCALEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALEN_A::THE_CALIBRATION_COUN)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALEN_A::THE_CALIBRATION_COUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&self) -> CCALEN_R {
        CCALEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CTCRST_W {
        CTCRST_W { w: self }
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&mut self) -> CCALEN_W {
        CCALEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
