#[doc = "Register `INTR_SIE_MASK` reader"]
pub struct R(crate::R<INTR_SIE_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SIE_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SIE_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SIE_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SIE_MASK` writer"]
pub struct W(crate::W<INTR_SIE_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SIE_MASK_SPEC>;
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
impl From<crate::W<INTR_SIE_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SIE_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct SOF_INTR_MASK_R(crate::FieldReader<bool, bool>);
impl SOF_INTR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_INTR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_INTR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct SOF_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INTR_MASK_W<'a> {
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
#[doc = "Field `BUS_RESET_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct BUS_RESET_INTR_MASK_R(crate::FieldReader<bool, bool>);
impl BUS_RESET_INTR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_RESET_INTR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_RESET_INTR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_RESET_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct BUS_RESET_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_INTR_MASK_W<'a> {
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
#[doc = "Field `EP0_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct EP0_INTR_MASK_R(crate::FieldReader<bool, bool>);
impl EP0_INTR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0_INTR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_INTR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct EP0_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INTR_MASK_W<'a> {
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
#[doc = "Field `LPM_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct LPM_INTR_MASK_R(crate::FieldReader<bool, bool>);
impl LPM_INTR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_INTR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_INTR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct LPM_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INTR_MASK_W<'a> {
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
#[doc = "Field `RESUME_INTR_MASK` reader - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct RESUME_INTR_MASK_R(crate::FieldReader<bool, bool>);
impl RESUME_INTR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_INTR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_INTR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME_INTR_MASK` writer - Set to 1 to enable interrupt corresponding to interrupt request register"]
pub struct RESUME_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_INTR_MASK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&self) -> SOF_INTR_MASK_R {
        SOF_INTR_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&self) -> BUS_RESET_INTR_MASK_R {
        BUS_RESET_INTR_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&self) -> EP0_INTR_MASK_R {
        EP0_INTR_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&self) -> LPM_INTR_MASK_R {
        LPM_INTR_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&self) -> RESUME_INTR_MASK_R {
        RESUME_INTR_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&mut self) -> SOF_INTR_MASK_W {
        SOF_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&mut self) -> BUS_RESET_INTR_MASK_W {
        BUS_RESET_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&mut self) -> EP0_INTR_MASK_W {
        EP0_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&mut self) -> LPM_INTR_MASK_W {
        LPM_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&mut self) -> RESUME_INTR_MASK_W {
        RESUME_INTR_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie_mask](index.html) module"]
pub struct INTR_SIE_MASK_SPEC;
impl crate::RegisterSpec for INTR_SIE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_sie_mask::R](R) reader structure"]
impl crate::Readable for INTR_SIE_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_sie_mask::W](W) writer structure"]
impl crate::Writable for INTR_SIE_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SIE_MASK to value 0"]
impl crate::Resettable for INTR_SIE_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
