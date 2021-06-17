#[doc = "Register `CALIBRATION` reader"]
pub struct R(crate::R<CALIBRATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIBRATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIBRATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIBRATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIBRATION` writer"]
pub struct W(crate::W<CALIBRATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIBRATION_SPEC>;
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
impl From<crate::W<CALIBRATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIBRATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALVAL` reader - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub struct CALVAL_R(crate::FieldReader<u32, u32>);
impl CALVAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        CALVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALVAL` writer - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub struct CALVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Calibration direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALDIR_A {
    #[doc = "1: Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION = 1,
    #[doc = "0: Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_ = 0,
}
impl From<CALDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CALDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub struct CALDIR_R(crate::FieldReader<bool, CALDIR_A>);
impl CALDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALDIR_A {
        match self.bits {
            true => CALDIR_A::BACKWARD_CALIBRATION,
            false => CALDIR_A::FORWARD_CALIBRATION_,
        }
    }
    #[doc = "Checks if the value of the field is `BACKWARD_CALIBRATION`"]
    #[inline(always)]
    pub fn is_backward_calibration(&self) -> bool {
        **self == CALDIR_A::BACKWARD_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `FORWARD_CALIBRATION_`"]
    #[inline(always)]
    pub fn is_forward_calibration_(&self) -> bool {
        **self == CALDIR_A::FORWARD_CALIBRATION_
    }
}
impl core::ops::Deref for CALDIR_R {
    type Target = crate::FieldReader<bool, CALDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub struct CALDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn backward_calibration(self) -> &'a mut W {
        self.variant(CALDIR_A::BACKWARD_CALIBRATION)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn forward_calibration_(self) -> &'a mut W {
        self.variant(CALDIR_A::FORWARD_CALIBRATION_)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&mut self) -> CALVAL_W {
        CALVAL_W { w: self }
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&mut self) -> CALDIR_W {
        CALDIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calibration](index.html) module"]
pub struct CALIBRATION_SPEC;
impl crate::RegisterSpec for CALIBRATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calibration::R](R) reader structure"]
impl crate::Readable for CALIBRATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calibration::W](W) writer structure"]
impl crate::Writable for CALIBRATION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIBRATION to value 0"]
impl crate::Resettable for CALIBRATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
