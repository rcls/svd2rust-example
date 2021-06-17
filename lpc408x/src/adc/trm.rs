#[doc = "Register `TRM` reader"]
pub struct R(crate::R<TRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRM` writer"]
pub struct W(crate::W<TRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRM_SPEC>;
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
impl From<crate::W<TRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCOFFS` reader - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub struct ADCOFFS_R(crate::FieldReader<u8, u8>);
impl ADCOFFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCOFFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCOFFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCOFFS` writer - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
pub struct ADCOFFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOFFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TRIM` reader - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub struct TRIM_R(crate::FieldReader<u8, u8>);
impl TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&self) -> ADCOFFS_R {
        ADCOFFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&mut self) -> ADCOFFS_W {
        ADCOFFS_W { w: self }
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trm](index.html) module"]
pub struct TRM_SPEC;
impl crate::RegisterSpec for TRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trm::R](R) reader structure"]
impl crate::Readable for TRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trm::W](W) writer structure"]
impl crate::Writable for TRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
