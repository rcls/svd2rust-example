#[doc = "Register `LL_CONFIG` reader"]
pub struct R(crate::R<LL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LL_CONFIG` writer"]
pub struct W(crate::W<LL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LL_CONFIG_SPEC>;
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
impl From<crate::W<LL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSI_SEL` reader - Controls the RSSI reads. When this bit is 1, the bit RSSI_INTR_SEL is don't care. 0 - RSSI read is initiated after the the packet is received 1 - RSSI read is completed before the packet is received. When RCB Interface is operating 4Mhz are lower this bit should be set to 1'b0."]
pub struct RSSI_SEL_R(crate::FieldReader<bool, bool>);
impl RSSI_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_SEL` writer - Controls the RSSI reads. When this bit is 1, the bit RSSI_INTR_SEL is don't care. 0 - RSSI read is initiated after the the packet is received 1 - RSSI read is completed before the packet is received. When RCB Interface is operating 4Mhz are lower this bit should be set to 1'b0."]
pub struct RSSI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_SEL_W<'a> {
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
#[doc = "Field `TX_RX_CTRL_SEL` reader - Controls the mode of issueing TX_EN & RX_EN to the Radio 1 - TX_EN and RX_EN are issued through direct pins 0 - TX_EN and RX_EN are issued through RCB writes"]
pub struct TX_RX_CTRL_SEL_R(crate::FieldReader<bool, bool>);
impl TX_RX_CTRL_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_RX_CTRL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RX_CTRL_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RX_CTRL_SEL` writer - Controls the mode of issueing TX_EN & RX_EN to the Radio 1 - TX_EN and RX_EN are issued through direct pins 0 - TX_EN and RX_EN are issued through RCB writes"]
pub struct TX_RX_CTRL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RX_CTRL_SEL_W<'a> {
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
#[doc = "Field `TIFS_ENABLE` reader - Setting this bit enables the tx 1MHz pulse to match the received bpktctl from CYBLERD55. This will result is reduced TIFS variation"]
pub struct TIFS_ENABLE_R(crate::FieldReader<bool, bool>);
impl TIFS_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIFS_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIFS_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIFS_ENABLE` writer - Setting this bit enables the tx 1MHz pulse to match the received bpktctl from CYBLERD55. This will result is reduced TIFS variation"]
pub struct TIFS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFS_ENABLE_W<'a> {
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
#[doc = "Field `TIMER_LF_SLOT_ENABLE` reader - Controls the wakeup timer configuration 1 - Wakeup time is compensated with the LF_OFFSET 0 - Wakeup time is not compensated with the LF_OFFSET as in legacy mode"]
pub struct TIMER_LF_SLOT_ENABLE_R(crate::FieldReader<bool, bool>);
impl TIMER_LF_SLOT_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_LF_SLOT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_LF_SLOT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_LF_SLOT_ENABLE` writer - Controls the wakeup timer configuration 1 - Wakeup time is compensated with the LF_OFFSET 0 - Wakeup time is not compensated with the LF_OFFSET as in legacy mode"]
pub struct TIMER_LF_SLOT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_LF_SLOT_ENABLE_W<'a> {
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
#[doc = "Field `RSSI_INTR_SEL` reader - Controls the engine interrupt generation based on RSSI reads. This is valid only if RSSI_SEL is 0. 0 - Receive interrupts are triggerred after the RSSI read is complete 1 - Receive interrupts are triggerred after the last bit of CRC"]
pub struct RSSI_INTR_SEL_R(crate::FieldReader<bool, bool>);
impl RSSI_INTR_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_INTR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_INTR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_INTR_SEL` writer - Controls the engine interrupt generation based on RSSI reads. This is valid only if RSSI_SEL is 0. 0 - Receive interrupts are triggerred after the RSSI read is complete 1 - Receive interrupts are triggerred after the last bit of CRC"]
pub struct RSSI_INTR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_INTR_SEL_W<'a> {
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
#[doc = "Field `RSSI_EARLY_CNFG` reader - Controls the early RSSI reads. This is applicable only when RSSI_SEL is 1. 1 - RSSI read is initiated during the first CRC byte reception. 0 - RSSI read is initiated during the third CRC byte reception."]
pub struct RSSI_EARLY_CNFG_R(crate::FieldReader<bool, bool>);
impl RSSI_EARLY_CNFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_EARLY_CNFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_EARLY_CNFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_EARLY_CNFG` writer - Controls the early RSSI reads. This is applicable only when RSSI_SEL is 1. 1 - RSSI read is initiated during the first CRC byte reception. 0 - RSSI read is initiated during the third CRC byte reception."]
pub struct RSSI_EARLY_CNFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_EARLY_CNFG_W<'a> {
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
#[doc = "Field `TX_RX_PIN_DLY` reader - Controls the delay from DBUS_TX, DBUS_RX assertion to the assertion on the pins. This is applicable only when TX_RX_CTRL_SEL is set. 0 - The pin assertion is delayed by 4 cycles. 1 - The pin assertion is delayed by 8 cycles."]
pub struct TX_RX_PIN_DLY_R(crate::FieldReader<bool, bool>);
impl TX_RX_PIN_DLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_RX_PIN_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RX_PIN_DLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RX_PIN_DLY` writer - Controls the delay from DBUS_TX, DBUS_RX assertion to the assertion on the pins. This is applicable only when TX_RX_CTRL_SEL is set. 0 - The pin assertion is delayed by 4 cycles. 1 - The pin assertion is delayed by 8 cycles."]
pub struct TX_RX_PIN_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RX_PIN_DLY_W<'a> {
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
#[doc = "Field `TX_PA_PWR_LVL_TYPE` reader - Controls the TX power level format given to the CYBLERD55 chip. 0 - The power level given to CYBLERD55 is in 4 bit code format from ADV_CH_TX_POWER for advertising channel and DTM packets & from CONN_CH_TX_POWER for connection channel packets. The power level setting is decoded and given to the PA. 1 - The power level given to CYBLERD55 is in 18 bit power level setting format from {ADV_CH_TX_POWER_LVL_MS, ADV_CH_TX_POWER_LVL_LS} channel and DTM packets & from {CONN_CH_TX_POWER_LVL_MS, CONN_CH_TX_POWER_LVL_LS} for connection channel packets. This setting is directly given to the PA."]
pub struct TX_PA_PWR_LVL_TYPE_R(crate::FieldReader<bool, bool>);
impl TX_PA_PWR_LVL_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_PA_PWR_LVL_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PA_PWR_LVL_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PA_PWR_LVL_TYPE` writer - Controls the TX power level format given to the CYBLERD55 chip. 0 - The power level given to CYBLERD55 is in 4 bit code format from ADV_CH_TX_POWER for advertising channel and DTM packets & from CONN_CH_TX_POWER for connection channel packets. The power level setting is decoded and given to the PA. 1 - The power level given to CYBLERD55 is in 18 bit power level setting format from {ADV_CH_TX_POWER_LVL_MS, ADV_CH_TX_POWER_LVL_LS} channel and DTM packets & from {CONN_CH_TX_POWER_LVL_MS, CONN_CH_TX_POWER_LVL_LS} for connection channel packets. This setting is directly given to the PA."]
pub struct TX_PA_PWR_LVL_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PA_PWR_LVL_TYPE_W<'a> {
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
#[doc = "Field `RSSI_ENERGY_RD` reader - Controls the RSSI reads. 0 - Channel Energy read is not initiated if no packet is received during a receive cycle 1 - Channel Energy read is initiated at the end of the receive cycle if no packet is received"]
pub struct RSSI_ENERGY_RD_R(crate::FieldReader<bool, bool>);
impl RSSI_ENERGY_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_ENERGY_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_ENERGY_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_ENERGY_RD` writer - Controls the RSSI reads. 0 - Channel Energy read is not initiated if no packet is received during a receive cycle 1 - Channel Energy read is initiated at the end of the receive cycle if no packet is received"]
pub struct RSSI_ENERGY_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_ENERGY_RD_W<'a> {
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
#[doc = "Field `RSSI_EACH_PKT` reader - Controls the RSSI reads. 0 - RSSI read is not initiated for zero length and aborted packets 1 - RSSI read is initiated for zero length and aborted packets"]
pub struct RSSI_EACH_PKT_R(crate::FieldReader<bool, bool>);
impl RSSI_EACH_PKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSI_EACH_PKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_EACH_PKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_EACH_PKT` writer - Controls the RSSI reads. 0 - RSSI read is not initiated for zero length and aborted packets 1 - RSSI read is initiated for zero length and aborted packets"]
pub struct RSSI_EACH_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_EACH_PKT_W<'a> {
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
#[doc = "Field `FORCE_TRIG_RCB_UPDATE` reader - Controls the RCB update to radio on TX/RX enable. Applicable only when TX_RX_CTRL_SEL is 1'b1 0 - RCB update is triggerred only when the fields change on rising edge of TX/RX enable 1 - RCB update is force triggerred on rising edge of TX/RX enable If TX_RX_CTRL_SEL is 1'b1 and ENABLE_RADIO_BOD is 1'b1, this bit needs to be set to 1'b1"]
pub struct FORCE_TRIG_RCB_UPDATE_R(crate::FieldReader<bool, bool>);
impl FORCE_TRIG_RCB_UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_TRIG_RCB_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_TRIG_RCB_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_TRIG_RCB_UPDATE` writer - Controls the RCB update to radio on TX/RX enable. Applicable only when TX_RX_CTRL_SEL is 1'b1 0 - RCB update is triggerred only when the fields change on rising edge of TX/RX enable 1 - RCB update is force triggerred on rising edge of TX/RX enable If TX_RX_CTRL_SEL is 1'b1 and ENABLE_RADIO_BOD is 1'b1, this bit needs to be set to 1'b1"]
pub struct FORCE_TRIG_RCB_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_TRIG_RCB_UPDATE_W<'a> {
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
#[doc = "Field `CHECK_DUP_CONN` reader - Controls the duplicate connection checkin ADV and INIT 0 - Does not check if the peer is already connection before a new connection is created 1 - Checks if the peer is already connection before a new connection is created and aborts a duplicate connection creation"]
pub struct CHECK_DUP_CONN_R(crate::FieldReader<bool, bool>);
impl CHECK_DUP_CONN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_DUP_CONN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_DUP_CONN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_DUP_CONN` writer - Controls the duplicate connection checkin ADV and INIT 0 - Does not check if the peer is already connection before a new connection is created 1 - Checks if the peer is already connection before a new connection is created and aborts a duplicate connection creation"]
pub struct CHECK_DUP_CONN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_DUP_CONN_W<'a> {
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
#[doc = "Field `MULTI_ENGINE_LPM` reader - Controls the LPM entry condition 0 - Legacy mode LPM entry check 1 - MMMS mode LPM entry check"]
pub struct MULTI_ENGINE_LPM_R(crate::FieldReader<bool, bool>);
impl MULTI_ENGINE_LPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MULTI_ENGINE_LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULTI_ENGINE_LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULTI_ENGINE_LPM` writer - Controls the LPM entry condition 0 - Legacy mode LPM entry check 1 - MMMS mode LPM entry check"]
pub struct MULTI_ENGINE_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_ENGINE_LPM_W<'a> {
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
#[doc = "Field `ADV_DIR_DEVICE_PRIV_EN` reader - Controls the ADV behavior while advertising ADV_DIR and only device privacy is set. When the ADV is transmitting INITA RPA, the bahavior when an Identity address in received from the Initiator in the CONN_REQ is given below 0 - Abort the CONN_REQ and continue with advertisement 1 - Check the address against PEER_SEC_ADDR_ADV and create connection on a match."]
pub struct ADV_DIR_DEVICE_PRIV_EN_R(crate::FieldReader<bool, bool>);
impl ADV_DIR_DEVICE_PRIV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_DIR_DEVICE_PRIV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_DIR_DEVICE_PRIV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_DIR_DEVICE_PRIV_EN` writer - Controls the ADV behavior while advertising ADV_DIR and only device privacy is set. When the ADV is transmitting INITA RPA, the bahavior when an Identity address in received from the Initiator in the CONN_REQ is given below 0 - Abort the CONN_REQ and continue with advertisement 1 - Check the address against PEER_SEC_ADDR_ADV and create connection on a match."]
pub struct ADV_DIR_DEVICE_PRIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_DIR_DEVICE_PRIV_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Controls the RSSI reads. When this bit is 1, the bit RSSI_INTR_SEL is don't care. 0 - RSSI read is initiated after the the packet is received 1 - RSSI read is completed before the packet is received. When RCB Interface is operating 4Mhz are lower this bit should be set to 1'b0."]
    #[inline(always)]
    pub fn rssi_sel(&self) -> RSSI_SEL_R {
        RSSI_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the mode of issueing TX_EN & RX_EN to the Radio 1 - TX_EN and RX_EN are issued through direct pins 0 - TX_EN and RX_EN are issued through RCB writes"]
    #[inline(always)]
    pub fn tx_rx_ctrl_sel(&self) -> TX_RX_CTRL_SEL_R {
        TX_RX_CTRL_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Setting this bit enables the tx 1MHz pulse to match the received bpktctl from CYBLERD55. This will result is reduced TIFS variation"]
    #[inline(always)]
    pub fn tifs_enable(&self) -> TIFS_ENABLE_R {
        TIFS_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the wakeup timer configuration 1 - Wakeup time is compensated with the LF_OFFSET 0 - Wakeup time is not compensated with the LF_OFFSET as in legacy mode"]
    #[inline(always)]
    pub fn timer_lf_slot_enable(&self) -> TIMER_LF_SLOT_ENABLE_R {
        TIMER_LF_SLOT_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls the engine interrupt generation based on RSSI reads. This is valid only if RSSI_SEL is 0. 0 - Receive interrupts are triggerred after the RSSI read is complete 1 - Receive interrupts are triggerred after the last bit of CRC"]
    #[inline(always)]
    pub fn rssi_intr_sel(&self) -> RSSI_INTR_SEL_R {
        RSSI_INTR_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls the early RSSI reads. This is applicable only when RSSI_SEL is 1. 1 - RSSI read is initiated during the first CRC byte reception. 0 - RSSI read is initiated during the third CRC byte reception."]
    #[inline(always)]
    pub fn rssi_early_cnfg(&self) -> RSSI_EARLY_CNFG_R {
        RSSI_EARLY_CNFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls the delay from DBUS_TX, DBUS_RX assertion to the assertion on the pins. This is applicable only when TX_RX_CTRL_SEL is set. 0 - The pin assertion is delayed by 4 cycles. 1 - The pin assertion is delayed by 8 cycles."]
    #[inline(always)]
    pub fn tx_rx_pin_dly(&self) -> TX_RX_PIN_DLY_R {
        TX_RX_PIN_DLY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls the TX power level format given to the CYBLERD55 chip. 0 - The power level given to CYBLERD55 is in 4 bit code format from ADV_CH_TX_POWER for advertising channel and DTM packets & from CONN_CH_TX_POWER for connection channel packets. The power level setting is decoded and given to the PA. 1 - The power level given to CYBLERD55 is in 18 bit power level setting format from {ADV_CH_TX_POWER_LVL_MS, ADV_CH_TX_POWER_LVL_LS} channel and DTM packets & from {CONN_CH_TX_POWER_LVL_MS, CONN_CH_TX_POWER_LVL_LS} for connection channel packets. This setting is directly given to the PA."]
    #[inline(always)]
    pub fn tx_pa_pwr_lvl_type(&self) -> TX_PA_PWR_LVL_TYPE_R {
        TX_PA_PWR_LVL_TYPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls the RSSI reads. 0 - Channel Energy read is not initiated if no packet is received during a receive cycle 1 - Channel Energy read is initiated at the end of the receive cycle if no packet is received"]
    #[inline(always)]
    pub fn rssi_energy_rd(&self) -> RSSI_ENERGY_RD_R {
        RSSI_ENERGY_RD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls the RSSI reads. 0 - RSSI read is not initiated for zero length and aborted packets 1 - RSSI read is initiated for zero length and aborted packets"]
    #[inline(always)]
    pub fn rssi_each_pkt(&self) -> RSSI_EACH_PKT_R {
        RSSI_EACH_PKT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls the RCB update to radio on TX/RX enable. Applicable only when TX_RX_CTRL_SEL is 1'b1 0 - RCB update is triggerred only when the fields change on rising edge of TX/RX enable 1 - RCB update is force triggerred on rising edge of TX/RX enable If TX_RX_CTRL_SEL is 1'b1 and ENABLE_RADIO_BOD is 1'b1, this bit needs to be set to 1'b1"]
    #[inline(always)]
    pub fn force_trig_rcb_update(&self) -> FORCE_TRIG_RCB_UPDATE_R {
        FORCE_TRIG_RCB_UPDATE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls the duplicate connection checkin ADV and INIT 0 - Does not check if the peer is already connection before a new connection is created 1 - Checks if the peer is already connection before a new connection is created and aborts a duplicate connection creation"]
    #[inline(always)]
    pub fn check_dup_conn(&self) -> CHECK_DUP_CONN_R {
        CHECK_DUP_CONN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls the LPM entry condition 0 - Legacy mode LPM entry check 1 - MMMS mode LPM entry check"]
    #[inline(always)]
    pub fn multi_engine_lpm(&self) -> MULTI_ENGINE_LPM_R {
        MULTI_ENGINE_LPM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls the ADV behavior while advertising ADV_DIR and only device privacy is set. When the ADV is transmitting INITA RPA, the bahavior when an Identity address in received from the Initiator in the CONN_REQ is given below 0 - Abort the CONN_REQ and continue with advertisement 1 - Check the address against PEER_SEC_ADDR_ADV and create connection on a match."]
    #[inline(always)]
    pub fn adv_dir_device_priv_en(&self) -> ADV_DIR_DEVICE_PRIV_EN_R {
        ADV_DIR_DEVICE_PRIV_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the RSSI reads. When this bit is 1, the bit RSSI_INTR_SEL is don't care. 0 - RSSI read is initiated after the the packet is received 1 - RSSI read is completed before the packet is received. When RCB Interface is operating 4Mhz are lower this bit should be set to 1'b0."]
    #[inline(always)]
    pub fn rssi_sel(&mut self) -> RSSI_SEL_W {
        RSSI_SEL_W { w: self }
    }
    #[doc = "Bit 1 - Controls the mode of issueing TX_EN & RX_EN to the Radio 1 - TX_EN and RX_EN are issued through direct pins 0 - TX_EN and RX_EN are issued through RCB writes"]
    #[inline(always)]
    pub fn tx_rx_ctrl_sel(&mut self) -> TX_RX_CTRL_SEL_W {
        TX_RX_CTRL_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Setting this bit enables the tx 1MHz pulse to match the received bpktctl from CYBLERD55. This will result is reduced TIFS variation"]
    #[inline(always)]
    pub fn tifs_enable(&mut self) -> TIFS_ENABLE_W {
        TIFS_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Controls the wakeup timer configuration 1 - Wakeup time is compensated with the LF_OFFSET 0 - Wakeup time is not compensated with the LF_OFFSET as in legacy mode"]
    #[inline(always)]
    pub fn timer_lf_slot_enable(&mut self) -> TIMER_LF_SLOT_ENABLE_W {
        TIMER_LF_SLOT_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Controls the engine interrupt generation based on RSSI reads. This is valid only if RSSI_SEL is 0. 0 - Receive interrupts are triggerred after the RSSI read is complete 1 - Receive interrupts are triggerred after the last bit of CRC"]
    #[inline(always)]
    pub fn rssi_intr_sel(&mut self) -> RSSI_INTR_SEL_W {
        RSSI_INTR_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Controls the early RSSI reads. This is applicable only when RSSI_SEL is 1. 1 - RSSI read is initiated during the first CRC byte reception. 0 - RSSI read is initiated during the third CRC byte reception."]
    #[inline(always)]
    pub fn rssi_early_cnfg(&mut self) -> RSSI_EARLY_CNFG_W {
        RSSI_EARLY_CNFG_W { w: self }
    }
    #[doc = "Bit 7 - Controls the delay from DBUS_TX, DBUS_RX assertion to the assertion on the pins. This is applicable only when TX_RX_CTRL_SEL is set. 0 - The pin assertion is delayed by 4 cycles. 1 - The pin assertion is delayed by 8 cycles."]
    #[inline(always)]
    pub fn tx_rx_pin_dly(&mut self) -> TX_RX_PIN_DLY_W {
        TX_RX_PIN_DLY_W { w: self }
    }
    #[doc = "Bit 8 - Controls the TX power level format given to the CYBLERD55 chip. 0 - The power level given to CYBLERD55 is in 4 bit code format from ADV_CH_TX_POWER for advertising channel and DTM packets & from CONN_CH_TX_POWER for connection channel packets. The power level setting is decoded and given to the PA. 1 - The power level given to CYBLERD55 is in 18 bit power level setting format from {ADV_CH_TX_POWER_LVL_MS, ADV_CH_TX_POWER_LVL_LS} channel and DTM packets & from {CONN_CH_TX_POWER_LVL_MS, CONN_CH_TX_POWER_LVL_LS} for connection channel packets. This setting is directly given to the PA."]
    #[inline(always)]
    pub fn tx_pa_pwr_lvl_type(&mut self) -> TX_PA_PWR_LVL_TYPE_W {
        TX_PA_PWR_LVL_TYPE_W { w: self }
    }
    #[doc = "Bit 9 - Controls the RSSI reads. 0 - Channel Energy read is not initiated if no packet is received during a receive cycle 1 - Channel Energy read is initiated at the end of the receive cycle if no packet is received"]
    #[inline(always)]
    pub fn rssi_energy_rd(&mut self) -> RSSI_ENERGY_RD_W {
        RSSI_ENERGY_RD_W { w: self }
    }
    #[doc = "Bit 10 - Controls the RSSI reads. 0 - RSSI read is not initiated for zero length and aborted packets 1 - RSSI read is initiated for zero length and aborted packets"]
    #[inline(always)]
    pub fn rssi_each_pkt(&mut self) -> RSSI_EACH_PKT_W {
        RSSI_EACH_PKT_W { w: self }
    }
    #[doc = "Bit 11 - Controls the RCB update to radio on TX/RX enable. Applicable only when TX_RX_CTRL_SEL is 1'b1 0 - RCB update is triggerred only when the fields change on rising edge of TX/RX enable 1 - RCB update is force triggerred on rising edge of TX/RX enable If TX_RX_CTRL_SEL is 1'b1 and ENABLE_RADIO_BOD is 1'b1, this bit needs to be set to 1'b1"]
    #[inline(always)]
    pub fn force_trig_rcb_update(&mut self) -> FORCE_TRIG_RCB_UPDATE_W {
        FORCE_TRIG_RCB_UPDATE_W { w: self }
    }
    #[doc = "Bit 12 - Controls the duplicate connection checkin ADV and INIT 0 - Does not check if the peer is already connection before a new connection is created 1 - Checks if the peer is already connection before a new connection is created and aborts a duplicate connection creation"]
    #[inline(always)]
    pub fn check_dup_conn(&mut self) -> CHECK_DUP_CONN_W {
        CHECK_DUP_CONN_W { w: self }
    }
    #[doc = "Bit 13 - Controls the LPM entry condition 0 - Legacy mode LPM entry check 1 - MMMS mode LPM entry check"]
    #[inline(always)]
    pub fn multi_engine_lpm(&mut self) -> MULTI_ENGINE_LPM_W {
        MULTI_ENGINE_LPM_W { w: self }
    }
    #[doc = "Bit 14 - Controls the ADV behavior while advertising ADV_DIR and only device privacy is set. When the ADV is transmitting INITA RPA, the bahavior when an Identity address in received from the Initiator in the CONN_REQ is given below 0 - Abort the CONN_REQ and continue with advertisement 1 - Check the address against PEER_SEC_ADDR_ADV and create connection on a match."]
    #[inline(always)]
    pub fn adv_dir_device_priv_en(&mut self) -> ADV_DIR_DEVICE_PRIV_EN_W {
        ADV_DIR_DEVICE_PRIV_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Layer additional configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_config](index.html) module"]
pub struct LL_CONFIG_SPEC;
impl crate::RegisterSpec for LL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_config::R](R) reader structure"]
impl crate::Readable for LL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ll_config::W](W) writer structure"]
impl crate::Writable for LL_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LL_CONFIG to value 0x4c00"]
impl crate::Resettable for LL_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4c00
    }
}
