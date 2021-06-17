#[doc = "Register `CMPOS2` reader"]
pub struct R(crate::R<CMPOS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPOS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPOS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPOS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPOS2` writer"]
pub struct W(crate::W<CMPOS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPOS2_SPEC>;
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
impl From<crate::W<CMPOS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPOS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCMP2` reader - Position compare value 2."]
pub struct PCMP2_R(crate::FieldReader<u32, u32>);
impl PCMP2_R {
    pub(crate) fn new(bits: u32) -> Self {
        PCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMP2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMP2` writer - Position compare value 2."]
pub struct PCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&self) -> PCMP2_R {
        PCMP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&mut self) -> PCMP2_W {
        PCMP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpos2](index.html) module"]
pub struct CMPOS2_SPEC;
impl crate::RegisterSpec for CMPOS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpos2::R](R) reader structure"]
impl crate::Readable for CMPOS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpos2::W](W) writer structure"]
impl crate::Writable for CMPOS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPOS2 to value 0xffff_ffff"]
impl crate::Resettable for CMPOS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
