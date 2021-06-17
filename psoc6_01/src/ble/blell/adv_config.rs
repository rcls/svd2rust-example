#[doc = "Register `ADV_CONFIG` reader"]
pub struct R(crate::R<ADV_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_CONFIG` writer"]
pub struct W(crate::W<ADV_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_CONFIG_SPEC>;
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
impl From<crate::W<ADV_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_STRT_EN` reader - Enable advertising event start interrupt."]
pub struct ADV_STRT_EN_R(crate::FieldReader<bool, bool>);
impl ADV_STRT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_STRT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_STRT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_STRT_EN` writer - Enable advertising event start interrupt."]
pub struct ADV_STRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_STRT_EN_W<'a> {
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
#[doc = "Field `ADV_CLS_EN` reader - Enable advertising event stop interrupt."]
pub struct ADV_CLS_EN_R(crate::FieldReader<bool, bool>);
impl ADV_CLS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_CLS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_CLS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_CLS_EN` writer - Enable advertising event stop interrupt."]
pub struct ADV_CLS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CLS_EN_W<'a> {
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
#[doc = "Field `ADV_TX_EN` reader - Enable adv packet transmitted interrupt."]
pub struct ADV_TX_EN_R(crate::FieldReader<bool, bool>);
impl ADV_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_TX_EN` writer - Enable adv packet transmitted interrupt."]
pub struct ADV_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TX_EN_W<'a> {
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
#[doc = "Field `SCN_RSP_TX_EN` reader - Enable scan response packet transmitted interrupt."]
pub struct SCN_RSP_TX_EN_R(crate::FieldReader<bool, bool>);
impl SCN_RSP_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCN_RSP_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCN_RSP_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCN_RSP_TX_EN` writer - Enable scan response packet transmitted interrupt."]
pub struct SCN_RSP_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_RSP_TX_EN_W<'a> {
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
#[doc = "Field `ADV_SCN_REQ_RX_EN` reader - Enable scan request packet received interrupt."]
pub struct ADV_SCN_REQ_RX_EN_R(crate::FieldReader<bool, bool>);
impl ADV_SCN_REQ_RX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_SCN_REQ_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_SCN_REQ_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_SCN_REQ_RX_EN` writer - Enable scan request packet received interrupt."]
pub struct ADV_SCN_REQ_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_SCN_REQ_RX_EN_W<'a> {
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
#[doc = "Field `ADV_CONN_REQ_RX_EN` reader - Enable connect request packet received interrupt."]
pub struct ADV_CONN_REQ_RX_EN_R(crate::FieldReader<bool, bool>);
impl ADV_CONN_REQ_RX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_CONN_REQ_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_CONN_REQ_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_CONN_REQ_RX_EN` writer - Enable connect request packet received interrupt."]
pub struct ADV_CONN_REQ_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CONN_REQ_RX_EN_W<'a> {
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
#[doc = "Field `SLV_CONNECTED_EN` reader - Enable slave connected interrupt."]
pub struct SLV_CONNECTED_EN_R(crate::FieldReader<bool, bool>);
impl SLV_CONNECTED_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CONNECTED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CONNECTED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CONNECTED_EN` writer - Enable slave connected interrupt."]
pub struct SLV_CONNECTED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CONNECTED_EN_W<'a> {
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
#[doc = "Field `ADV_TIMEOUT_EN` reader - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
pub struct ADV_TIMEOUT_EN_R(crate::FieldReader<bool, bool>);
impl ADV_TIMEOUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_TIMEOUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_TIMEOUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_TIMEOUT_EN` writer - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
pub struct ADV_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TIMEOUT_EN_W<'a> {
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
#[doc = "Field `ADV_RAND_DISABLE` reader - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
pub struct ADV_RAND_DISABLE_R(crate::FieldReader<bool, bool>);
impl ADV_RAND_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_RAND_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_RAND_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_RAND_DISABLE` writer - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
pub struct ADV_RAND_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RAND_DISABLE_W<'a> {
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
#[doc = "Field `ADV_SCN_PEER_RPA_UNMCH_EN` reader - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
pub struct ADV_SCN_PEER_RPA_UNMCH_EN_R(crate::FieldReader<bool, bool>);
impl ADV_SCN_PEER_RPA_UNMCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_SCN_PEER_RPA_UNMCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_SCN_PEER_RPA_UNMCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_SCN_PEER_RPA_UNMCH_EN` writer - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
pub struct ADV_SCN_PEER_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_SCN_PEER_RPA_UNMCH_EN_W<'a> {
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
#[doc = "Field `ADV_CONN_PEER_RPA_UNMCH_EN` reader - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub struct ADV_CONN_PEER_RPA_UNMCH_EN_R(crate::FieldReader<bool, bool>);
impl ADV_CONN_PEER_RPA_UNMCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_CONN_PEER_RPA_UNMCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_CONN_PEER_RPA_UNMCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_CONN_PEER_RPA_UNMCH_EN` writer - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub struct ADV_CONN_PEER_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CONN_PEER_RPA_UNMCH_EN_W<'a> {
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
#[doc = "Field `ADV_PKT_INTERVAL` reader - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
pub struct ADV_PKT_INTERVAL_R(crate::FieldReader<u8, u8>);
impl ADV_PKT_INTERVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADV_PKT_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_PKT_INTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_PKT_INTERVAL` writer - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
pub struct ADV_PKT_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_PKT_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable advertising event start interrupt."]
    #[inline(always)]
    pub fn adv_strt_en(&self) -> ADV_STRT_EN_R {
        ADV_STRT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&self) -> ADV_CLS_EN_R {
        ADV_CLS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&self) -> ADV_TX_EN_R {
        ADV_TX_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&self) -> SCN_RSP_TX_EN_R {
        SCN_RSP_TX_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&self) -> ADV_SCN_REQ_RX_EN_R {
        ADV_SCN_REQ_RX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&self) -> ADV_CONN_REQ_RX_EN_R {
        ADV_CONN_REQ_RX_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&self) -> SLV_CONNECTED_EN_R {
        SLV_CONNECTED_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&self) -> ADV_TIMEOUT_EN_R {
        ADV_TIMEOUT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&self) -> ADV_RAND_DISABLE_R {
        ADV_RAND_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&self) -> ADV_SCN_PEER_RPA_UNMCH_EN_R {
        ADV_SCN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&self) -> ADV_CONN_PEER_RPA_UNMCH_EN_R {
        ADV_CONN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
    #[inline(always)]
    pub fn adv_pkt_interval(&self) -> ADV_PKT_INTERVAL_R {
        ADV_PKT_INTERVAL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable advertising event start interrupt."]
    #[inline(always)]
    pub fn adv_strt_en(&mut self) -> ADV_STRT_EN_W {
        ADV_STRT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&mut self) -> ADV_CLS_EN_W {
        ADV_CLS_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&mut self) -> ADV_TX_EN_W {
        ADV_TX_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&mut self) -> SCN_RSP_TX_EN_W {
        SCN_RSP_TX_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&mut self) -> ADV_SCN_REQ_RX_EN_W {
        ADV_SCN_REQ_RX_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&mut self) -> ADV_CONN_REQ_RX_EN_W {
        ADV_CONN_REQ_RX_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&mut self) -> SLV_CONNECTED_EN_W {
        SLV_CONNECTED_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&mut self) -> ADV_TIMEOUT_EN_W {
        ADV_TIMEOUT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&mut self) -> ADV_RAND_DISABLE_W {
        ADV_RAND_DISABLE_W { w: self }
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&mut self) -> ADV_SCN_PEER_RPA_UNMCH_EN_W {
        ADV_SCN_PEER_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&mut self) -> ADV_CONN_PEER_RPA_UNMCH_EN_W {
        ADV_CONN_PEER_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bits 11:15 - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
    #[inline(always)]
    pub fn adv_pkt_interval(&mut self) -> ADV_PKT_INTERVAL_W {
        ADV_PKT_INTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertiser configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_config](index.html) module"]
pub struct ADV_CONFIG_SPEC;
impl crate::RegisterSpec for ADV_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_config::R](R) reader structure"]
impl crate::Readable for ADV_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_config::W](W) writer structure"]
impl crate::Writable for ADV_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_CONFIG to value 0x20ff"]
impl crate::Resettable for ADV_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20ff
    }
}
