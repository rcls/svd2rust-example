#[doc = "Register `RTC_AUX` reader"]
pub struct R(crate::R<RTC_AUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_AUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_AUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_AUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_AUX` writer"]
pub struct W(crate::W<RTC_AUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_AUX_SPEC>;
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
impl From<crate::W<RTC_AUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_AUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_OSCF` reader - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub struct RTC_OSCF_R(crate::FieldReader<bool, bool>);
impl RTC_OSCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_OSCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_OSCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_OSCF` writer - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub struct RTC_OSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OSCF_W<'a> {
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
#[doc = "Field `RTC_PDOUT` reader - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub struct RTC_PDOUT_R(crate::FieldReader<bool, bool>);
impl RTC_PDOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PDOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PDOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_PDOUT` writer - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub struct RTC_PDOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PDOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&self) -> RTC_OSCF_R {
        RTC_OSCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&self) -> RTC_PDOUT_R {
        RTC_PDOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&mut self) -> RTC_OSCF_W {
        RTC_OSCF_W { w: self }
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&mut self) -> RTC_PDOUT_W {
        RTC_PDOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_aux](index.html) module"]
pub struct RTC_AUX_SPEC;
impl crate::RegisterSpec for RTC_AUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_aux::R](R) reader structure"]
impl crate::Readable for RTC_AUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_aux::W](W) writer structure"]
impl crate::Writable for RTC_AUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_AUX to value 0x10"]
impl crate::Resettable for RTC_AUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
