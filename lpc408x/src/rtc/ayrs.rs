#[doc = "Register `AYRS` reader"]
pub struct R(crate::R<AYRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AYRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AYRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AYRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AYRS` writer"]
pub struct W(crate::W<AYRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AYRS_SPEC>;
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
impl From<crate::W<AYRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AYRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub struct YEAR_R(crate::FieldReader<u16, u16>);
impl YEAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR` writer - Year value in the range of 0 to 4095."]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm value for Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ayrs](index.html) module"]
pub struct AYRS_SPEC;
impl crate::RegisterSpec for AYRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ayrs::R](R) reader structure"]
impl crate::Readable for AYRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ayrs::W](W) writer structure"]
impl crate::Writable for AYRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AYRS to value 0"]
impl crate::Resettable for AYRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
