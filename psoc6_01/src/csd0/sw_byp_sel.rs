#[doc = "Register `SW_BYP_SEL` reader"]
pub struct R(crate::R<SW_BYP_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_BYP_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_BYP_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_BYP_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_BYP_SEL` writer"]
pub struct W(crate::W<SW_BYP_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_BYP_SEL_SPEC>;
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
impl From<crate::W<SW_BYP_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_BYP_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_BYA` reader - Set corresponding switch"]
pub struct SW_BYA_R(crate::FieldReader<bool, bool>);
impl SW_BYA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_BYA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_BYA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_BYA` writer - Set corresponding switch"]
pub struct SW_BYA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_BYA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SW_BYB` reader - Set corresponding switch"]
pub struct SW_BYB_R(crate::FieldReader<bool, bool>);
impl SW_BYB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_BYB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_BYB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_BYB` writer - Set corresponding switch"]
pub struct SW_BYB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_BYB_W<'a> {
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
#[doc = "Field `SW_CBCC` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub struct SW_CBCC_R(crate::FieldReader<bool, bool>);
impl SW_CBCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_CBCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_CBCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_CBCC` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub struct SW_CBCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CBCC_W<'a> {
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
impl R {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&self) -> SW_BYA_R {
        SW_BYA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&self) -> SW_BYB_R {
        SW_BYB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&self) -> SW_CBCC_R {
        SW_CBCC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&mut self) -> SW_BYA_W {
        SW_BYA_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&mut self) -> SW_BYB_W {
        SW_BYB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&mut self) -> SW_CBCC_W {
        SW_CBCC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AMUXBUS bypass switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_byp_sel](index.html) module"]
pub struct SW_BYP_SEL_SPEC;
impl crate::RegisterSpec for SW_BYP_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_byp_sel::R](R) reader structure"]
impl crate::Readable for SW_BYP_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_byp_sel::W](W) writer structure"]
impl crate::Writable for SW_BYP_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_BYP_SEL to value 0"]
impl crate::Resettable for SW_BYP_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
