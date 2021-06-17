#[doc = "Register `EVENT_ENABLE` reader"]
pub struct R(crate::R<EVENT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENT_ENABLE` writer"]
pub struct W(crate::W<EVENT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_ENABLE_SPEC>;
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
impl From<crate::W<EVENT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_INT_EN` reader - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
pub struct ADV_INT_EN_R(crate::FieldReader<bool, bool>);
impl ADV_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_INT_EN` writer - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
pub struct ADV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_INT_EN_W<'a> {
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
#[doc = "Field `SCN_INT_EN` reader - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
pub struct SCN_INT_EN_R(crate::FieldReader<bool, bool>);
impl SCN_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCN_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCN_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCN_INT_EN` writer - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
pub struct SCN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_INT_EN_W<'a> {
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
#[doc = "Field `INIT_INT_EN` reader - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
pub struct INIT_INT_EN_R(crate::FieldReader<bool, bool>);
impl INIT_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_INT_EN` writer - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
pub struct INIT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_INT_EN_W<'a> {
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
#[doc = "Field `CONN_INT_EN` reader - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
pub struct CONN_INT_EN_R(crate::FieldReader<bool, bool>);
impl CONN_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONN_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_INT_EN` writer - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
pub struct CONN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_INT_EN_W<'a> {
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
#[doc = "Field `SM_INT_EN` reader - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
pub struct SM_INT_EN_R(crate::FieldReader<bool, bool>);
impl SM_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SM_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_INT_EN` writer - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
pub struct SM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_INT_EN_W<'a> {
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
#[doc = "Field `DSM_INT_EN` reader - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
pub struct DSM_INT_EN_R(crate::FieldReader<bool, bool>);
impl DSM_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSM_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSM_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSM_INT_EN` writer - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
pub struct DSM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_INT_EN_W<'a> {
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
#[doc = "Field `ENC_INT_EN` reader - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
pub struct ENC_INT_EN_R(crate::FieldReader<bool, bool>);
impl ENC_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC_INT_EN` writer - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
pub struct ENC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_INT_EN_W<'a> {
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
#[doc = "Field `RSSI_RX_DONE_INT_EN` reader - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
pub struct RSSI_RX_DONE_INT_EN_R(crate::FieldReader<bool, bool>);
impl RSSI_RX_DONE_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_RX_DONE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_RX_DONE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_RX_DONE_INT_EN` writer - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
pub struct RSSI_RX_DONE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_RX_DONE_INT_EN_W<'a> {
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
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&self) -> ADV_INT_EN_R {
        ADV_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&self) -> SCN_INT_EN_R {
        SCN_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&self) -> INIT_INT_EN_R {
        INIT_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&self) -> CONN_INT_EN_R {
        CONN_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&self) -> SM_INT_EN_R {
        SM_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&self) -> DSM_INT_EN_R {
        DSM_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&self) -> ENC_INT_EN_R {
        ENC_INT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&self) -> RSSI_RX_DONE_INT_EN_R {
        RSSI_RX_DONE_INT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&mut self) -> ADV_INT_EN_W {
        ADV_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&mut self) -> SCN_INT_EN_W {
        SCN_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&mut self) -> INIT_INT_EN_W {
        INIT_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&mut self) -> CONN_INT_EN_W {
        CONN_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&mut self) -> SM_INT_EN_W {
        SM_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&mut self) -> DSM_INT_EN_W {
        DSM_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&mut self) -> ENC_INT_EN_W {
        ENC_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&mut self) -> RSSI_RX_DONE_INT_EN_W {
        RSSI_RX_DONE_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event indications enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_enable](index.html) module"]
pub struct EVENT_ENABLE_SPEC;
impl crate::RegisterSpec for EVENT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event_enable::R](R) reader structure"]
impl crate::Readable for EVENT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [event_enable::W](W) writer structure"]
impl crate::Writable for EVENT_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENT_ENABLE to value 0"]
impl crate::Resettable for EVENT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
