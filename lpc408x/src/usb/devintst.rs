#[doc = "Register `DEVINTST` reader"]
pub struct R(crate::R<DEVINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME` reader - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
pub struct FRAME_R(crate::FieldReader<bool, bool>);
impl FRAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_FAST` reader - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
pub struct EP_FAST_R(crate::FieldReader<bool, bool>);
impl EP_FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_SLOW` reader - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
pub struct EP_SLOW_R(crate::FieldReader<bool, bool>);
impl EP_SLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_SLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_SLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_STAT` reader - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
pub struct DEV_STAT_R(crate::FieldReader<bool, bool>);
impl DEV_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEV_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCEMPTY` reader - The command code register (USBCmdCode) is empty (New command can be written)."]
pub struct CCEMPTY_R(crate::FieldReader<bool, bool>);
impl CCEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDFULL` reader - Command data register (USBCmdData) is full (Data can be read now)."]
pub struct CDFULL_R(crate::FieldReader<bool, bool>);
impl CDFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxENDPKT` reader - The current packet in the endpoint buffer is transferred to the CPU."]
pub struct RXENDPKT_R(crate::FieldReader<bool, bool>);
impl RXENDPKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENDPKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENDPKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxENDPKT` reader - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
pub struct TXENDPKT_R(crate::FieldReader<bool, bool>);
impl TXENDPKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDPKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENDPKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_RLZED` reader - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
pub struct EP_RLZED_R(crate::FieldReader<bool, bool>);
impl EP_RLZED_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_RLZED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_RLZED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_INT` reader - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
pub struct ERR_INT_R(crate::FieldReader<bool, bool>);
impl ERR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_fast(&self) -> EP_FAST_R {
        EP_FAST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_slow(&self) -> EP_SLOW_R {
        EP_SLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DEV_STAT_R {
        DEV_STAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The command code register (USBCmdCode) is empty (New command can be written)."]
    #[inline(always)]
    pub fn ccempty(&self) -> CCEMPTY_R {
        CCEMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Command data register (USBCmdData) is full (Data can be read now)."]
    #[inline(always)]
    pub fn cdfull(&self) -> CDFULL_R {
        CDFULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The current packet in the endpoint buffer is transferred to the CPU."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RXENDPKT_R {
        RXENDPKT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TXENDPKT_R {
        TXENDPKT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
    #[inline(always)]
    pub fn ep_rlzed(&self) -> EP_RLZED_R {
        EP_RLZED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "USB Device Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintst](index.html) module"]
pub struct DEVINTST_SPEC;
impl crate::RegisterSpec for DEVINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devintst::R](R) reader structure"]
impl crate::Readable for DEVINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVINTST to value 0x10"]
impl crate::Resettable for DEVINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
