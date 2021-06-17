#[doc = "Register `ILR` reader"]
pub struct R(crate::R<ILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILR` writer"]
pub struct W(crate::W<ILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILR_SPEC>;
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
impl From<crate::W<ILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCIF` reader - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub struct RTCCIF_R(crate::FieldReader<bool, bool>);
impl RTCCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIF` writer - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub struct RTCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIF_W<'a> {
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
#[doc = "Field `RTCALF` reader - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub struct RTCALF_R(crate::FieldReader<bool, bool>);
impl RTCALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCALF` writer - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub struct RTCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&self) -> RTCCIF_R {
        RTCCIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&self) -> RTCALF_R {
        RTCALF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&mut self) -> RTCCIF_W {
        RTCCIF_W { w: self }
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&mut self) -> RTCALF_W {
        RTCALF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilr](index.html) module"]
pub struct ILR_SPEC;
impl crate::RegisterSpec for ILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ilr::R](R) reader structure"]
impl crate::Readable for ILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ilr::W](W) writer structure"]
impl crate::Writable for ILR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ILR to value 0"]
impl crate::Resettable for ILR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
