#[doc = "Register `RSTCON1` reader"]
pub struct R(crate::R<RSTCON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCON1` writer"]
pub struct W(crate::W<RSTCON1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCON1_SPEC>;
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
impl From<crate::W<RSTCON1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCON1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTIOCON` reader - Reset control bit for the IOCON registers."]
pub struct RSTIOCON_R(crate::FieldReader<bool, bool>);
impl RSTIOCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIOCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIOCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIOCON` writer - Reset control bit for the IOCON registers."]
pub struct RSTIOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIOCON_W<'a> {
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
#[doc = "Field `RSTDAC` reader - D/A converter (DAC) reset control bit."]
pub struct RSTDAC_R(crate::FieldReader<bool, bool>);
impl RSTDAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTDAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDAC` writer - D/A converter (DAC) reset control bit."]
pub struct RSTDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDAC_W<'a> {
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
#[doc = "Field `RSTCANACC` reader - CAN acceptance filter reset control bit."]
pub struct RSTCANACC_R(crate::FieldReader<bool, bool>);
impl RSTCANACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCANACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTCANACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCANACC` writer - CAN acceptance filter reset control bit."]
pub struct RSTCANACC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCANACC_W<'a> {
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
#[doc = "Field `RSTCMP` reader - Comparator 0/1 reset control bit."]
pub struct RSTCMP_R(crate::FieldReader<bool, bool>);
impl RSTCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCMP` writer - Comparator 0/1 reset control bit."]
pub struct RSTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCMP_W<'a> {
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
    #[doc = "Bit 0 - Reset control bit for the IOCON registers."]
    #[inline(always)]
    pub fn rstiocon(&self) -> RSTIOCON_R {
        RSTIOCON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - D/A converter (DAC) reset control bit."]
    #[inline(always)]
    pub fn rstdac(&self) -> RSTDAC_R {
        RSTDAC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAN acceptance filter reset control bit."]
    #[inline(always)]
    pub fn rstcanacc(&self) -> RSTCANACC_R {
        RSTCANACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator 0/1 reset control bit."]
    #[inline(always)]
    pub fn rstcmp(&self) -> RSTCMP_R {
        RSTCMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset control bit for the IOCON registers."]
    #[inline(always)]
    pub fn rstiocon(&mut self) -> RSTIOCON_W {
        RSTIOCON_W { w: self }
    }
    #[doc = "Bit 1 - D/A converter (DAC) reset control bit."]
    #[inline(always)]
    pub fn rstdac(&mut self) -> RSTDAC_W {
        RSTDAC_W { w: self }
    }
    #[doc = "Bit 2 - CAN acceptance filter reset control bit."]
    #[inline(always)]
    pub fn rstcanacc(&mut self) -> RSTCANACC_W {
        RSTCANACC_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 0/1 reset control bit."]
    #[inline(always)]
    pub fn rstcmp(&mut self) -> RSTCMP_W {
        RSTCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Individual peripheral reset control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcon1](index.html) module"]
pub struct RSTCON1_SPEC;
impl crate::RegisterSpec for RSTCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcon1::R](R) reader structure"]
impl crate::Readable for RSTCON1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcon1::W](W) writer structure"]
impl crate::Writable for RSTCON1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCON1 to value 0"]
impl crate::Resettable for RSTCON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
