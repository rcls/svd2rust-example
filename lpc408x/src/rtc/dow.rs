#[doc = "Register `DOW` reader"]
pub struct R(crate::R<DOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOW` writer"]
pub struct W(crate::W<DOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOW_SPEC>;
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
impl From<crate::W<DOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6."]
pub struct DOW_R(crate::FieldReader<u8, u8>);
impl DOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOW` writer - Day of week value in the range of 0 to 6."]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Day of Week Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dow](index.html) module"]
pub struct DOW_SPEC;
impl crate::RegisterSpec for DOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dow::R](R) reader structure"]
impl crate::Readable for DOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dow::W](W) writer structure"]
impl crate::Writable for DOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOW to value 0"]
impl crate::Resettable for DOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
