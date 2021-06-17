#[doc = "Register `AMR` reader"]
pub struct R(crate::R<AMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMR` writer"]
pub struct W(crate::W<AMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMR_SPEC>;
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
impl From<crate::W<AMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMRSEC` reader - When 1, the Second value is not compared for the alarm."]
pub struct AMRSEC_R(crate::FieldReader<bool, bool>);
impl AMRSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRSEC` writer - When 1, the Second value is not compared for the alarm."]
pub struct AMRSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRSEC_W<'a> {
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
#[doc = "Field `AMRMIN` reader - When 1, the Minutes value is not compared for the alarm."]
pub struct AMRMIN_R(crate::FieldReader<bool, bool>);
impl AMRMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRMIN` writer - When 1, the Minutes value is not compared for the alarm."]
pub struct AMRMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMIN_W<'a> {
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
#[doc = "Field `AMRHOUR` reader - When 1, the Hour value is not compared for the alarm."]
pub struct AMRHOUR_R(crate::FieldReader<bool, bool>);
impl AMRHOUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRHOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRHOUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRHOUR` writer - When 1, the Hour value is not compared for the alarm."]
pub struct AMRHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRHOUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AMRDOM` reader - When 1, the Day of Month value is not compared for the alarm."]
pub struct AMRDOM_R(crate::FieldReader<bool, bool>);
impl AMRDOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRDOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRDOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRDOM` writer - When 1, the Day of Month value is not compared for the alarm."]
pub struct AMRDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `AMRDOW` reader - When 1, the Day of Week value is not compared for the alarm."]
pub struct AMRDOW_R(crate::FieldReader<bool, bool>);
impl AMRDOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRDOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRDOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRDOW` writer - When 1, the Day of Week value is not compared for the alarm."]
pub struct AMRDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOW_W<'a> {
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
#[doc = "Field `AMRDOY` reader - When 1, the Day of Year value is not compared for the alarm."]
pub struct AMRDOY_R(crate::FieldReader<bool, bool>);
impl AMRDOY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRDOY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRDOY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRDOY` writer - When 1, the Day of Year value is not compared for the alarm."]
pub struct AMRDOY_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `AMRMON` reader - When 1, the Month value is not compared for the alarm."]
pub struct AMRMON_R(crate::FieldReader<bool, bool>);
impl AMRMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRMON` writer - When 1, the Month value is not compared for the alarm."]
pub struct AMRMON_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMON_W<'a> {
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
#[doc = "Field `AMRYEAR` reader - When 1, the Year value is not compared for the alarm."]
pub struct AMRYEAR_R(crate::FieldReader<bool, bool>);
impl AMRYEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMRYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMRYEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMRYEAR` writer - When 1, the Year value is not compared for the alarm."]
pub struct AMRYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRYEAR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&self) -> AMRSEC_R {
        AMRSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&self) -> AMRMIN_R {
        AMRMIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&self) -> AMRHOUR_R {
        AMRHOUR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&self) -> AMRDOM_R {
        AMRDOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&self) -> AMRDOW_R {
        AMRDOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&self) -> AMRDOY_R {
        AMRDOY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&self) -> AMRMON_R {
        AMRMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&self) -> AMRYEAR_R {
        AMRYEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&mut self) -> AMRSEC_W {
        AMRSEC_W { w: self }
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&mut self) -> AMRMIN_W {
        AMRMIN_W { w: self }
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&mut self) -> AMRHOUR_W {
        AMRHOUR_W { w: self }
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&mut self) -> AMRDOM_W {
        AMRDOM_W { w: self }
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&mut self) -> AMRDOW_W {
        AMRDOW_W { w: self }
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&mut self) -> AMRDOY_W {
        AMRDOY_W { w: self }
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&mut self) -> AMRMON_W {
        AMRMON_W { w: self }
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&mut self) -> AMRYEAR_W {
        AMRYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amr](index.html) module"]
pub struct AMR_SPEC;
impl crate::RegisterSpec for AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amr::R](R) reader structure"]
impl crate::Readable for AMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amr::W](W) writer structure"]
impl crate::Writable for AMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMR to value 0"]
impl crate::Resettable for AMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
