#[doc = "Register `LIM[%s]` reader"]
pub struct R(crate::R<LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIM[%s]` writer"]
pub struct W(crate::W<LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIM_SPEC>;
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
impl From<crate::W<LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLIM` reader - Limit value."]
pub struct MCLIM_R(crate::FieldReader<u32, u32>);
impl MCLIM_R {
    pub(crate) fn new(bits: u32) -> Self {
        MCLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLIM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLIM` writer - Limit value."]
pub struct MCLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&self) -> MCLIM_R {
        MCLIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&mut self) -> MCLIM_W {
        MCLIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lim](index.html) module"]
pub struct LIM_SPEC;
impl crate::RegisterSpec for LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lim::R](R) reader structure"]
impl crate::Readable for LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lim::W](W) writer structure"]
impl crate::Writable for LIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIM[%s]
to value 0"]
impl crate::Resettable for LIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
