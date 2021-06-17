#[doc = "Register `AHRS` reader"]
pub struct R(crate::R<AHRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHRS` writer"]
pub struct W(crate::W<AHRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHRS_SPEC>;
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
impl From<crate::W<AHRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub struct HOURS_R(crate::FieldReader<u8, u8>);
impl HOURS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOURS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOURS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOURS` writer - Hours value in the range of 0 to 23"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm value for Hours\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahrs](index.html) module"]
pub struct AHRS_SPEC;
impl crate::RegisterSpec for AHRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahrs::R](R) reader structure"]
impl crate::Readable for AHRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahrs::W](W) writer structure"]
impl crate::Writable for AHRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHRS to value 0"]
impl crate::Resettable for AHRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
