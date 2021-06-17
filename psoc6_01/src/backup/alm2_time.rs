#[doc = "Register `ALM2_TIME` reader"]
pub struct R(crate::R<ALM2_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALM2_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALM2_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALM2_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALM2_TIME` writer"]
pub struct W(crate::W<ALM2_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALM2_TIME_SPEC>;
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
impl From<crate::W<ALM2_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALM2_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALM_SEC` reader - Alarm seconds in BCD, 0-59"]
pub struct ALM_SEC_R(crate::FieldReader<u8, u8>);
impl ALM_SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_SEC` writer - Alarm seconds in BCD, 0-59"]
pub struct ALM_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `ALM_SEC_EN` reader - Alarm second enable: 0=ignore, 1=match"]
pub struct ALM_SEC_EN_R(crate::FieldReader<bool, bool>);
impl ALM_SEC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_SEC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_SEC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_SEC_EN` writer - Alarm second enable: 0=ignore, 1=match"]
pub struct ALM_SEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_SEC_EN_W<'a> {
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
#[doc = "Field `ALM_MIN` reader - Alarm minutes in BCD, 0-59"]
pub struct ALM_MIN_R(crate::FieldReader<u8, u8>);
impl ALM_MIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_MIN` writer - Alarm minutes in BCD, 0-59"]
pub struct ALM_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `ALM_MIN_EN` reader - Alarm minutes enable: 0=ignore, 1=match"]
pub struct ALM_MIN_EN_R(crate::FieldReader<bool, bool>);
impl ALM_MIN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_MIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_MIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_MIN_EN` writer - Alarm minutes enable: 0=ignore, 1=match"]
pub struct ALM_MIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MIN_EN_W<'a> {
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
#[doc = "Field `ALM_HOUR` reader - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub struct ALM_HOUR_R(crate::FieldReader<u8, u8>);
impl ALM_HOUR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_HOUR` writer - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub struct ALM_HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `ALM_HOUR_EN` reader - Alarm hour enable: 0=ignore, 1=match"]
pub struct ALM_HOUR_EN_R(crate::FieldReader<bool, bool>);
impl ALM_HOUR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_HOUR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_HOUR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_HOUR_EN` writer - Alarm hour enable: 0=ignore, 1=match"]
pub struct ALM_HOUR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_HOUR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ALM_DAY` reader - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub struct ALM_DAY_R(crate::FieldReader<u8, u8>);
impl ALM_DAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM_DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_DAY` writer - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub struct ALM_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `ALM_DAY_EN` reader - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub struct ALM_DAY_EN_R(crate::FieldReader<bool, bool>);
impl ALM_DAY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALM_DAY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM_DAY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM_DAY_EN` writer - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub struct ALM_DAY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DAY_EN_W<'a> {
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
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&self) -> ALM_SEC_R {
        ALM_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&self) -> ALM_SEC_EN_R {
        ALM_SEC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&self) -> ALM_MIN_R {
        ALM_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&self) -> ALM_MIN_EN_R {
        ALM_MIN_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&self) -> ALM_HOUR_R {
        ALM_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&self) -> ALM_HOUR_EN_R {
        ALM_HOUR_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&self) -> ALM_DAY_R {
        ALM_DAY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&self) -> ALM_DAY_EN_R {
        ALM_DAY_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&mut self) -> ALM_SEC_W {
        ALM_SEC_W { w: self }
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&mut self) -> ALM_SEC_EN_W {
        ALM_SEC_EN_W { w: self }
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&mut self) -> ALM_MIN_W {
        ALM_MIN_W { w: self }
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&mut self) -> ALM_MIN_EN_W {
        ALM_MIN_EN_W { w: self }
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&mut self) -> ALM_HOUR_W {
        ALM_HOUR_W { w: self }
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&mut self) -> ALM_HOUR_EN_W {
        ALM_HOUR_EN_W { w: self }
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&mut self) -> ALM_DAY_W {
        ALM_DAY_W { w: self }
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&mut self) -> ALM_DAY_EN_W {
        ALM_DAY_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm2_time](index.html) module"]
pub struct ALM2_TIME_SPEC;
impl crate::RegisterSpec for ALM2_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alm2_time::R](R) reader structure"]
impl crate::Readable for ALM2_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alm2_time::W](W) writer structure"]
impl crate::Writable for ALM2_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALM2_TIME to value 0x0100_0000"]
impl crate::Resettable for ALM2_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
