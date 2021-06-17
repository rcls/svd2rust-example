#[doc = "Register `INTR_USBHOST_MASK` reader"]
pub struct R(crate::R<INTR_USBHOST_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_USBHOST_MASK` writer"]
pub struct W(crate::W<INTR_USBHOST_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_USBHOST_MASK_SPEC>;
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
impl From<crate::W<INTR_USBHOST_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_USBHOST_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIRQM` reader - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub struct SOFIRQM_R(crate::FieldReader<bool, bool>);
impl SOFIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIRQM` writer - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub struct SOFIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQM_W<'a> {
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
#[doc = "Field `DIRQM` reader - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub struct DIRQM_R(crate::FieldReader<bool, bool>);
impl DIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRQM` writer - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub struct DIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQM_W<'a> {
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
#[doc = "Field `CNNIRQM` reader - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub struct CNNIRQM_R(crate::FieldReader<bool, bool>);
impl CNNIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNNIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNNIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNNIRQM` writer - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub struct CNNIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQM_W<'a> {
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
#[doc = "Field `CMPIRQM` reader - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub struct CMPIRQM_R(crate::FieldReader<bool, bool>);
impl CMPIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPIRQM` writer - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub struct CMPIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQM_W<'a> {
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
#[doc = "Field `URIRQM` reader - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub struct URIRQM_R(crate::FieldReader<bool, bool>);
impl URIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        URIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URIRQM` writer - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub struct URIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQM_W<'a> {
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
#[doc = "Field `RWKIRQM` reader - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub struct RWKIRQM_R(crate::FieldReader<bool, bool>);
impl RWKIRQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKIRQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKIRQM` writer - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub struct RWKIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQM_W<'a> {
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
#[doc = "Field `TCANM` reader - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub struct TCANM_R(crate::FieldReader<bool, bool>);
impl TCANM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCANM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCANM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCANM` writer - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub struct TCANM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCANM_W<'a> {
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
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&self) -> SOFIRQM_R {
        SOFIRQM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&self) -> DIRQM_R {
        DIRQM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&self) -> CNNIRQM_R {
        CNNIRQM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&self) -> CMPIRQM_R {
        CMPIRQM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&self) -> URIRQM_R {
        URIRQM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&self) -> RWKIRQM_R {
        RWKIRQM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&self) -> TCANM_R {
        TCANM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&mut self) -> SOFIRQM_W {
        SOFIRQM_W { w: self }
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&mut self) -> DIRQM_W {
        DIRQM_W { w: self }
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&mut self) -> CNNIRQM_W {
        CNNIRQM_W { w: self }
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&mut self) -> CMPIRQM_W {
        CMPIRQM_W { w: self }
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&mut self) -> URIRQM_W {
        URIRQM_W { w: self }
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&mut self) -> RWKIRQM_W {
        RWKIRQM_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&mut self) -> TCANM_W {
        TCANM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_mask](index.html) module"]
pub struct INTR_USBHOST_MASK_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_mask::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_mask::W](W) writer structure"]
impl crate::Writable for INTR_USBHOST_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_USBHOST_MASK to value 0"]
impl crate::Resettable for INTR_USBHOST_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
