#[doc = "Register `RX_FIFO_RD` reader"]
pub struct R(crate::R<RX_FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. During debug it may be required to read the FIFO without a corresponding POP of the FIFO. This can be achieved by using the RX_FIFO_RD_SILENT register A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. During debug it may be required to read the FIFO without a corresponding POP of the FIFO. This can be achieved by using the RX_FIFO_RD_SILENT register A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receiver FIFO read register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd](index.html) module"]
pub struct RX_FIFO_RD_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_rd::R](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_RD to value 0"]
impl crate::Resettable for RX_FIFO_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
