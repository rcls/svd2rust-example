#[doc = "Register `ENC_INTR` reader"]
pub struct R(crate::R<ENC_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_INTR` writer"]
pub struct W(crate::W<ENC_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_INTR_SPEC>;
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
impl From<crate::W<ENC_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTH_PASS_INTR` reader - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
pub struct AUTH_PASS_INTR_R(crate::FieldReader<bool, bool>);
impl AUTH_PASS_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTH_PASS_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTH_PASS_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTH_PASS_INTR` writer - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
pub struct AUTH_PASS_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTH_PASS_INTR_W<'a> {
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
#[doc = "Field `ECB_PROC_INTR` reader - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
pub struct ECB_PROC_INTR_R(crate::FieldReader<bool, bool>);
impl ECB_PROC_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECB_PROC_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECB_PROC_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECB_PROC_INTR` writer - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
pub struct ECB_PROC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECB_PROC_INTR_W<'a> {
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
#[doc = "Field `CCM_PROC_INTR` reader - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
pub struct CCM_PROC_INTR_R(crate::FieldReader<bool, bool>);
impl CCM_PROC_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCM_PROC_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_PROC_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM_PROC_INTR` writer - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
pub struct CCM_PROC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_PROC_INTR_W<'a> {
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
#[doc = "Field `IN_DATA_CLEAR` reader - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
pub struct IN_DATA_CLEAR_R(crate::FieldReader<bool, bool>);
impl IN_DATA_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_DATA_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DATA_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DATA_CLEAR` writer - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
pub struct IN_DATA_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DATA_CLEAR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&self) -> AUTH_PASS_INTR_R {
        AUTH_PASS_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&self) -> ECB_PROC_INTR_R {
        ECB_PROC_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&self) -> CCM_PROC_INTR_R {
        CCM_PROC_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&self) -> IN_DATA_CLEAR_R {
        IN_DATA_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&mut self) -> AUTH_PASS_INTR_W {
        AUTH_PASS_INTR_W { w: self }
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&mut self) -> ECB_PROC_INTR_W {
        ECB_PROC_INTR_W { w: self }
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&mut self) -> CCM_PROC_INTR_W {
        CCM_PROC_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&mut self) -> IN_DATA_CLEAR_W {
        IN_DATA_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Interrupt status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_intr](index.html) module"]
pub struct ENC_INTR_SPEC;
impl crate::RegisterSpec for ENC_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_intr::R](R) reader structure"]
impl crate::Readable for ENC_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_intr::W](W) writer structure"]
impl crate::Writable for ENC_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_INTR to value 0"]
impl crate::Resettable for ENC_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
