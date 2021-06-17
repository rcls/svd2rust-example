#[doc = "Register `TXPRODUCEINDEX` reader"]
pub struct R(crate::R<TXPRODUCEINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPRODUCEINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPRODUCEINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPRODUCEINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPRODUCEINDEX` writer"]
pub struct W(crate::W<TXPRODUCEINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPRODUCEINDEX_SPEC>;
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
impl From<crate::W<TXPRODUCEINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPRODUCEINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPI` reader - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub struct TXPI_R(crate::FieldReader<u16, u16>);
impl TXPI_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPI` writer - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
pub struct TXPI_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&self) -> TXPI_R {
        TXPI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&mut self) -> TXPI_W {
        TXPI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit produce index register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txproduceindex](index.html) module"]
pub struct TXPRODUCEINDEX_SPEC;
impl crate::RegisterSpec for TXPRODUCEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txproduceindex::R](R) reader structure"]
impl crate::Readable for TXPRODUCEINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txproduceindex::W](W) writer structure"]
impl crate::Writable for TXPRODUCEINDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPRODUCEINDEX to value 0"]
impl crate::Resettable for TXPRODUCEINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
