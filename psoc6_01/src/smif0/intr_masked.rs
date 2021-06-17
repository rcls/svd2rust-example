#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TR_TX_REQ` reader - Logical and of corresponding request and mask bits."]
pub struct TR_TX_REQ_R(crate::FieldReader<bool, bool>);
impl TR_TX_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR_TX_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_TX_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR_RX_REQ` reader - Logical and of corresponding request and mask bits."]
pub struct TR_RX_REQ_R(crate::FieldReader<bool, bool>);
impl TR_RX_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR_RX_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_RX_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub struct XIP_ALIGNMENT_ERROR_R(crate::FieldReader<bool, bool>);
impl XIP_ALIGNMENT_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIP_ALIGNMENT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIP_ALIGNMENT_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub struct TX_CMD_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_CMD_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CMD_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CMD_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub struct TX_DATA_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_DATA_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DATA_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub struct RX_DATA_FIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_DATA_FIFO_UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_FIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_FIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
