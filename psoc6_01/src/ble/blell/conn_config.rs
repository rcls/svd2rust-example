#[doc = "Register `CONN_CONFIG` reader"]
pub struct R(crate::R<CONN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CONFIG` writer"]
pub struct W(crate::W<CONN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CONFIG_SPEC>;
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
impl From<crate::W<CONN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_PKT_LIMIT` reader - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
pub struct RX_PKT_LIMIT_R(crate::FieldReader<u8, u8>);
impl RX_PKT_LIMIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PKT_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PKT_LIMIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PKT_LIMIT` writer - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
pub struct RX_PKT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PKT_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RX_INTR_THRESHOLD` reader - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
pub struct RX_INTR_THRESHOLD_R(crate::FieldReader<u8, u8>);
impl RX_INTR_THRESHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_INTR_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_INTR_THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_INTR_THRESHOLD` writer - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
pub struct RX_INTR_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INTR_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `MD_BIT_CLEAR` reader - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub struct MD_BIT_CLEAR_R(crate::FieldReader<bool, bool>);
impl MD_BIT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MD_BIT_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_BIT_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD_BIT_CLEAR` writer - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub struct MD_BIT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_BIT_CLEAR_W<'a> {
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
#[doc = "Field `DSM_SLOT_VARIANCE` reader - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
pub struct DSM_SLOT_VARIANCE_R(crate::FieldReader<bool, bool>);
impl DSM_SLOT_VARIANCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSM_SLOT_VARIANCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_SLOT_VARIANCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSM_SLOT_VARIANCE` writer - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
pub struct DSM_SLOT_VARIANCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_SLOT_VARIANCE_W<'a> {
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
#[doc = "Field `SLV_MD_CONFIG` reader - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
pub struct SLV_MD_CONFIG_R(crate::FieldReader<bool, bool>);
impl SLV_MD_CONFIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_MD_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_MD_CONFIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_MD_CONFIG` writer - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
pub struct SLV_MD_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_MD_CONFIG_W<'a> {
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
#[doc = "Field `EXTEND_CU_TX_WIN` reader - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
pub struct EXTEND_CU_TX_WIN_R(crate::FieldReader<bool, bool>);
impl EXTEND_CU_TX_WIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEND_CU_TX_WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEND_CU_TX_WIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEND_CU_TX_WIN` writer - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
pub struct EXTEND_CU_TX_WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEND_CU_TX_WIN_W<'a> {
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
#[doc = "Field `MASK_SUTO_AT_UPDT` reader - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
pub struct MASK_SUTO_AT_UPDT_R(crate::FieldReader<bool, bool>);
impl MASK_SUTO_AT_UPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK_SUTO_AT_UPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_SUTO_AT_UPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_SUTO_AT_UPDT` writer - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
pub struct MASK_SUTO_AT_UPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_SUTO_AT_UPDT_W<'a> {
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
#[doc = "Field `CONN_REQ_1SLOT_EARLY` reader - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
pub struct CONN_REQ_1SLOT_EARLY_R(crate::FieldReader<bool, bool>);
impl CONN_REQ_1SLOT_EARLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONN_REQ_1SLOT_EARLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_REQ_1SLOT_EARLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_REQ_1SLOT_EARLY` writer - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
pub struct CONN_REQ_1SLOT_EARLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_1SLOT_EARLY_W<'a> {
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
    #[doc = "Bits 0:3 - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
    #[inline(always)]
    pub fn rx_pkt_limit(&self) -> RX_PKT_LIMIT_R {
        RX_PKT_LIMIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
    #[inline(always)]
    pub fn rx_intr_threshold(&self) -> RX_INTR_THRESHOLD_R {
        RX_INTR_THRESHOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&self) -> MD_BIT_CLEAR_R {
        MD_BIT_CLEAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&self) -> DSM_SLOT_VARIANCE_R {
        DSM_SLOT_VARIANCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&self) -> SLV_MD_CONFIG_R {
        SLV_MD_CONFIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&self) -> EXTEND_CU_TX_WIN_R {
        EXTEND_CU_TX_WIN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&self) -> MASK_SUTO_AT_UPDT_R {
        MASK_SUTO_AT_UPDT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&self) -> CONN_REQ_1SLOT_EARLY_R {
        CONN_REQ_1SLOT_EARLY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
    #[inline(always)]
    pub fn rx_pkt_limit(&mut self) -> RX_PKT_LIMIT_W {
        RX_PKT_LIMIT_W { w: self }
    }
    #[doc = "Bits 4:7 - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
    #[inline(always)]
    pub fn rx_intr_threshold(&mut self) -> RX_INTR_THRESHOLD_W {
        RX_INTR_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&mut self) -> MD_BIT_CLEAR_W {
        MD_BIT_CLEAR_W { w: self }
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&mut self) -> DSM_SLOT_VARIANCE_W {
        DSM_SLOT_VARIANCE_W { w: self }
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&mut self) -> SLV_MD_CONFIG_W {
        SLV_MD_CONFIG_W { w: self }
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&mut self) -> EXTEND_CU_TX_WIN_W {
        EXTEND_CU_TX_WIN_W { w: self }
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&mut self) -> MASK_SUTO_AT_UPDT_W {
        MASK_SUTO_AT_UPDT_W { w: self }
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&mut self) -> CONN_REQ_1SLOT_EARLY_W {
        CONN_REQ_1SLOT_EARLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_config](index.html) module"]
pub struct CONN_CONFIG_SPEC;
impl crate::RegisterSpec for CONN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_config::R](R) reader structure"]
impl crate::Readable for CONN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_config::W](W) writer structure"]
impl crate::Writable for CONN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CONFIG to value 0xe11f"]
impl crate::Resettable for CONN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe11f
    }
}
