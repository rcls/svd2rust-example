#[doc = "Register `SCAN_PARAM` reader"]
pub struct R(crate::R<SCAN_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_PARAM` writer"]
pub struct W(crate::W<SCAN_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_PARAM_SPEC>;
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
impl From<crate::W<SCAN_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ADDR` reader - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
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
#[doc = "Field `TX_ADDR` writer - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
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
#[doc = "Field `SCAN_TYPE` reader - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
pub struct SCAN_TYPE_R(crate::FieldReader<u8, u8>);
impl SCAN_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCAN_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_TYPE` writer - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
pub struct SCAN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `SCAN_FILT_POLICY` reader - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
pub struct SCAN_FILT_POLICY_R(crate::FieldReader<u8, u8>);
impl SCAN_FILT_POLICY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCAN_FILT_POLICY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_FILT_POLICY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_FILT_POLICY` writer - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
pub struct SCAN_FILT_POLICY_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_FILT_POLICY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DUP_FILT_EN` reader - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
pub struct DUP_FILT_EN_R(crate::FieldReader<bool, bool>);
impl DUP_FILT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUP_FILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUP_FILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUP_FILT_EN` writer - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
pub struct DUP_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUP_FILT_EN_W<'a> {
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
#[doc = "Field `DUP_FILT_CHK_ADV_DIR` reader - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
pub struct DUP_FILT_CHK_ADV_DIR_R(crate::FieldReader<bool, bool>);
impl DUP_FILT_CHK_ADV_DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUP_FILT_CHK_ADV_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUP_FILT_CHK_ADV_DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUP_FILT_CHK_ADV_DIR` writer - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
pub struct DUP_FILT_CHK_ADV_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUP_FILT_CHK_ADV_DIR_W<'a> {
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
#[doc = "Field `SCAN_RSP_ADVA_CHECK` reader - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
pub struct SCAN_RSP_ADVA_CHECK_R(crate::FieldReader<bool, bool>);
impl SCAN_RSP_ADVA_CHECK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_RSP_ADVA_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_RSP_ADVA_CHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_RSP_ADVA_CHECK` writer - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
pub struct SCAN_RSP_ADVA_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RSP_ADVA_CHECK_W<'a> {
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
#[doc = "Field `SCAN_RCV_IA_IN_PRIV` reader - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub struct SCAN_RCV_IA_IN_PRIV_R(crate::FieldReader<bool, bool>);
impl SCAN_RCV_IA_IN_PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_RCV_IA_IN_PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_RCV_IA_IN_PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_RCV_IA_IN_PRIV` writer - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub struct SCAN_RCV_IA_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RCV_IA_IN_PRIV_W<'a> {
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
#[doc = "Field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV` reader - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
pub struct SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R(crate::FieldReader<bool, bool>);
impl SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV` writer - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
pub struct SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
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
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&self) -> SCAN_TYPE_R {
        SCAN_TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&self) -> SCAN_FILT_POLICY_R {
        SCAN_FILT_POLICY_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&self) -> DUP_FILT_EN_R {
        DUP_FILT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&self) -> DUP_FILT_CHK_ADV_DIR_R {
        DUP_FILT_CHK_ADV_DIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&self) -> SCAN_RSP_ADVA_CHECK_R {
        SCAN_RSP_ADVA_CHECK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&self) -> SCAN_RCV_IA_IN_PRIV_R {
        SCAN_RCV_IA_IN_PRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W {
        TX_ADDR_W { w: self }
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&mut self) -> SCAN_TYPE_W {
        SCAN_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&mut self) -> SCAN_FILT_POLICY_W {
        SCAN_FILT_POLICY_W { w: self }
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&mut self) -> DUP_FILT_EN_W {
        DUP_FILT_EN_W { w: self }
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&mut self) -> DUP_FILT_CHK_ADV_DIR_W {
        DUP_FILT_CHK_ADV_DIR_W { w: self }
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&mut self) -> SCAN_RSP_ADVA_CHECK_W {
        SCAN_RSP_ADVA_CHECK_W { w: self }
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&mut self) -> SCAN_RCV_IA_IN_PRIV_W {
        SCAN_RCV_IA_IN_PRIV_W { w: self }
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&mut self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scanning parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_param](index.html) module"]
pub struct SCAN_PARAM_SPEC;
impl crate::RegisterSpec for SCAN_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_param::R](R) reader structure"]
impl crate::Readable for SCAN_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_param::W](W) writer structure"]
impl crate::Writable for SCAN_PARAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_PARAM to value 0"]
impl crate::Resettable for SCAN_PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
