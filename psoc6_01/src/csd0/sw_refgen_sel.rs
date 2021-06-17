#[doc = "Register `SW_REFGEN_SEL` reader"]
pub struct R(crate::R<SW_REFGEN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_REFGEN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_REFGEN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_REFGEN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_REFGEN_SEL` writer"]
pub struct W(crate::W<SW_REFGEN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_REFGEN_SEL_SPEC>;
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
impl From<crate::W<SW_REFGEN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_REFGEN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_IAIB` reader - Set corresponding switch"]
pub struct SW_IAIB_R(crate::FieldReader<bool, bool>);
impl SW_IAIB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IAIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IAIB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IAIB` writer - Set corresponding switch"]
pub struct SW_IAIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IAIB_W<'a> {
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
#[doc = "Field `SW_IBCB` reader - Set corresponding switch"]
pub struct SW_IBCB_R(crate::FieldReader<bool, bool>);
impl SW_IBCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_IBCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_IBCB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_IBCB` writer - Set corresponding switch"]
pub struct SW_IBCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IBCB_W<'a> {
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
#[doc = "Field `SW_SGMB` reader - Set corresponding switch"]
pub struct SW_SGMB_R(crate::FieldReader<bool, bool>);
impl SW_SGMB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SGMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SGMB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SGMB` writer - Set corresponding switch"]
pub struct SW_SGMB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGMB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SW_SGRP` reader - Set corresponding switch"]
pub struct SW_SGRP_R(crate::FieldReader<bool, bool>);
impl SW_SGRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SGRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SGRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SGRP` writer - Set corresponding switch"]
pub struct SW_SGRP_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGRP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SW_SGRE` reader - Set corresponding switch"]
pub struct SW_SGRE_R(crate::FieldReader<bool, bool>);
impl SW_SGRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SGRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SGRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SGRE` writer - Set corresponding switch"]
pub struct SW_SGRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SW_SGR` reader - Set corresponding switch"]
pub struct SW_SGR_R(crate::FieldReader<bool, bool>);
impl SW_SGR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_SGR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_SGR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_SGR` writer - Set corresponding switch"]
pub struct SW_SGR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&self) -> SW_IAIB_R {
        SW_IAIB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&self) -> SW_IBCB_R {
        SW_IBCB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&self) -> SW_SGMB_R {
        SW_SGMB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&self) -> SW_SGRP_R {
        SW_SGRP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&self) -> SW_SGRE_R {
        SW_SGRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&self) -> SW_SGR_R {
        SW_SGR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&mut self) -> SW_IAIB_W {
        SW_IAIB_W { w: self }
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&mut self) -> SW_IBCB_W {
        SW_IBCB_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&mut self) -> SW_SGMB_W {
        SW_SGMB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&mut self) -> SW_SGRP_W {
        SW_SGRP_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&mut self) -> SW_SGRE_W {
        SW_SGRE_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&mut self) -> SW_SGR_W {
        SW_SGR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Generator Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_refgen_sel](index.html) module"]
pub struct SW_REFGEN_SEL_SPEC;
impl crate::RegisterSpec for SW_REFGEN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_refgen_sel::R](R) reader structure"]
impl crate::Readable for SW_REFGEN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_refgen_sel::W](W) writer structure"]
impl crate::Writable for SW_REFGEN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_REFGEN_SEL to value 0"]
impl crate::Resettable for SW_REFGEN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
