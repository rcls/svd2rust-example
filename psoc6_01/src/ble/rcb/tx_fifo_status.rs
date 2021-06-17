#[doc = "Register `TX_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Amount of enties in the transmitter FIFO. The value of this field ranges from 0 to 16"]
pub struct USED_R(crate::FieldReader<u8, u8>);
impl USED_R {
    pub(crate) fn new(bits: u8) -> Self {
        USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR_VALID` reader - Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is tranmitted next (when the protocol state machine is not transmitting a data frame)."]
pub struct SR_VALID_R(crate::FieldReader<bool, bool>);
impl SR_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_PTR` reader - FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
pub struct RD_PTR_R(crate::FieldReader<u8, u8>);
impl RD_PTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_PTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_PTR` reader - FIFO write pointer: FIFO location at which a new data frame is written."]
pub struct WR_PTR_R(crate::FieldReader<u8, u8>);
impl WR_PTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WR_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_PTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Amount of enties in the transmitter FIFO. The value of this field ranges from 0 to 16"]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is tranmitted next (when the protocol state machine is not transmitting a data frame)."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FIFO write pointer: FIFO location at which a new data frame is written."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Transmitter FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_status](index.html) module"]
pub struct TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
