#[doc = "Register `INTR_USBHOST_SET` reader"]
pub struct R(crate::R<INTR_USBHOST_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_USBHOST_SET` writer"]
pub struct W(crate::W<INTR_USBHOST_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_USBHOST_SET_SPEC>;
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
impl From<crate::W<INTR_USBHOST_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_USBHOST_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIRQS` reader - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct SOFIRQS_R(crate::FieldReader<bool, bool>);
impl SOFIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIRQS` writer - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct SOFIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQS_W<'a> {
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
#[doc = "Field `DIRQS` reader - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct DIRQS_R(crate::FieldReader<bool, bool>);
impl DIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRQS` writer - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct DIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQS_W<'a> {
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
#[doc = "Field `CNNIRQS` reader - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct CNNIRQS_R(crate::FieldReader<bool, bool>);
impl CNNIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNNIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNNIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNNIRQS` writer - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct CNNIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQS_W<'a> {
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
#[doc = "Field `CMPIRQS` reader - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct CMPIRQS_R(crate::FieldReader<bool, bool>);
impl CMPIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPIRQS` writer - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct CMPIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQS_W<'a> {
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
#[doc = "Field `URIRQS` reader - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct URIRQS_R(crate::FieldReader<bool, bool>);
impl URIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        URIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URIRQS` writer - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct URIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQS_W<'a> {
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
#[doc = "Field `RWKIRQS` reader - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct RWKIRQS_R(crate::FieldReader<bool, bool>);
impl RWKIRQS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKIRQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKIRQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKIRQS` writer - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct RWKIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQS_W<'a> {
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
#[doc = "Field `RSVD_6` reader - N/A"]
pub struct RSVD_6_R(crate::FieldReader<bool, bool>);
impl RSVD_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_6` writer - N/A"]
pub struct RSVD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_6_W<'a> {
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
#[doc = "Field `TCANS` reader - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct TCANS_R(crate::FieldReader<bool, bool>);
impl TCANS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCANS` writer - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub struct TCANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCANS_W<'a> {
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
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&self) -> SOFIRQS_R {
        SOFIRQS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&self) -> DIRQS_R {
        DIRQS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&self) -> CNNIRQS_R {
        CNNIRQS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&self) -> CMPIRQS_R {
        CMPIRQS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&self) -> URIRQS_R {
        URIRQS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&self) -> RWKIRQS_R {
        RWKIRQS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&self) -> TCANS_R {
        TCANS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&mut self) -> SOFIRQS_W {
        SOFIRQS_W { w: self }
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&mut self) -> DIRQS_W {
        DIRQS_W { w: self }
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&mut self) -> CNNIRQS_W {
        CNNIRQS_W { w: self }
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&mut self) -> CMPIRQS_W {
        CMPIRQS_W { w: self }
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&mut self) -> URIRQS_W {
        URIRQS_W { w: self }
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&mut self) -> RWKIRQS_W {
        RWKIRQS_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&mut self) -> TCANS_W {
        TCANS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_set](index.html) module"]
pub struct INTR_USBHOST_SET_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_set::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_set::W](W) writer structure"]
impl crate::Writable for INTR_USBHOST_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_USBHOST_SET to value 0"]
impl crate::Resettable for INTR_USBHOST_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
