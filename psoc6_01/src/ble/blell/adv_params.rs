#[doc = "Register `ADV_PARAMS` reader"]
pub struct R(crate::R<ADV_PARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_PARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_PARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_PARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_PARAMS` writer"]
pub struct W(crate::W<ADV_PARAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_PARAMS_SPEC>;
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
impl From<crate::W<ADV_PARAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_PARAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ADDR` reader - Device own address type. 1 - Address type is random. 0 - Address type is public."]
pub struct TX_ADDR_R(crate::FieldReader<bool, bool>);
impl TX_ADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ADDR` writer - Device own address type. 1 - Address type is random. 0 - Address type is public."]
pub struct TX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ADDR_W<'a> {
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
#[doc = "Field `ADV_TYPE` reader - The Advertising type is used to determine the packet type that is used for advertising when advertising is enabled. 0x0 - Connectable undirected advertising. (adv_ind) 0x1 - Connectable directed advertising (adv_direct_ind). 0x2 - Discoverable undirected advertising (adv_discover_ind) 0x3 - Non connectable undirected advertising (adv_nonconn_ind)."]
pub struct ADV_TYPE_R(crate::FieldReader<u8, u8>);
impl ADV_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADV_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_TYPE` writer - The Advertising type is used to determine the packet type that is used for advertising when advertising is enabled. 0x0 - Connectable undirected advertising. (adv_ind) 0x1 - Connectable directed advertising (adv_direct_ind). 0x2 - Discoverable undirected advertising (adv_discover_ind) 0x3 - Non connectable undirected advertising (adv_nonconn_ind)."]
pub struct ADV_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `ADV_FILT_POLICY` reader - Advertising filter policy. The set of devices that the advertising procedure uses for device filtering is called the White List. 0x0 - Allow scan request from any device, allow connect request from any device. 0x1 - Allow scan request from devices in white list only, allow connect request from any device. 0x2 - Allow scan request from any device, allow connect request from devices in white list only. 0x3 - Allow scan request from devices in white list only, allow connect request from devices in white list only."]
pub struct ADV_FILT_POLICY_R(crate::FieldReader<u8, u8>);
impl ADV_FILT_POLICY_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADV_FILT_POLICY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_FILT_POLICY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_FILT_POLICY` writer - Advertising filter policy. The set of devices that the advertising procedure uses for device filtering is called the White List. 0x0 - Allow scan request from any device, allow connect request from any device. 0x1 - Allow scan request from devices in white list only, allow connect request from any device. 0x2 - Allow scan request from any device, allow connect request from devices in white list only. 0x3 - Allow scan request from devices in white list only, allow connect request from devices in white list only."]
pub struct ADV_FILT_POLICY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_FILT_POLICY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `ADV_CHANNEL_MAP` reader - Advertising channel map indicates the advertising channels used for advertising. By setting the bit, corresponding channel is enabled for use. Atleast one channel bit should be set. 7 - enable channel 39. 6 - enable channel 38. 5 - enable channel 37."]
pub struct ADV_CHANNEL_MAP_R(crate::FieldReader<u8, u8>);
impl ADV_CHANNEL_MAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADV_CHANNEL_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_CHANNEL_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_CHANNEL_MAP` writer - Advertising channel map indicates the advertising channels used for advertising. By setting the bit, corresponding channel is enabled for use. Atleast one channel bit should be set. 7 - enable channel 39. 6 - enable channel 38. 5 - enable channel 37."]
pub struct ADV_CHANNEL_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CHANNEL_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `RX_ADDR` reader - Peer addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. 1 - Rx addr type is random. 0 - Rx addr type is public"]
pub struct RX_ADDR_R(crate::FieldReader<bool, bool>);
impl RX_ADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ADDR` writer - Peer addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. 1 - Rx addr type is random. 0 - Rx addr type is public"]
pub struct RX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADDR_W<'a> {
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
#[doc = "Field `RX_SEC_ADDR` reader - Peer secondary addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. This address type corresponds to the PEER_SERC_ADDR register. Valid only if PRIV_1_2_ADV is set. 1 - Rx secondary addr type is random. 0 - Rx secondary addr type is public"]
pub struct RX_SEC_ADDR_R(crate::FieldReader<bool, bool>);
impl RX_SEC_ADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_SEC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SEC_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SEC_ADDR` writer - Peer secondary addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. This address type corresponds to the PEER_SERC_ADDR register. Valid only if PRIV_1_2_ADV is set. 1 - Rx secondary addr type is random. 0 - Rx secondary addr type is public"]
pub struct RX_SEC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SEC_ADDR_W<'a> {
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
#[doc = "Field `ADV_LOW_DUTY_CYCLE` reader - This bit field is used to specify to the Controller the Low Duty Cycle connectable directed advertising variant being used. 1 - Low Duty Cycle Connectable Directed Advertising. 0 - High Duty Cycle Connectable Directed Advertising."]
pub struct ADV_LOW_DUTY_CYCLE_R(crate::FieldReader<bool, bool>);
impl ADV_LOW_DUTY_CYCLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_LOW_DUTY_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_LOW_DUTY_CYCLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_LOW_DUTY_CYCLE` writer - This bit field is used to specify to the Controller the Low Duty Cycle connectable directed advertising variant being used. 1 - Low Duty Cycle Connectable Directed Advertising. 0 - High Duty Cycle Connectable Directed Advertising."]
pub struct ADV_LOW_DUTY_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_LOW_DUTY_CYCLE_W<'a> {
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
#[doc = "Field `INITA_RPA_CHECK` reader - This bit field is used to specify the Advertiser behavior on receiving the same INITA in the connect_req as in the ADV_DIRECT_IND packet it sent. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 0 - Accept the connect_req packet 1 - Reject the connect_req packet"]
pub struct INITA_RPA_CHECK_R(crate::FieldReader<bool, bool>);
impl INITA_RPA_CHECK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITA_RPA_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITA_RPA_CHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITA_RPA_CHECK` writer - This bit field is used to specify the Advertiser behavior on receiving the same INITA in the connect_req as in the ADV_DIRECT_IND packet it sent. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 0 - Accept the connect_req packet 1 - Reject the connect_req packet"]
pub struct INITA_RPA_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_RPA_CHECK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TX_ADDR_PRIV` reader - Device own address type subtype when Address type is random. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Random Address type is private. 0 - Random Address type is static."]
pub struct TX_ADDR_PRIV_R(crate::FieldReader<bool, bool>);
impl TX_ADDR_PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ADDR_PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ADDR_PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ADDR_PRIV` writer - Device own address type subtype when Address type is random. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Random Address type is private. 0 - Random Address type is static."]
pub struct TX_ADDR_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ADDR_PRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ADV_RCV_IA_IN_PRIV` reader - Advertiser behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub struct ADV_RCV_IA_IN_PRIV_R(crate::FieldReader<bool, bool>);
impl ADV_RCV_IA_IN_PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_RCV_IA_IN_PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_RCV_IA_IN_PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_RCV_IA_IN_PRIV` writer - Advertiser behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub struct ADV_RCV_IA_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RCV_IA_IN_PRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ADV_RPT_PEER_NRPA_ADDR_IN_PRIV` reader - Advertiser behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. This is applicable when whitelist is disabled. 1 - Only report the packets with peer NRPA address in privacy mode 0 - Respond to packets with peer NRPA address in privacy mode"]
pub struct ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R(crate::FieldReader<bool, bool>);
impl ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_RPT_PEER_NRPA_ADDR_IN_PRIV` writer - Advertiser behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. This is applicable when whitelist is disabled. 1 - Only report the packets with peer NRPA address in privacy mode 0 - Respond to packets with peer NRPA address in privacy mode"]
pub struct ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
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
#[doc = "Field `RCV_TX_ADDR` reader - Transmit address field of the received packet extracted from the receive packet. This field is used by firmware to report peer_addr_type parameter in the connection complete event."]
pub struct RCV_TX_ADDR_R(crate::FieldReader<bool, bool>);
impl RCV_TX_ADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCV_TX_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_TX_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Device own address type. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - The Advertising type is used to determine the packet type that is used for advertising when advertising is enabled. 0x0 - Connectable undirected advertising. (adv_ind) 0x1 - Connectable directed advertising (adv_direct_ind). 0x2 - Discoverable undirected advertising (adv_discover_ind) 0x3 - Non connectable undirected advertising (adv_nonconn_ind)."]
    #[inline(always)]
    pub fn adv_type(&self) -> ADV_TYPE_R {
        ADV_TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Advertising filter policy. The set of devices that the advertising procedure uses for device filtering is called the White List. 0x0 - Allow scan request from any device, allow connect request from any device. 0x1 - Allow scan request from devices in white list only, allow connect request from any device. 0x2 - Allow scan request from any device, allow connect request from devices in white list only. 0x3 - Allow scan request from devices in white list only, allow connect request from devices in white list only."]
    #[inline(always)]
    pub fn adv_filt_policy(&self) -> ADV_FILT_POLICY_R {
        ADV_FILT_POLICY_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Advertising channel map indicates the advertising channels used for advertising. By setting the bit, corresponding channel is enabled for use. Atleast one channel bit should be set. 7 - enable channel 39. 6 - enable channel 38. 5 - enable channel 37."]
    #[inline(always)]
    pub fn adv_channel_map(&self) -> ADV_CHANNEL_MAP_R {
        ADV_CHANNEL_MAP_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Peer addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. 1 - Rx addr type is random. 0 - Rx addr type is public"]
    #[inline(always)]
    pub fn rx_addr(&self) -> RX_ADDR_R {
        RX_ADDR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peer secondary addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. This address type corresponds to the PEER_SERC_ADDR register. Valid only if PRIV_1_2_ADV is set. 1 - Rx secondary addr type is random. 0 - Rx secondary addr type is public"]
    #[inline(always)]
    pub fn rx_sec_addr(&self) -> RX_SEC_ADDR_R {
        RX_SEC_ADDR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit field is used to specify to the Controller the Low Duty Cycle connectable directed advertising variant being used. 1 - Low Duty Cycle Connectable Directed Advertising. 0 - High Duty Cycle Connectable Directed Advertising."]
    #[inline(always)]
    pub fn adv_low_duty_cycle(&self) -> ADV_LOW_DUTY_CYCLE_R {
        ADV_LOW_DUTY_CYCLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit field is used to specify the Advertiser behavior on receiving the same INITA in the connect_req as in the ADV_DIRECT_IND packet it sent. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 0 - Accept the connect_req packet 1 - Reject the connect_req packet"]
    #[inline(always)]
    pub fn inita_rpa_check(&self) -> INITA_RPA_CHECK_R {
        INITA_RPA_CHECK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Device own address type subtype when Address type is random. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Random Address type is private. 0 - Random Address type is static."]
    #[inline(always)]
    pub fn tx_addr_priv(&self) -> TX_ADDR_PRIV_R {
        TX_ADDR_PRIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Advertiser behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn adv_rcv_ia_in_priv(&self) -> ADV_RCV_IA_IN_PRIV_R {
        ADV_RCV_IA_IN_PRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Advertiser behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. This is applicable when whitelist is disabled. 1 - Only report the packets with peer NRPA address in privacy mode 0 - Respond to packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn adv_rpt_peer_nrpa_addr_in_priv(&self) -> ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
        ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmit address field of the received packet extracted from the receive packet. This field is used by firmware to report peer_addr_type parameter in the connection complete event."]
    #[inline(always)]
    pub fn rcv_tx_addr(&self) -> RCV_TX_ADDR_R {
        RCV_TX_ADDR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device own address type. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W {
        TX_ADDR_W { w: self }
    }
    #[doc = "Bits 1:2 - The Advertising type is used to determine the packet type that is used for advertising when advertising is enabled. 0x0 - Connectable undirected advertising. (adv_ind) 0x1 - Connectable directed advertising (adv_direct_ind). 0x2 - Discoverable undirected advertising (adv_discover_ind) 0x3 - Non connectable undirected advertising (adv_nonconn_ind)."]
    #[inline(always)]
    pub fn adv_type(&mut self) -> ADV_TYPE_W {
        ADV_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4 - Advertising filter policy. The set of devices that the advertising procedure uses for device filtering is called the White List. 0x0 - Allow scan request from any device, allow connect request from any device. 0x1 - Allow scan request from devices in white list only, allow connect request from any device. 0x2 - Allow scan request from any device, allow connect request from devices in white list only. 0x3 - Allow scan request from devices in white list only, allow connect request from devices in white list only."]
    #[inline(always)]
    pub fn adv_filt_policy(&mut self) -> ADV_FILT_POLICY_W {
        ADV_FILT_POLICY_W { w: self }
    }
    #[doc = "Bits 5:7 - Advertising channel map indicates the advertising channels used for advertising. By setting the bit, corresponding channel is enabled for use. Atleast one channel bit should be set. 7 - enable channel 39. 6 - enable channel 38. 5 - enable channel 37."]
    #[inline(always)]
    pub fn adv_channel_map(&mut self) -> ADV_CHANNEL_MAP_W {
        ADV_CHANNEL_MAP_W { w: self }
    }
    #[doc = "Bit 8 - Peer addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. 1 - Rx addr type is random. 0 - Rx addr type is public"]
    #[inline(always)]
    pub fn rx_addr(&mut self) -> RX_ADDR_W {
        RX_ADDR_W { w: self }
    }
    #[doc = "Bit 9 - Peer secondary addresses type. This is the Direct_Address_type field programmed, only if ADV_DIRECT_IND type is sent. This address type corresponds to the PEER_SERC_ADDR register. Valid only if PRIV_1_2_ADV is set. 1 - Rx secondary addr type is random. 0 - Rx secondary addr type is public"]
    #[inline(always)]
    pub fn rx_sec_addr(&mut self) -> RX_SEC_ADDR_W {
        RX_SEC_ADDR_W { w: self }
    }
    #[doc = "Bit 10 - This bit field is used to specify to the Controller the Low Duty Cycle connectable directed advertising variant being used. 1 - Low Duty Cycle Connectable Directed Advertising. 0 - High Duty Cycle Connectable Directed Advertising."]
    #[inline(always)]
    pub fn adv_low_duty_cycle(&mut self) -> ADV_LOW_DUTY_CYCLE_W {
        ADV_LOW_DUTY_CYCLE_W { w: self }
    }
    #[doc = "Bit 11 - This bit field is used to specify the Advertiser behavior on receiving the same INITA in the connect_req as in the ADV_DIRECT_IND packet it sent. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 0 - Accept the connect_req packet 1 - Reject the connect_req packet"]
    #[inline(always)]
    pub fn inita_rpa_check(&mut self) -> INITA_RPA_CHECK_W {
        INITA_RPA_CHECK_W { w: self }
    }
    #[doc = "Bit 12 - Device own address type subtype when Address type is random. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Random Address type is private. 0 - Random Address type is static."]
    #[inline(always)]
    pub fn tx_addr_priv(&mut self) -> TX_ADDR_PRIV_W {
        TX_ADDR_PRIV_W { w: self }
    }
    #[doc = "Bit 13 - Advertiser behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn adv_rcv_ia_in_priv(&mut self) -> ADV_RCV_IA_IN_PRIV_W {
        ADV_RCV_IA_IN_PRIV_W { w: self }
    }
    #[doc = "Bit 14 - Advertiser behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set. This is applicable when whitelist is disabled. 1 - Only report the packets with peer NRPA address in privacy mode 0 - Respond to packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn adv_rpt_peer_nrpa_addr_in_priv(&mut self) -> ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_W {
        ADV_RPT_PEER_NRPA_ADDR_IN_PRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertising parameters register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_params](index.html) module"]
pub struct ADV_PARAMS_SPEC;
impl crate::RegisterSpec for ADV_PARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_params::R](R) reader structure"]
impl crate::Readable for ADV_PARAMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_params::W](W) writer structure"]
impl crate::Writable for ADV_PARAMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_PARAMS to value 0xe0"]
impl crate::Resettable for ADV_PARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe0
    }
}
