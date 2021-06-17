#[doc = "Register `ADOM` reader"]
pub struct R(crate::R<ADOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADOM` writer"]
pub struct W(crate::W<ADOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOM_SPEC>;
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
impl From<crate::W<ADOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub struct DOM_R(crate::FieldReader<u8, u8>);
impl DOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOM` writer - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub struct DOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&mut self) -> DOM_W {
        DOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm value for Day of Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adom](index.html) module"]
pub struct ADOM_SPEC;
impl crate::RegisterSpec for ADOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adom::R](R) reader structure"]
impl crate::Readable for ADOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adom::W](W) writer structure"]
impl crate::Writable for ADOM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADOM to value 0"]
impl crate::Resettable for ADOM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
