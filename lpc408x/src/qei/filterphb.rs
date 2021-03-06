#[doc = "Register `FILTERPHB` reader"]
pub struct R(crate::R<FILTERPHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTERPHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTERPHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTERPHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTERPHB` writer"]
pub struct W(crate::W<FILTERPHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTERPHB_SPEC>;
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
impl From<crate::W<FILTERPHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTERPHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTB` reader - Digital filter sampling delay for PhB."]
pub struct FILTB_R(crate::FieldReader<u32, u32>);
impl FILTB_R {
    pub(crate) fn new(bits: u32) -> Self {
        FILTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTB` writer - Digital filter sampling delay for PhB."]
pub struct FILTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhB."]
    #[inline(always)]
    pub fn filtb(&self) -> FILTB_R {
        FILTB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhB."]
    #[inline(always)]
    pub fn filtb(&mut self) -> FILTB_W {
        FILTB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital filter register on PHB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filterphb](index.html) module"]
pub struct FILTERPHB_SPEC;
impl crate::RegisterSpec for FILTERPHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filterphb::R](R) reader structure"]
impl crate::Readable for FILTERPHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filterphb::W](W) writer structure"]
impl crate::Writable for FILTERPHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTERPHB to value 0"]
impl crate::Resettable for FILTERPHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
