#[doc = "Register `TC[%s]` reader"]
pub struct R(crate::R<TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC[%s]` writer"]
pub struct W(crate::W<TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_SPEC>;
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
impl From<crate::W<TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCTC` reader - Timer/Counter value."]
pub struct MCTC_R(crate::FieldReader<u32, u32>);
impl MCTC_R {
    pub(crate) fn new(bits: u32) -> Self {
        MCTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCTC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCTC` writer - Timer/Counter value."]
pub struct MCTC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&self) -> MCTC_R {
        MCTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&mut self) -> MCTC_W {
        MCTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](index.html) module"]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc::R](R) reader structure"]
impl crate::Readable for TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc::W](W) writer structure"]
impl crate::Writable for TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC[%s]
to value 0"]
impl crate::Resettable for TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
