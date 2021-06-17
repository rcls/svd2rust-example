#[doc = "Register `DATALENGTH` reader"]
pub struct R(crate::R<DATALENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATALENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATALENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATALENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATALENGTH` writer"]
pub struct W(crate::W<DATALENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATALENGTH_SPEC>;
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
impl From<crate::W<DATALENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATALENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALENGTH` reader - Data length value"]
pub struct DATALENGTH_R(crate::FieldReader<u16, u16>);
impl DATALENGTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATALENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALENGTH` writer - Data length value"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data length value"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data length value"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W {
        DATALENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data length register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalength](index.html) module"]
pub struct DATALENGTH_SPEC;
impl crate::RegisterSpec for DATALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datalength::R](R) reader structure"]
impl crate::Readable for DATALENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datalength::W](W) writer structure"]
impl crate::Writable for DATALENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATALENGTH to value 0"]
impl crate::Resettable for DATALENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
