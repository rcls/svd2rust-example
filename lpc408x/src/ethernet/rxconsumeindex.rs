#[doc = "Register `RXCONSUMEINDEX` reader"]
pub struct R(crate::R<RXCONSUMEINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCONSUMEINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCONSUMEINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCONSUMEINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCONSUMEINDEX` writer"]
pub struct W(crate::W<RXCONSUMEINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCONSUMEINDEX_SPEC>;
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
impl From<crate::W<RXCONSUMEINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCONSUMEINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCONSUMEIX` reader - Index of the descriptor that is going to be processed next by the receive"]
pub struct RXCONSUMEIX_R(crate::FieldReader<u16, u16>);
impl RXCONSUMEIX_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXCONSUMEIX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCONSUMEIX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCONSUMEIX` writer - Index of the descriptor that is going to be processed next by the receive"]
pub struct RXCONSUMEIX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCONSUMEIX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&self) -> RXCONSUMEIX_R {
        RXCONSUMEIX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&mut self) -> RXCONSUMEIX_W {
        RXCONSUMEIX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive consume index register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxconsumeindex](index.html) module"]
pub struct RXCONSUMEINDEX_SPEC;
impl crate::RegisterSpec for RXCONSUMEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxconsumeindex::R](R) reader structure"]
impl crate::Readable for RXCONSUMEINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxconsumeindex::W](W) writer structure"]
impl crate::Writable for RXCONSUMEINDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCONSUMEINDEX to value 0"]
impl crate::Resettable for RXCONSUMEINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
