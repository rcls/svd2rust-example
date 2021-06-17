#[doc = "Register `OA1_SLOPE_OFFSET_TRIM` reader"]
pub struct R(crate::R<OA1_SLOPE_OFFSET_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA1_SLOPE_OFFSET_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA1_SLOPE_OFFSET_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA1_SLOPE_OFFSET_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA1_SLOPE_OFFSET_TRIM` writer"]
pub struct W(crate::W<OA1_SLOPE_OFFSET_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA1_SLOPE_OFFSET_TRIM_SPEC>;
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
impl From<crate::W<OA1_SLOPE_OFFSET_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA1_SLOPE_OFFSET_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1_SLOPE_OFFSET_TRIM` reader - Opamp1 slope offset drift trim"]
pub struct OA1_SLOPE_OFFSET_TRIM_R(crate::FieldReader<u8, u8>);
impl OA1_SLOPE_OFFSET_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OA1_SLOPE_OFFSET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA1_SLOPE_OFFSET_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA1_SLOPE_OFFSET_TRIM` writer - Opamp1 slope offset drift trim"]
pub struct OA1_SLOPE_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_SLOPE_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Opamp1 slope offset drift trim"]
    #[inline(always)]
    pub fn oa1_slope_offset_trim(&self) -> OA1_SLOPE_OFFSET_TRIM_R {
        OA1_SLOPE_OFFSET_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Opamp1 slope offset drift trim"]
    #[inline(always)]
    pub fn oa1_slope_offset_trim(&mut self) -> OA1_SLOPE_OFFSET_TRIM_W {
        OA1_SLOPE_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp1 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_slope_offset_trim](index.html) module"]
pub struct OA1_SLOPE_OFFSET_TRIM_SPEC;
impl crate::RegisterSpec for OA1_SLOPE_OFFSET_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa1_slope_offset_trim::R](R) reader structure"]
impl crate::Readable for OA1_SLOPE_OFFSET_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa1_slope_offset_trim::W](W) writer structure"]
impl crate::Writable for OA1_SLOPE_OFFSET_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA1_SLOPE_OFFSET_TRIM to value 0"]
impl crate::Resettable for OA1_SLOPE_OFFSET_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
