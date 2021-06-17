#[doc = "Register `TSV0` reader"]
pub struct R(crate::R<TSV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSV0_SPEC>) -> Self {
        R(reader)
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
#[doc = "Field `LCE` reader - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
pub struct LCE_R(crate::FieldReader<bool, bool>);
impl LCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOR` reader - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
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
#[doc = "Field `DONE` reader - Transmission of packet was completed."]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULTICAST` reader - Packet's destination was a multicast address."]
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
#[doc = "Field `BROADCAST` reader - Packet's destination was a broadcast address."]
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
#[doc = "Field `PACKETDEFER` reader - Packet was deferred for at least one attempt, but less than an excessive defer."]
pub struct PACKETDEFER_R(crate::FieldReader<bool, bool>);
impl PACKETDEFER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PACKETDEFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACKETDEFER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXDF` reader - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
pub struct EXDF_R(crate::FieldReader<bool, bool>);
impl EXDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCOL` reader - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
pub struct EXCOL_R(crate::FieldReader<bool, bool>);
impl EXCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCOL` reader - Late Collision. Collision occurred beyond collision window, 512 bit times."]
pub struct LCOL_R(crate::FieldReader<bool, bool>);
impl LCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIANT` reader - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
pub struct GIANT_R(crate::FieldReader<bool, bool>);
impl GIANT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GIANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GIANT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERRUN` reader - Host side caused buffer underrun."]
pub struct UNDERRUN_R(crate::FieldReader<bool, bool>);
impl UNDERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTALBYTES` reader - The total number of bytes transferred including collided attempts."]
pub struct TOTALBYTES_R(crate::FieldReader<u16, u16>);
impl TOTALBYTES_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTALBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTALBYTES_R {
    type Target = crate::FieldReader<u16, u16>;
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
#[doc = "Field `BACKPRESSURE` reader - Carrier-sense method backpressure was previously applied."]
pub struct BACKPRESSURE_R(crate::FieldReader<bool, bool>);
impl BACKPRESSURE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACKPRESSURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKPRESSURE_R {
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
    #[doc = "Bit 0 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lce(&self) -> LCE_R {
        LCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission of packet was completed."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Packet's destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet's destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Packet was deferred for at least one attempt, but less than an excessive defer."]
    #[inline(always)]
    pub fn packetdefer(&self) -> PACKETDEFER_R {
        PACKETDEFER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
    #[inline(always)]
    pub fn exdf(&self) -> EXDF_R {
        EXDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Late Collision. Collision occurred beyond collision window, 512 bit times."]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
    #[inline(always)]
    pub fn giant(&self) -> GIANT_R {
        GIANT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Host side caused buffer underrun."]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:27 - The total number of bytes transferred including collided attempts."]
    #[inline(always)]
    pub fn totalbytes(&self) -> TOTALBYTES_R {
        TOTALBYTES_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Carrier-sense method backpressure was previously applied."]
    #[inline(always)]
    pub fn backpressure(&self) -> BACKPRESSURE_R {
        BACKPRESSURE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Transmit status vector 0 register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsv0](index.html) module"]
pub struct TSV0_SPEC;
impl crate::RegisterSpec for TSV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsv0::R](R) reader structure"]
impl crate::Readable for TSV0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSV0 to value 0"]
impl crate::Resettable for TSV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
