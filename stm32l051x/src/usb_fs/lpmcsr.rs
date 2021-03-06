#[doc = "Register `LPMCSR` reader"]
pub struct R(crate::R<LPMCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCSR` writer"]
pub struct W(crate::W<LPMCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCSR_SPEC>;
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
impl From<crate::W<LPMCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BESL` reader - BESL"]
pub struct BESL_R(crate::FieldReader<u8, u8>);
impl BESL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BESL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMWAKE` reader - REMWAKE"]
pub struct REMWAKE_R(crate::FieldReader<bool, bool>);
impl REMWAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REMWAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMWAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMACK` reader - LPMACK"]
pub struct LPMACK_R(crate::FieldReader<bool, bool>);
impl LPMACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMACK` writer - LPMACK"]
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
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
#[doc = "Field `LPMEN` reader - LPMEN"]
pub struct LPMEN_R(crate::FieldReader<bool, bool>);
impl LPMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMEN` writer - LPMEN"]
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
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
impl R {
    #[doc = "Bits 4:7 - BESL"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - REMWAKE"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcsr](index.html) module"]
pub struct LPMCSR_SPEC;
impl crate::RegisterSpec for LPMCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmcsr::R](R) reader structure"]
impl crate::Readable for LPMCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcsr::W](W) writer structure"]
impl crate::Writable for LPMCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
