#[doc = "Register `FILTERINX` reader"]
pub struct R(crate::R<FILTERINX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTERINX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTERINX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTERINX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTERINX` writer"]
pub struct W(crate::W<FILTERINX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTERINX_SPEC>;
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
impl From<crate::W<FILTERINX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTERINX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FITLINX` reader - Digital filter sampling delay for the index."]
pub struct FITLINX_R(crate::FieldReader<u32, u32>);
impl FITLINX_R {
    pub(crate) fn new(bits: u32) -> Self {
        FITLINX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FITLINX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FITLINX` writer - Digital filter sampling delay for the index."]
pub struct FITLINX_W<'a> {
    w: &'a mut W,
}
impl<'a> FITLINX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay for the index."]
    #[inline(always)]
    pub fn fitlinx(&self) -> FITLINX_R {
        FITLINX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay for the index."]
    #[inline(always)]
    pub fn fitlinx(&mut self) -> FITLINX_W {
        FITLINX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital filter register on IDX\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filterinx](index.html) module"]
pub struct FILTERINX_SPEC;
impl crate::RegisterSpec for FILTERINX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filterinx::R](R) reader structure"]
impl crate::Readable for FILTERINX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filterinx::W](W) writer structure"]
impl crate::Writable for FILTERINX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTERINX to value 0"]
impl crate::Resettable for FILTERINX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
