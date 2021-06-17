#[doc = "Register `HOST_ERR` reader"]
pub struct R(crate::R<HOST_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_ERR` writer"]
pub struct W(crate::W<HOST_ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_ERR_SPEC>;
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
impl From<crate::W<HOST_ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HS_A {
    #[doc = "0: Acknowledge Packet"]
    ACK = 0,
    #[doc = "1: Non-Acknowledge Packet"]
    NAK = 1,
    #[doc = "2: Stall Packet"]
    STALL = 2,
    #[doc = "3: Null Packet"]
    NULL = 3,
}
impl From<HS_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HS` reader - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct HS_R(crate::FieldReader<u8, HS_A>);
impl HS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_A {
        match self.bits {
            0 => HS_A::ACK,
            1 => HS_A::NAK,
            2 => HS_A::STALL,
            3 => HS_A::NULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == HS_A::ACK
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        **self == HS_A::NAK
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == HS_A::STALL
    }
    #[doc = "Checks if the value of the field is `NULL`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        **self == HS_A::NULL
    }
}
impl core::ops::Deref for HS_R {
    type Target = crate::FieldReader<u8, HS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS` writer - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(HS_A::ACK)
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(HS_A::NAK)
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(HS_A::STALL)
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn null(self) -> &'a mut W {
        self.variant(HS_A::NULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `STUFF` reader - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct STUFF_R(crate::FieldReader<bool, bool>);
impl STUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUFF` writer - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct STUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> STUFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TGERR` reader - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct TGERR_R(crate::FieldReader<bool, bool>);
impl TGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGERR` writer - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct TGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TGERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CRC` reader - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct CRC_R(crate::FieldReader<bool, bool>);
impl CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TOUT` reader - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct TOUT_R(crate::FieldReader<bool, bool>);
impl TOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT` writer - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RERR` reader - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct RERR_R(crate::FieldReader<bool, bool>);
impl RERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RERR` writer - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct RERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LSTSOF` reader - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct LSTSOF_R(crate::FieldReader<bool, bool>);
impl LSTSOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTSOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTSOF` writer - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct LSTSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTSOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&self) -> STUFF_R {
        STUFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&self) -> TGERR_R {
        TGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&self) -> LSTSOF_R {
        LSTSOF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&mut self) -> STUFF_W {
        STUFF_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&mut self) -> TGERR_W {
        TGERR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W {
        RERR_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&mut self) -> LSTSOF_W {
        LSTSOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Error Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_err](index.html) module"]
pub struct HOST_ERR_SPEC;
impl crate::RegisterSpec for HOST_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_err::R](R) reader structure"]
impl crate::Readable for HOST_ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_err::W](W) writer structure"]
impl crate::Writable for HOST_ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_ERR to value 0x03"]
impl crate::Resettable for HOST_ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
