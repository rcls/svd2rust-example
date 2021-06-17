#[doc = "Register `ANA_TRIM1` reader"]
pub struct R(crate::R<ANA_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_TRIM1` writer"]
pub struct W(crate::W<ANA_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_TRIM1_SPEC>;
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
impl From<crate::W<ANA_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_REF_BUF_TRIM` reader - SAR Reference buffer trim"]
pub struct SAR_REF_BUF_TRIM_R(crate::FieldReader<u8, u8>);
impl SAR_REF_BUF_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAR_REF_BUF_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_REF_BUF_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_REF_BUF_TRIM` writer - SAR Reference buffer trim"]
pub struct SAR_REF_BUF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_REF_BUF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&self) -> SAR_REF_BUF_TRIM_R {
        SAR_REF_BUF_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&mut self) -> SAR_REF_BUF_TRIM_W {
        SAR_REF_BUF_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim1](index.html) module"]
pub struct ANA_TRIM1_SPEC;
impl crate::RegisterSpec for ANA_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_trim1::R](R) reader structure"]
impl crate::Readable for ANA_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_trim1::W](W) writer structure"]
impl crate::Writable for ANA_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_TRIM1 to value 0"]
impl crate::Resettable for ANA_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
