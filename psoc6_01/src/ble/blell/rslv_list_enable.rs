#[doc = "Register `RSLV_LIST_ENABLE[%s]` reader"]
pub struct R(crate::R<RSLV_LIST_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLV_LIST_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLV_LIST_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLV_LIST_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSLV_LIST_ENABLE[%s]` writer"]
pub struct W(crate::W<RSLV_LIST_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSLV_LIST_ENABLE_SPEC>;
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
impl From<crate::W<RSLV_LIST_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSLV_LIST_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALID_ENTRY` reader - Indicates if the index is valid"]
pub struct VALID_ENTRY_R(crate::FieldReader<bool, bool>);
impl VALID_ENTRY_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_ENTRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_ENTRY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID_ENTRY` writer - Indicates if the index is valid"]
pub struct VALID_ENTRY_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_ENTRY_W<'a> {
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
#[doc = "Field `PEER_ADDR_IRK_SET` reader - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
pub struct PEER_ADDR_IRK_SET_R(crate::FieldReader<bool, bool>);
impl PEER_ADDR_IRK_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEER_ADDR_IRK_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEER_ADDR_IRK_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEER_ADDR_IRK_SET` writer - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
pub struct PEER_ADDR_IRK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_IRK_SET_W<'a> {
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
#[doc = "Field `SELF_ADDR_IRK_SET_RX` reader - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
pub struct SELF_ADDR_IRK_SET_RX_R(crate::FieldReader<bool, bool>);
impl SELF_ADDR_IRK_SET_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELF_ADDR_IRK_SET_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELF_ADDR_IRK_SET_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELF_ADDR_IRK_SET_RX` writer - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
pub struct SELF_ADDR_IRK_SET_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_IRK_SET_RX_W<'a> {
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
#[doc = "Field `WHITELISTED_PEER` reader - Indicates if the listed peer device is in the whitelist"]
pub struct WHITELISTED_PEER_R(crate::FieldReader<bool, bool>);
impl WHITELISTED_PEER_R {
    pub(crate) fn new(bits: bool) -> Self {
        WHITELISTED_PEER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WHITELISTED_PEER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHITELISTED_PEER` writer - Indicates if the listed peer device is in the whitelist"]
pub struct WHITELISTED_PEER_W<'a> {
    w: &'a mut W,
}
impl<'a> WHITELISTED_PEER_W<'a> {
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
#[doc = "Field `PEER_ADDR_TYPE` reader - Indicates the address type of the listed peer device"]
pub struct PEER_ADDR_TYPE_R(crate::FieldReader<bool, bool>);
impl PEER_ADDR_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEER_ADDR_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEER_ADDR_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEER_ADDR_TYPE` writer - Indicates the address type of the listed peer device"]
pub struct PEER_ADDR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_TYPE_W<'a> {
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
#[doc = "Field `PEER_ADDR_RPA_VAL` reader - Indicates that the peer device RPA in the list is valid"]
pub struct PEER_ADDR_RPA_VAL_R(crate::FieldReader<bool, bool>);
impl PEER_ADDR_RPA_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEER_ADDR_RPA_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEER_ADDR_RPA_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEER_ADDR_RPA_VAL` writer - Indicates that the peer device RPA in the list is valid"]
pub struct PEER_ADDR_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_RPA_VAL_W<'a> {
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
#[doc = "Field `SELF_ADDR_RXD_RPA_VAL` reader - Indicates that the received self RPA in the list is valid"]
pub struct SELF_ADDR_RXD_RPA_VAL_R(crate::FieldReader<bool, bool>);
impl SELF_ADDR_RXD_RPA_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELF_ADDR_RXD_RPA_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELF_ADDR_RXD_RPA_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELF_ADDR_RXD_RPA_VAL` writer - Indicates that the received self RPA in the list is valid"]
pub struct SELF_ADDR_RXD_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_RXD_RPA_VAL_W<'a> {
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
#[doc = "Field `SELF_ADDR_TX_RPA_VAL` reader - Indicates that the self RPA in the list to be transmitted is valid"]
pub struct SELF_ADDR_TX_RPA_VAL_R(crate::FieldReader<bool, bool>);
impl SELF_ADDR_TX_RPA_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELF_ADDR_TX_RPA_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELF_ADDR_TX_RPA_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELF_ADDR_TX_RPA_VAL` writer - Indicates that the self RPA in the list to be transmitted is valid"]
pub struct SELF_ADDR_TX_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_TX_RPA_VAL_W<'a> {
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
#[doc = "Field `SELF_ADDR_INIT_RPA_SEL` reader - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
pub struct SELF_ADDR_INIT_RPA_SEL_R(crate::FieldReader<bool, bool>);
impl SELF_ADDR_INIT_RPA_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELF_ADDR_INIT_RPA_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELF_ADDR_INIT_RPA_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELF_ADDR_INIT_RPA_SEL` writer - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
pub struct SELF_ADDR_INIT_RPA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_INIT_RPA_SEL_W<'a> {
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
#[doc = "Field `SELF_ADDR_TYPE_TX` reader - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
pub struct SELF_ADDR_TYPE_TX_R(crate::FieldReader<bool, bool>);
impl SELF_ADDR_TYPE_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELF_ADDR_TYPE_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELF_ADDR_TYPE_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELF_ADDR_TYPE_TX` writer - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
pub struct SELF_ADDR_TYPE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_TYPE_TX_W<'a> {
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
#[doc = "Field `ENTRY_CONNECTED` reader - Indicates if the entry is already in connection with our device"]
pub struct ENTRY_CONNECTED_R(crate::FieldReader<bool, bool>);
impl ENTRY_CONNECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENTRY_CONNECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTRY_CONNECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTRY_CONNECTED` writer - Indicates if the entry is already in connection with our device"]
pub struct ENTRY_CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRY_CONNECTED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the index is valid"]
    #[inline(always)]
    pub fn valid_entry(&self) -> VALID_ENTRY_R {
        VALID_ENTRY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn peer_addr_irk_set(&self) -> PEER_ADDR_IRK_SET_R {
        PEER_ADDR_IRK_SET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn self_addr_irk_set_rx(&self) -> SELF_ADDR_IRK_SET_RX_R {
        SELF_ADDR_IRK_SET_RX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates if the listed peer device is in the whitelist"]
    #[inline(always)]
    pub fn whitelisted_peer(&self) -> WHITELISTED_PEER_R {
        WHITELISTED_PEER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates the address type of the listed peer device"]
    #[inline(always)]
    pub fn peer_addr_type(&self) -> PEER_ADDR_TYPE_R {
        PEER_ADDR_TYPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that the peer device RPA in the list is valid"]
    #[inline(always)]
    pub fn peer_addr_rpa_val(&self) -> PEER_ADDR_RPA_VAL_R {
        PEER_ADDR_RPA_VAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that the received self RPA in the list is valid"]
    #[inline(always)]
    pub fn self_addr_rxd_rpa_val(&self) -> SELF_ADDR_RXD_RPA_VAL_R {
        SELF_ADDR_RXD_RPA_VAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that the self RPA in the list to be transmitted is valid"]
    #[inline(always)]
    pub fn self_addr_tx_rpa_val(&self) -> SELF_ADDR_TX_RPA_VAL_R {
        SELF_ADDR_TX_RPA_VAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
    #[inline(always)]
    pub fn self_addr_init_rpa_sel(&self) -> SELF_ADDR_INIT_RPA_SEL_R {
        SELF_ADDR_INIT_RPA_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
    #[inline(always)]
    pub fn self_addr_type_tx(&self) -> SELF_ADDR_TYPE_TX_R {
        SELF_ADDR_TYPE_TX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates if the entry is already in connection with our device"]
    #[inline(always)]
    pub fn entry_connected(&self) -> ENTRY_CONNECTED_R {
        ENTRY_CONNECTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the index is valid"]
    #[inline(always)]
    pub fn valid_entry(&mut self) -> VALID_ENTRY_W {
        VALID_ENTRY_W { w: self }
    }
    #[doc = "Bit 1 - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn peer_addr_irk_set(&mut self) -> PEER_ADDR_IRK_SET_W {
        PEER_ADDR_IRK_SET_W { w: self }
    }
    #[doc = "Bit 2 - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn self_addr_irk_set_rx(&mut self) -> SELF_ADDR_IRK_SET_RX_W {
        SELF_ADDR_IRK_SET_RX_W { w: self }
    }
    #[doc = "Bit 3 - Indicates if the listed peer device is in the whitelist"]
    #[inline(always)]
    pub fn whitelisted_peer(&mut self) -> WHITELISTED_PEER_W {
        WHITELISTED_PEER_W { w: self }
    }
    #[doc = "Bit 4 - Indicates the address type of the listed peer device"]
    #[inline(always)]
    pub fn peer_addr_type(&mut self) -> PEER_ADDR_TYPE_W {
        PEER_ADDR_TYPE_W { w: self }
    }
    #[doc = "Bit 5 - Indicates that the peer device RPA in the list is valid"]
    #[inline(always)]
    pub fn peer_addr_rpa_val(&mut self) -> PEER_ADDR_RPA_VAL_W {
        PEER_ADDR_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 6 - Indicates that the received self RPA in the list is valid"]
    #[inline(always)]
    pub fn self_addr_rxd_rpa_val(&mut self) -> SELF_ADDR_RXD_RPA_VAL_W {
        SELF_ADDR_RXD_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 7 - Indicates that the self RPA in the list to be transmitted is valid"]
    #[inline(always)]
    pub fn self_addr_tx_rpa_val(&mut self) -> SELF_ADDR_TX_RPA_VAL_W {
        SELF_ADDR_TX_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 8 - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
    #[inline(always)]
    pub fn self_addr_init_rpa_sel(&mut self) -> SELF_ADDR_INIT_RPA_SEL_W {
        SELF_ADDR_INIT_RPA_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
    #[inline(always)]
    pub fn self_addr_type_tx(&mut self) -> SELF_ADDR_TYPE_TX_W {
        SELF_ADDR_TYPE_TX_W { w: self }
    }
    #[doc = "Bit 10 - Indicates if the entry is already in connection with our device"]
    #[inline(always)]
    pub fn entry_connected(&mut self) -> ENTRY_CONNECTED_W {
        ENTRY_CONNECTED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resolving list entry control bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_enable](index.html) module"]
pub struct RSLV_LIST_ENABLE_SPEC;
impl crate::RegisterSpec for RSLV_LIST_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rslv_list_enable::R](R) reader structure"]
impl crate::Readable for RSLV_LIST_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rslv_list_enable::W](W) writer structure"]
impl crate::Writable for RSLV_LIST_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSLV_LIST_ENABLE[%s]
to value 0"]
impl crate::Resettable for RSLV_LIST_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
