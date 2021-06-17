#[doc = "Register `RXPRODUCEINDEX` reader"]
pub struct R(crate::R<RXPRODUCEINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPRODUCEINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPRODUCEINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPRODUCEINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXPRODUCEIX` reader - Index of the descriptor that is going to be filled next by the receive datapath."]
pub struct RXPRODUCEIX_R(crate::FieldReader<u16, u16>);
impl RXPRODUCEIX_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXPRODUCEIX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPRODUCEIX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be filled next by the receive datapath."]
    #[inline(always)]
    pub fn rxproduceix(&self) -> RXPRODUCEIX_R {
        RXPRODUCEIX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive produce index register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxproduceindex](index.html) module"]
pub struct RXPRODUCEINDEX_SPEC;
impl crate::RegisterSpec for RXPRODUCEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxproduceindex::R](R) reader structure"]
impl crate::Readable for RXPRODUCEINDEX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXPRODUCEINDEX to value 0"]
impl crate::Resettable for RXPRODUCEINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
