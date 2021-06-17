#[doc = "Register `CONN_INTR` reader"]
pub struct R(crate::R<CONN_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_INTR` writer"]
pub struct W(crate::W<CONN_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_INTR_SPEC>;
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
impl From<crate::W<CONN_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_CLOSED` reader - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
pub struct CONN_CLOSED_R(crate::FieldReader<bool, bool>);
impl CONN_CLOSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONN_CLOSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_CLOSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_CLOSED` writer - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
pub struct CONN_CLOSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_CLOSED_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CONN_ESTB` reader - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
pub struct CONN_ESTB_R(crate::FieldReader<bool, bool>);
impl CONN_ESTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONN_ESTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_ESTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_ESTB` writer - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
pub struct CONN_ESTB_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_ESTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MAP_UPDT_DONE` reader - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
pub struct MAP_UPDT_DONE_R(crate::FieldReader<bool, bool>);
impl MAP_UPDT_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAP_UPDT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAP_UPDT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP_UPDT_DONE` writer - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
pub struct MAP_UPDT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_UPDT_DONE_W<'a> {
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
#[doc = "Field `START_CE` reader - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
pub struct START_CE_R(crate::FieldReader<bool, bool>);
impl START_CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_CE` writer - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
pub struct START_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CE_W<'a> {
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
#[doc = "Field `CLOSE_CE` reader - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
pub struct CLOSE_CE_R(crate::FieldReader<bool, bool>);
impl CLOSE_CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLOSE_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOSE_CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOSE_CE` writer - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
pub struct CLOSE_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOSE_CE_W<'a> {
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
#[doc = "Field `CE_TX_ACK` reader - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
pub struct CE_TX_ACK_R(crate::FieldReader<bool, bool>);
impl CE_TX_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE_TX_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_TX_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE_TX_ACK` writer - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
pub struct CE_TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_TX_ACK_W<'a> {
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
#[doc = "Field `CE_RX` reader - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
pub struct CE_RX_R(crate::FieldReader<bool, bool>);
impl CE_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE_RX` writer - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
pub struct CE_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_RX_W<'a> {
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
#[doc = "Field `CON_UPDT_DONE` reader - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
pub struct CON_UPDT_DONE_R(crate::FieldReader<bool, bool>);
impl CON_UPDT_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CON_UPDT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CON_UPDT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CON_UPDT_DONE` writer - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
pub struct CON_UPDT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CON_UPDT_DONE_W<'a> {
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
#[doc = "Field `DISCON_STATUS` reader - Reason for disconnect - indicates the reason the link is disconnected by hardware. 001 - connection failed to be established 010 - supervision timeout 011 - kill connection by host 100 - kill connection after ACK transmitted 101 - PDU response timer expired"]
pub struct DISCON_STATUS_R(crate::FieldReader<u8, u8>);
impl DISCON_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DISCON_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCON_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDU_STATUS` reader - Status of PDU received. This information is valid along with receive interrupt. xx1 - Bad Packet (packet with CRC error) 000 - empty PDU 010 - new data (non-empty) PDU 110 - Duplicate Packet"]
pub struct RX_PDU_STATUS_R(crate::FieldReader<u8, u8>);
impl RX_PDU_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PDU_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDU_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PING_TIMER_EXPIRD_INTR` reader - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
pub struct PING_TIMER_EXPIRD_INTR_R(crate::FieldReader<bool, bool>);
impl PING_TIMER_EXPIRD_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PING_TIMER_EXPIRD_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PING_TIMER_EXPIRD_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PING_TIMER_EXPIRD_INTR` writer - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
pub struct PING_TIMER_EXPIRD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PING_TIMER_EXPIRD_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` reader - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
pub struct PING_NEARLY_EXPIRD_INTR_R(crate::FieldReader<bool, bool>);
impl PING_NEARLY_EXPIRD_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PING_NEARLY_EXPIRD_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PING_NEARLY_EXPIRD_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` writer - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
pub struct PING_NEARLY_EXPIRD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PING_NEARLY_EXPIRD_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn conn_closed(&self) -> CONN_CLOSED_R {
        CONN_CLOSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
    #[inline(always)]
    pub fn conn_estb(&self) -> CONN_ESTB_R {
        CONN_ESTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
    #[inline(always)]
    pub fn map_updt_done(&self) -> MAP_UPDT_DONE_R {
        MAP_UPDT_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
    #[inline(always)]
    pub fn start_ce(&self) -> START_CE_R {
        START_CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
    #[inline(always)]
    pub fn close_ce(&self) -> CLOSE_CE_R {
        CLOSE_CE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
    #[inline(always)]
    pub fn ce_tx_ack(&self) -> CE_TX_ACK_R {
        CE_TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
    #[inline(always)]
    pub fn ce_rx(&self) -> CE_RX_R {
        CE_RX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn con_updt_done(&self) -> CON_UPDT_DONE_R {
        CON_UPDT_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reason for disconnect - indicates the reason the link is disconnected by hardware. 001 - connection failed to be established 010 - supervision timeout 011 - kill connection by host 100 - kill connection after ACK transmitted 101 - PDU response timer expired"]
    #[inline(always)]
    pub fn discon_status(&self) -> DISCON_STATUS_R {
        DISCON_STATUS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Status of PDU received. This information is valid along with receive interrupt. xx1 - Bad Packet (packet with CRC error) 000 - empty PDU 010 - new data (non-empty) PDU 110 - Duplicate Packet"]
    #[inline(always)]
    pub fn rx_pdu_status(&self) -> RX_PDU_STATUS_R {
        RX_PDU_STATUS_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&self) -> PING_TIMER_EXPIRD_INTR_R {
        PING_TIMER_EXPIRD_INTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&self) -> PING_NEARLY_EXPIRD_INTR_R {
        PING_NEARLY_EXPIRD_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn conn_closed(&mut self) -> CONN_CLOSED_W {
        CONN_CLOSED_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
    #[inline(always)]
    pub fn conn_estb(&mut self) -> CONN_ESTB_W {
        CONN_ESTB_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
    #[inline(always)]
    pub fn map_updt_done(&mut self) -> MAP_UPDT_DONE_W {
        MAP_UPDT_DONE_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
    #[inline(always)]
    pub fn start_ce(&mut self) -> START_CE_W {
        START_CE_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
    #[inline(always)]
    pub fn close_ce(&mut self) -> CLOSE_CE_W {
        CLOSE_CE_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
    #[inline(always)]
    pub fn ce_tx_ack(&mut self) -> CE_TX_ACK_W {
        CE_TX_ACK_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
    #[inline(always)]
    pub fn ce_rx(&mut self) -> CE_RX_W {
        CE_RX_W { w: self }
    }
    #[doc = "Bit 7 - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn con_updt_done(&mut self) -> CON_UPDT_DONE_W {
        CON_UPDT_DONE_W { w: self }
    }
    #[doc = "Bit 14 - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&mut self) -> PING_TIMER_EXPIRD_INTR_W {
        PING_TIMER_EXPIRD_INTR_W { w: self }
    }
    #[doc = "Bit 15 - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&mut self) -> PING_NEARLY_EXPIRD_INTR_W {
        PING_NEARLY_EXPIRD_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_intr](index.html) module"]
pub struct CONN_INTR_SPEC;
impl crate::RegisterSpec for CONN_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_intr::R](R) reader structure"]
impl crate::Readable for CONN_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_intr::W](W) writer structure"]
impl crate::Writable for CONN_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_INTR to value 0"]
impl crate::Resettable for CONN_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
