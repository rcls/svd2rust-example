#[doc = "Register `ADOY` reader"]
pub struct R(crate::R<ADOY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADOY` writer"]
pub struct W(crate::W<ADOY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOY_SPEC>;
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
impl From<crate::W<ADOY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub struct DOY_R(crate::FieldReader<u16, u16>);
impl DOY_R {
    pub(crate) fn new(bits: u16) -> Self {
        DOY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOY` writer - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub struct DOY_W<'a> {
    w: &'a mut W,
}
impl<'a> DOY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&mut self) -> DOY_W {
        DOY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm value for Day of Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adoy](index.html) module"]
pub struct ADOY_SPEC;
impl crate::RegisterSpec for ADOY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adoy::R](R) reader structure"]
impl crate::Readable for ADOY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adoy::W](W) writer structure"]
impl crate::Writable for ADOY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADOY to value 0"]
impl crate::Resettable for ADOY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
