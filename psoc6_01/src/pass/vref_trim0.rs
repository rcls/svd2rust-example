#[doc = "Register `VREF_TRIM0` reader"]
pub struct R(crate::R<VREF_TRIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_TRIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_TRIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_TRIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF_TRIM0` writer"]
pub struct W(crate::W<VREF_TRIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_TRIM0_SPEC>;
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
impl From<crate::W<VREF_TRIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_TRIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREF_ABS_TRIM` reader - N/A"]
pub struct VREF_ABS_TRIM_R(crate::FieldReader<u8, u8>);
impl VREF_ABS_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_ABS_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_ABS_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_ABS_TRIM` writer - N/A"]
pub struct VREF_ABS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_ABS_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_abs_trim(&self) -> VREF_ABS_TRIM_R {
        VREF_ABS_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_abs_trim(&mut self) -> VREF_ABS_TRIM_W {
        VREF_ABS_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim0](index.html) module"]
pub struct VREF_TRIM0_SPEC;
impl crate::RegisterSpec for VREF_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref_trim0::R](R) reader structure"]
impl crate::Readable for VREF_TRIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref_trim0::W](W) writer structure"]
impl crate::Writable for VREF_TRIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF_TRIM0 to value 0"]
impl crate::Resettable for VREF_TRIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}