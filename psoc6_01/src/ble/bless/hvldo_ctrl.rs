#[doc = "Register `HVLDO_CTRL` reader"]
pub struct R(crate::R<HVLDO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HVLDO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HVLDO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HVLDO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HVLDO_CTRL` writer"]
pub struct W(crate::W<HVLDO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HVLDO_CTRL_SPEC>;
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
impl From<crate::W<HVLDO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HVLDO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADFT_EN` reader - ADFT enable"]
pub struct ADFT_EN_R(crate::FieldReader<bool, bool>);
impl ADFT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFT_EN` writer - ADFT enable"]
pub struct ADFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_EN_W<'a> {
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
#[doc = "Field `ADFT_CTRL` reader - ADFT select"]
pub struct ADFT_CTRL_R(crate::FieldReader<u8, u8>);
impl ADFT_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADFT_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFT_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFT_CTRL` writer - ADFT select"]
pub struct ADFT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `VREF_EXT_EN` reader - Vref ext input enable."]
pub struct VREF_EXT_EN_R(crate::FieldReader<bool, bool>);
impl VREF_EXT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREF_EXT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_EXT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_EXT_EN` writer - Vref ext input enable."]
pub struct VREF_EXT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_EXT_EN_W<'a> {
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
#[doc = "Field `STATUS` reader - hvldo LV detect status"]
pub struct STATUS_R(crate::FieldReader<bool, bool>);
impl STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&self) -> ADFT_EN_R {
        ADFT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&self) -> ADFT_CTRL_R {
        ADFT_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&self) -> VREF_EXT_EN_R {
        VREF_EXT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 31 - hvldo LV detect status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&mut self) -> ADFT_EN_W {
        ADFT_EN_W { w: self }
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&mut self) -> ADFT_CTRL_W {
        ADFT_CTRL_W { w: self }
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&mut self) -> VREF_EXT_EN_W {
        VREF_EXT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HVLDO Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hvldo_ctrl](index.html) module"]
pub struct HVLDO_CTRL_SPEC;
impl crate::RegisterSpec for HVLDO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hvldo_ctrl::R](R) reader structure"]
impl crate::Readable for HVLDO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hvldo_ctrl::W](W) writer structure"]
impl crate::Writable for HVLDO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HVLDO_CTRL to value 0"]
impl crate::Resettable for HVLDO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
