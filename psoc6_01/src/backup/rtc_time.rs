#[doc = "Register `RTC_TIME` reader"]
pub struct R(crate::R<RTC_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME` writer"]
pub struct W(crate::W<RTC_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_SPEC>;
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
impl From<crate::W<RTC_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SEC` reader - Calendar seconds in BCD, 0-59"]
pub struct RTC_SEC_R(crate::FieldReader<u8, u8>);
impl RTC_SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SEC` writer - Calendar seconds in BCD, 0-59"]
pub struct RTC_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `RTC_MIN` reader - Calendar minutes in BCD, 0-59"]
pub struct RTC_MIN_R(crate::FieldReader<u8, u8>);
impl RTC_MIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MIN` writer - Calendar minutes in BCD, 0-59"]
pub struct RTC_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `RTC_HOUR` reader - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
pub struct RTC_HOUR_R(crate::FieldReader<u8, u8>);
impl RTC_HOUR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_HOUR` writer - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
pub struct RTC_HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `CTRL_12HR` reader - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub struct CTRL_12HR_R(crate::FieldReader<bool, bool>);
impl CTRL_12HR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_12HR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_12HR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_12HR` writer - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub struct CTRL_12HR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_12HR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RTC_DAY` reader - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub struct RTC_DAY_R(crate::FieldReader<u8, u8>);
impl RTC_DAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DAY` writer - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub struct RTC_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&self) -> RTC_SEC_R {
        RTC_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&self) -> RTC_MIN_R {
        RTC_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&self) -> RTC_HOUR_R {
        RTC_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&self) -> CTRL_12HR_R {
        CTRL_12HR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&self) -> RTC_DAY_R {
        RTC_DAY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&mut self) -> RTC_SEC_W {
        RTC_SEC_W { w: self }
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&mut self) -> RTC_MIN_W {
        RTC_MIN_W { w: self }
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&mut self) -> RTC_HOUR_W {
        RTC_HOUR_W { w: self }
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&mut self) -> CTRL_12HR_W {
        CTRL_12HR_W { w: self }
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&mut self) -> RTC_DAY_W {
        RTC_DAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time](index.html) module"]
pub struct RTC_TIME_SPEC;
impl crate::RegisterSpec for RTC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time::R](R) reader structure"]
impl crate::Readable for RTC_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time::W](W) writer structure"]
impl crate::Writable for RTC_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME to value 0"]
impl crate::Resettable for RTC_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
