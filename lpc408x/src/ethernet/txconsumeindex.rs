#[doc = "Register `TXCONSUMEINDEX` reader"]
pub struct R(crate::R<TXCONSUMEINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCONSUMEINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCONSUMEINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCONSUMEINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCI` reader - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
pub struct TXCI_R(crate::FieldReader<u16, u16>);
impl TXCI_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
    #[inline(always)]
    pub fn txci(&self) -> TXCI_R {
        TXCI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit consume index register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txconsumeindex](index.html) module"]
pub struct TXCONSUMEINDEX_SPEC;
impl crate::RegisterSpec for TXCONSUMEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txconsumeindex::R](R) reader structure"]
impl crate::Readable for TXCONSUMEINDEX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXCONSUMEINDEX to value 0"]
impl crate::Resettable for TXCONSUMEINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
