#[doc = "Register `CIIR` reader"]
pub struct R(crate::R<CIIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIIR` writer"]
pub struct W(crate::W<CIIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIIR_SPEC>;
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
impl From<crate::W<CIIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMSEC` reader - When 1, an increment of the Second value generates an interrupt."]
pub struct IMSEC_R(crate::FieldReader<bool, bool>);
impl IMSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMSEC` writer - When 1, an increment of the Second value generates an interrupt."]
pub struct IMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IMSEC_W<'a> {
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
#[doc = "Field `IMMIN` reader - When 1, an increment of the Minute value generates an interrupt."]
pub struct IMMIN_R(crate::FieldReader<bool, bool>);
impl IMMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMIN` writer - When 1, an increment of the Minute value generates an interrupt."]
pub struct IMMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMIN_W<'a> {
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
#[doc = "Field `IMHOUR` reader - When 1, an increment of the Hour value generates an interrupt."]
pub struct IMHOUR_R(crate::FieldReader<bool, bool>);
impl IMHOUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMHOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMHOUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMHOUR` writer - When 1, an increment of the Hour value generates an interrupt."]
pub struct IMHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMHOUR_W<'a> {
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
#[doc = "Field `IMDOM` reader - When 1, an increment of the Day of Month value generates an interrupt."]
pub struct IMDOM_R(crate::FieldReader<bool, bool>);
impl IMDOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMDOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMDOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMDOM` writer - When 1, an increment of the Day of Month value generates an interrupt."]
pub struct IMDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOM_W<'a> {
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
#[doc = "Field `IMDOW` reader - When 1, an increment of the Day of Week value generates an interrupt."]
pub struct IMDOW_R(crate::FieldReader<bool, bool>);
impl IMDOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMDOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMDOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMDOW` writer - When 1, an increment of the Day of Week value generates an interrupt."]
pub struct IMDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOW_W<'a> {
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
#[doc = "Field `IMDOY` reader - When 1, an increment of the Day of Year value generates an interrupt."]
pub struct IMDOY_R(crate::FieldReader<bool, bool>);
impl IMDOY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMDOY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMDOY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMDOY` writer - When 1, an increment of the Day of Year value generates an interrupt."]
pub struct IMDOY_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOY_W<'a> {
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
#[doc = "Field `IMMON` reader - When 1, an increment of the Month value generates an interrupt."]
pub struct IMMON_R(crate::FieldReader<bool, bool>);
impl IMMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMON` writer - When 1, an increment of the Month value generates an interrupt."]
pub struct IMMON_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMON_W<'a> {
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
#[doc = "Field `IMYEAR` reader - When 1, an increment of the Year value generates an interrupt."]
pub struct IMYEAR_R(crate::FieldReader<bool, bool>);
impl IMYEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMYEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMYEAR` writer - When 1, an increment of the Year value generates an interrupt."]
pub struct IMYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMYEAR_W<'a> {
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
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&self) -> IMSEC_R {
        IMSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&self) -> IMMIN_R {
        IMMIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&self) -> IMHOUR_R {
        IMHOUR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&self) -> IMDOM_R {
        IMDOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&self) -> IMDOW_R {
        IMDOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&self) -> IMDOY_R {
        IMDOY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&self) -> IMMON_R {
        IMMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&self) -> IMYEAR_R {
        IMYEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&mut self) -> IMSEC_W {
        IMSEC_W { w: self }
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&mut self) -> IMMIN_W {
        IMMIN_W { w: self }
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&mut self) -> IMHOUR_W {
        IMHOUR_W { w: self }
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&mut self) -> IMDOM_W {
        IMDOM_W { w: self }
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&mut self) -> IMDOW_W {
        IMDOW_W { w: self }
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&mut self) -> IMDOY_W {
        IMDOY_W { w: self }
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&mut self) -> IMMON_W {
        IMMON_W { w: self }
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&mut self) -> IMYEAR_W {
        IMYEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter Increment Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ciir](index.html) module"]
pub struct CIIR_SPEC;
impl crate::RegisterSpec for CIIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ciir::R](R) reader structure"]
impl crate::Readable for CIIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ciir::W](W) writer structure"]
impl crate::Writable for CIIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIIR to value 0"]
impl crate::Resettable for CIIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
