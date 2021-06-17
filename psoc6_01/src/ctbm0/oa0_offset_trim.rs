#[doc = "Register `OA0_OFFSET_TRIM` reader"]
pub struct R(crate::R<OA0_OFFSET_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA0_OFFSET_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA0_OFFSET_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA0_OFFSET_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA0_OFFSET_TRIM` writer"]
pub struct W(crate::W<OA0_OFFSET_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA0_OFFSET_TRIM_SPEC>;
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
impl From<crate::W<OA0_OFFSET_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA0_OFFSET_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0_OFFSET_TRIM` reader - Opamp0 offset trim"]
pub struct OA0_OFFSET_TRIM_R(crate::FieldReader<u8, u8>);
impl OA0_OFFSET_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OA0_OFFSET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA0_OFFSET_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA0_OFFSET_TRIM` writer - Opamp0 offset trim"]
pub struct OA0_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Opamp0 offset trim"]
    #[inline(always)]
    pub fn oa0_offset_trim(&self) -> OA0_OFFSET_TRIM_R {
        OA0_OFFSET_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Opamp0 offset trim"]
    #[inline(always)]
    pub fn oa0_offset_trim(&mut self) -> OA0_OFFSET_TRIM_W {
        OA0_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp0 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_offset_trim](index.html) module"]
pub struct OA0_OFFSET_TRIM_SPEC;
impl crate::RegisterSpec for OA0_OFFSET_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa0_offset_trim::R](R) reader structure"]
impl crate::Readable for OA0_OFFSET_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa0_offset_trim::W](W) writer structure"]
impl crate::Writable for OA0_OFFSET_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA0_OFFSET_TRIM to value 0"]
impl crate::Resettable for OA0_OFFSET_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}