#[doc = "Register `CE_LENGTH` reader"]
pub struct R(crate::R<CE_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_LENGTH` writer"]
pub struct W(crate::W<CE_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_LENGTH_SPEC>;
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
impl From<crate::W<CE_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONNECTION_EVENT_LENGTH` reader - N/A"]
pub struct CONNECTION_EVENT_LENGTH_R(crate::FieldReader<u16, u16>);
impl CONNECTION_EVENT_LENGTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        CONNECTION_EVENT_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONNECTION_EVENT_LENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONNECTION_EVENT_LENGTH` writer - N/A"]
pub struct CONNECTION_EVENT_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTION_EVENT_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn connection_event_length(&self) -> CONNECTION_EVENT_LENGTH_R {
        CONNECTION_EVENT_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn connection_event_length(&mut self) -> CONNECTION_EVENT_LENGTH_W {
        CONNECTION_EVENT_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection event length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_length](index.html) module"]
pub struct CE_LENGTH_SPEC;
impl crate::RegisterSpec for CE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_length::R](R) reader structure"]
impl crate::Readable for CE_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_length::W](W) writer structure"]
impl crate::Writable for CE_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_LENGTH to value 0"]
impl crate::Resettable for CE_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
