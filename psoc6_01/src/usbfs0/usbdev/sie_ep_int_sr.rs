#[doc = "Register `SIE_EP_INT_SR` reader"]
pub struct R(crate::R<SIE_EP_INT_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_EP_INT_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_EP_INT_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_EP_INT_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_EP_INT_SR` writer"]
pub struct W(crate::W<SIE_EP_INT_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_EP_INT_SR_SPEC>;
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
impl From<crate::W<SIE_EP_INT_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_EP_INT_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_INTR` reader - Interrupt status for EP1"]
pub struct EP1_INTR_R(crate::FieldReader<bool, bool>);
impl EP1_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_INTR` writer - Interrupt status for EP1"]
pub struct EP1_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_INTR_W<'a> {
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
#[doc = "Field `EP2_INTR` reader - Interrupt status for EP2"]
pub struct EP2_INTR_R(crate::FieldReader<bool, bool>);
impl EP2_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_INTR` writer - Interrupt status for EP2"]
pub struct EP2_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_INTR_W<'a> {
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
#[doc = "Field `EP3_INTR` reader - Interrupt status for EP3"]
pub struct EP3_INTR_R(crate::FieldReader<bool, bool>);
impl EP3_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_INTR` writer - Interrupt status for EP3"]
pub struct EP3_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_INTR_W<'a> {
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
#[doc = "Field `EP4_INTR` reader - Interrupt status for EP4"]
pub struct EP4_INTR_R(crate::FieldReader<bool, bool>);
impl EP4_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_INTR` writer - Interrupt status for EP4"]
pub struct EP4_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_INTR_W<'a> {
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
#[doc = "Field `EP5_INTR` reader - Interrupt status for EP5"]
pub struct EP5_INTR_R(crate::FieldReader<bool, bool>);
impl EP5_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_INTR` writer - Interrupt status for EP5"]
pub struct EP5_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_INTR_W<'a> {
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
#[doc = "Field `EP6_INTR` reader - Interrupt status for EP6"]
pub struct EP6_INTR_R(crate::FieldReader<bool, bool>);
impl EP6_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_INTR` writer - Interrupt status for EP6"]
pub struct EP6_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_INTR_W<'a> {
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
#[doc = "Field `EP7_INTR` reader - Interrupt status for EP7"]
pub struct EP7_INTR_R(crate::FieldReader<bool, bool>);
impl EP7_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_INTR` writer - Interrupt status for EP7"]
pub struct EP7_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_INTR_W<'a> {
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
#[doc = "Field `EP8_INTR` reader - Interrupt status for EP8"]
pub struct EP8_INTR_R(crate::FieldReader<bool, bool>);
impl EP8_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP8_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_INTR` writer - Interrupt status for EP8"]
pub struct EP8_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_INTR_W<'a> {
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
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&mut self) -> EP1_INTR_W {
        EP1_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&mut self) -> EP2_INTR_W {
        EP2_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&mut self) -> EP3_INTR_W {
        EP3_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&mut self) -> EP4_INTR_W {
        EP4_INTR_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&mut self) -> EP5_INTR_W {
        EP5_INTR_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&mut self) -> EP6_INTR_W {
        EP6_INTR_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&mut self) -> EP7_INTR_W {
        EP7_INTR_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&mut self) -> EP8_INTR_W {
        EP8_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SIE Data Endpoint Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep_int_sr](index.html) module"]
pub struct SIE_EP_INT_SR_SPEC;
impl crate::RegisterSpec for SIE_EP_INT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ep_int_sr::R](R) reader structure"]
impl crate::Readable for SIE_EP_INT_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ep_int_sr::W](W) writer structure"]
impl crate::Writable for SIE_EP_INT_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_EP_INT_SR to value 0"]
impl crate::Resettable for SIE_EP_INT_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
