#[doc = "Register `FILTERPHA` reader"]
pub struct R(crate::R<FILTERPHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTERPHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTERPHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTERPHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTERPHA` writer"]
pub struct W(crate::W<FILTERPHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTERPHA_SPEC>;
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
impl From<crate::W<FILTERPHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTERPHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTA` reader - Digital filter sampling delay for PhA."]
pub struct FILTA_R(crate::FieldReader<u32, u32>);
impl FILTA_R {
    pub(crate) fn new(bits: u32) -> Self {
        FILTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTA` writer - Digital filter sampling delay for PhA."]
pub struct FILTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhA."]
    #[inline(always)]
    pub fn filta(&self) -> FILTA_R {
        FILTA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhA."]
    #[inline(always)]
    pub fn filta(&mut self) -> FILTA_W {
        FILTA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital filter register on PHA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filterpha](index.html) module"]
pub struct FILTERPHA_SPEC;
impl crate::RegisterSpec for FILTERPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filterpha::R](R) reader structure"]
impl crate::Readable for FILTERPHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filterpha::W](W) writer structure"]
impl crate::Writable for FILTERPHA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTERPHA to value 0"]
impl crate::Resettable for FILTERPHA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
