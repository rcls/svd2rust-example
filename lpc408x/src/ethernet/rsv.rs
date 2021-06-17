#[doc = "Register `RSV` reader"]
pub struct R(crate::R<RSV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBC` reader - Received byte count. Indicates length of received frame."]
pub struct RBC_R(crate::FieldReader<u16, u16>);
impl RBC_R {
    pub(crate) fn new(bits: u16) -> Self {
        RBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPI` reader - Packet previously ignored. Indicates that a packet was dropped."]
pub struct PPI_R(crate::FieldReader<bool, bool>);
impl PPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDVSEEN` reader - RXDV event previously seen. Indicates that the last receive event seen was not long enough to be a valid packet."]
pub struct RXDVSEEN_R(crate::FieldReader<bool, bool>);
impl RXDVSEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDVSEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDVSEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CESEEN` reader - Carrier event previously seen. Indicates that at some time since the last receive statistics, a carrier event was detected."]
pub struct CESEEN_R(crate::FieldReader<bool, bool>);
impl CESEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CESEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CESEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV` reader - Receive code violation. Indicates that received PHY data does not represent a valid receive code."]
pub struct RCV_R(crate::FieldReader<bool, bool>);
impl RCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERR` reader - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
pub struct CRCERR_R(crate::FieldReader<bool, bool>);
impl CRCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCERR` reader - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
pub struct LCERR_R(crate::FieldReader<bool, bool>);
impl LCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOR` reader - Length out of range. Indicates that frame type/length field was larger than 1518 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
pub struct LOR_R(crate::FieldReader<bool, bool>);
impl LOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROK` reader - Receive OK. The packet had valid CRC and no symbol errors."]
pub struct ROK_R(crate::FieldReader<bool, bool>);
impl ROK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULTICAST` reader - The packet destination was a multicast address."]
pub struct MULTICAST_R(crate::FieldReader<bool, bool>);
impl MULTICAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULTICAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULTICAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROADCAST` reader - The packet destination was a broadcast address."]
pub struct BROADCAST_R(crate::FieldReader<bool, bool>);
impl BROADCAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROADCAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROADCAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIBBLENIBBLE` reader - Indicates that after the end of packet another 1-7 bits were received. A single nibble, called dribble nibble, is formed but not sent out."]
pub struct DRIBBLENIBBLE_R(crate::FieldReader<bool, bool>);
impl DRIBBLENIBBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRIBBLENIBBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIBBLENIBBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROLFRAME` reader - The frame was a control frame."]
pub struct CONTROLFRAME_R(crate::FieldReader<bool, bool>);
impl CONTROLFRAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONTROLFRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROLFRAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE` reader - The frame was a control frame with a valid PAUSE opcode."]
pub struct PAUSE_R(crate::FieldReader<bool, bool>);
impl PAUSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UO` reader - Unsupported Opcode. The current frame was recognized as a Control Frame but contains an unknown opcode."]
pub struct UO_R(crate::FieldReader<bool, bool>);
impl UO_R {
    pub(crate) fn new(bits: bool) -> Self {
        UO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLAN` reader - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
pub struct VLAN_R(crate::FieldReader<bool, bool>);
impl VLAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Received byte count. Indicates length of received frame."]
    #[inline(always)]
    pub fn rbc(&self) -> RBC_R {
        RBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Packet previously ignored. Indicates that a packet was dropped."]
    #[inline(always)]
    pub fn ppi(&self) -> PPI_R {
        PPI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXDV event previously seen. Indicates that the last receive event seen was not long enough to be a valid packet."]
    #[inline(always)]
    pub fn rxdvseen(&self) -> RXDVSEEN_R {
        RXDVSEEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Carrier event previously seen. Indicates that at some time since the last receive statistics, a carrier event was detected."]
    #[inline(always)]
    pub fn ceseen(&self) -> CESEEN_R {
        CESEEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive code violation. Indicates that received PHY data does not represent a valid receive code."]
    #[inline(always)]
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lcerr(&self) -> LCERR_R {
        LCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Length out of range. Indicates that frame type/length field was larger than 1518 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Receive OK. The packet had valid CRC and no symbol errors."]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The packet destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The packet destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Indicates that after the end of packet another 1-7 bits were received. A single nibble, called dribble nibble, is formed but not sent out."]
    #[inline(always)]
    pub fn dribblenibble(&self) -> DRIBBLENIBBLE_R {
        DRIBBLENIBBLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Unsupported Opcode. The current frame was recognized as a Control Frame but contains an unknown opcode."]
    #[inline(always)]
    pub fn uo(&self) -> UO_R {
        UO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
#[doc = "Receive status vector register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv](index.html) module"]
pub struct RSV_SPEC;
impl crate::RegisterSpec for RSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsv::R](R) reader structure"]
impl crate::Readable for RSV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSV to value 0"]
impl crate::Resettable for RSV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
