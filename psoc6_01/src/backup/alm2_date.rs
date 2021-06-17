#[doc = "Register `ALM2_DATE` reader"]
pub struct R(crate::R<ALM2_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALM2_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALM2_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALM2_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALM2_DATE` writer"]
pub struct W(crate::W<ALM2_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALM2_DATE_SPEC>;
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
impl From<crate::W<ALM2_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALM2_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALM_DATE` reader - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub struct ALM_DATE_R(crate::FieldReader<u8, u8>);
impl ALM_DATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_DATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_DATE` writer - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub struct ALM_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `ALM_DATE_EN` reader - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub struct ALM_DATE_EN_R(crate::FieldReader<bool, bool>);
impl ALM_DATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_DATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_DATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_DATE_EN` writer - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub struct ALM_DATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ALM_MON` reader - Alarm Month in BCD, 1-12"]
pub struct ALM_MON_R(crate::FieldReader<u8, u8>);
impl ALM_MON_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_MON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_MON` writer - Alarm Month in BCD, 1-12"]
pub struct ALM_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ALM_MON_EN` reader - Alarm Month enable: 0=ignore, 1=match"]
pub struct ALM_MON_EN_R(crate::FieldReader<bool, bool>);
impl ALM_MON_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_MON_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_MON_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_MON_EN` writer - Alarm Month enable: 0=ignore, 1=match"]
pub struct ALM_MON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ALM_EN` reader - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub struct ALM_EN_R(crate::FieldReader<bool, bool>);
impl ALM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_EN` writer - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub struct ALM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&self) -> ALM_DATE_R {
        ALM_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&self) -> ALM_DATE_EN_R {
        ALM_DATE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&self) -> ALM_MON_R {
        ALM_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&self) -> ALM_MON_EN_R {
        ALM_MON_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&self) -> ALM_EN_R {
        ALM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&mut self) -> ALM_DATE_W {
        ALM_DATE_W { w: self }
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&mut self) -> ALM_DATE_EN_W {
        ALM_DATE_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&mut self) -> ALM_MON_W {
        ALM_MON_W { w: self }
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&mut self) -> ALM_MON_EN_W {
        ALM_MON_EN_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&mut self) -> ALM_EN_W {
        ALM_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 2 Day of Month, Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm2_date](index.html) module"]
pub struct ALM2_DATE_SPEC;
impl crate::RegisterSpec for ALM2_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alm2_date::R](R) reader structure"]
impl crate::Readable for ALM2_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alm2_date::W](W) writer structure"]
impl crate::Writable for ALM2_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALM2_DATE to value 0x0101"]
impl crate::Resettable for ALM2_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
