#[doc = "Register `TMR` reader"]
pub struct R(crate::R<TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR` writer"]
pub struct W(crate::W<TMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_SPEC>;
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
impl From<crate::W<TMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT_CNT` reader - The TMR interrupt is set when TMR_CNT reaches this value."]
pub struct TIMEOUT_CNT_R(crate::FieldReader<u16, u16>);
impl TIMEOUT_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT_CNT` writer - The TMR interrupt is set when TMR_CNT reaches this value."]
pub struct TIMEOUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TIMEOUT_CNT_R {
        TIMEOUT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The TMR interrupt is set when TMR_CNT reaches this value."]
    #[inline(always)]
    pub fn timeout_cnt(&mut self) -> TIMEOUT_CNT_W {
        TIMEOUT_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](index.html) module"]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr::R](R) reader structure"]
impl crate::Readable for TMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr::W](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR to value 0xffff"]
impl crate::Resettable for TMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
