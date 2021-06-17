#[doc = "Register `INIT_INTR` reader"]
pub struct R(crate::R<INIT_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_INTR` writer"]
pub struct W(crate::W<INIT_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_INTR_SPEC>;
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
impl From<crate::W<INIT_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_INTERVAL_EXPIRE_INTR` reader - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_INTERVAL_EXPIRE_INTR_R(crate::FieldReader<bool, bool>);
impl INIT_INTERVAL_EXPIRE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_INTERVAL_EXPIRE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_INTERVAL_EXPIRE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_INTERVAL_EXPIRE_INTR` writer - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_INTERVAL_EXPIRE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_INTERVAL_EXPIRE_INTR_W<'a> {
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
#[doc = "Field `INIT_CLOSE_WINDOW_INR` reader - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_CLOSE_WINDOW_INR_R(crate::FieldReader<bool, bool>);
impl INIT_CLOSE_WINDOW_INR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_CLOSE_WINDOW_INR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_CLOSE_WINDOW_INR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_CLOSE_WINDOW_INR` writer - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_CLOSE_WINDOW_INR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_CLOSE_WINDOW_INR_W<'a> {
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
#[doc = "Field `INIT_TX_START_INTR` reader - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_TX_START_INTR_R(crate::FieldReader<bool, bool>);
impl INIT_TX_START_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_TX_START_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_TX_START_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_TX_START_INTR` writer - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct INIT_TX_START_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_TX_START_INTR_W<'a> {
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
#[doc = "Field `MASTER_CONN_CREATED` reader - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct MASTER_CONN_CREATED_R(crate::FieldReader<bool, bool>);
impl MASTER_CONN_CREATED_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_CONN_CREATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_CONN_CREATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_CONN_CREATED` writer - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
pub struct MASTER_CONN_CREATED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CONN_CREATED_W<'a> {
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
#[doc = "Field `ADV_RX_SELF_ADDR_UNMCH_INTR` reader - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
pub struct ADV_RX_SELF_ADDR_UNMCH_INTR_R(crate::FieldReader<bool, bool>);
impl ADV_RX_SELF_ADDR_UNMCH_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_RX_SELF_ADDR_UNMCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_RX_SELF_ADDR_UNMCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_RX_SELF_ADDR_UNMCH_INTR` writer - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
pub struct ADV_RX_SELF_ADDR_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_SELF_ADDR_UNMCH_INTR_W<'a> {
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
#[doc = "Field `ADV_RX_PEER_ADDR_UNMCH_INTR` reader - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
pub struct ADV_RX_PEER_ADDR_UNMCH_INTR_R(crate::FieldReader<bool, bool>);
impl ADV_RX_PEER_ADDR_UNMCH_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_RX_PEER_ADDR_UNMCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_RX_PEER_ADDR_UNMCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_RX_PEER_ADDR_UNMCH_INTR` writer - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
pub struct ADV_RX_PEER_ADDR_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_PEER_ADDR_UNMCH_INTR_W<'a> {
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
#[doc = "Field `INITA_TX_ADDR_NOT_SET_INTR` reader - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INITA_TX_ADDR_NOT_SET_INTR_R(crate::FieldReader<bool, bool>);
impl INITA_TX_ADDR_NOT_SET_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITA_TX_ADDR_NOT_SET_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITA_TX_ADDR_NOT_SET_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITA_TX_ADDR_NOT_SET_INTR` writer - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INITA_TX_ADDR_NOT_SET_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_TX_ADDR_NOT_SET_INTR_W<'a> {
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
#[doc = "Field `INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR` reader - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R(crate::FieldReader<bool, bool>);
impl INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR` writer - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR` reader - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R(crate::FieldReader<bool, bool>);
impl INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR` writer - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub struct INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_interval_expire_intr(&self) -> INIT_INTERVAL_EXPIRE_INTR_R {
        INIT_INTERVAL_EXPIRE_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_close_window_inr(&self) -> INIT_CLOSE_WINDOW_INR_R {
        INIT_CLOSE_WINDOW_INR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_tx_start_intr(&self) -> INIT_TX_START_INTR_R {
        INIT_TX_START_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn master_conn_created(&self) -> MASTER_CONN_CREATED_R {
        MASTER_CONN_CREATED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_self_addr_unmch_intr(&self) -> ADV_RX_SELF_ADDR_UNMCH_INTR_R {
        ADV_RX_SELF_ADDR_UNMCH_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_addr_unmch_intr(&self) -> ADV_RX_PEER_ADDR_UNMCH_INTR_R {
        ADV_RX_PEER_ADDR_UNMCH_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr(&self) -> INITA_TX_ADDR_NOT_SET_INTR_R {
        INITA_TX_ADDR_NOT_SET_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_peer_addr_match_priv_mismatch_intr(
        &self,
    ) -> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_self_addr_match_priv_mismatch_intr(
        &self,
    ) -> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_interval_expire_intr(&mut self) -> INIT_INTERVAL_EXPIRE_INTR_W {
        INIT_INTERVAL_EXPIRE_INTR_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_close_window_inr(&mut self) -> INIT_CLOSE_WINDOW_INR_W {
        INIT_CLOSE_WINDOW_INR_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_tx_start_intr(&mut self) -> INIT_TX_START_INTR_W {
        INIT_TX_START_INTR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn master_conn_created(&mut self) -> MASTER_CONN_CREATED_W {
        MASTER_CONN_CREATED_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_self_addr_unmch_intr(&mut self) -> ADV_RX_SELF_ADDR_UNMCH_INTR_W {
        ADV_RX_SELF_ADDR_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_addr_unmch_intr(&mut self) -> ADV_RX_PEER_ADDR_UNMCH_INTR_W {
        ADV_RX_PEER_ADDR_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr(&mut self) -> INITA_TX_ADDR_NOT_SET_INTR_W {
        INITA_TX_ADDR_NOT_SET_INTR_W { w: self }
    }
    #[doc = "Bit 8 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_peer_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_self_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_intr](index.html) module"]
pub struct INIT_INTR_SPEC;
impl crate::RegisterSpec for INIT_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_intr::R](R) reader structure"]
impl crate::Readable for INIT_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_intr::W](W) writer structure"]
impl crate::Writable for INIT_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_INTR to value 0"]
impl crate::Resettable for INIT_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
