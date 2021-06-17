#[doc = "Register `CLOCK_CONFIG` reader"]
pub struct R(crate::R<CLOCK_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CONFIG` writer"]
pub struct W(crate::W<CLOCK_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CONFIG_SPEC>;
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
impl From<crate::W<CLOCK_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_CLK_GATE_EN` reader - Advertiser block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the advertiser module (llh_adv) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct ADV_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl ADV_CLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_CLK_GATE_EN` writer - Advertiser block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the advertiser module (llh_adv) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct ADV_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `SCAN_CLK_GATE_EN` reader - Scan block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the scanner module (llh_scan) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct SCAN_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl SCAN_CLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_CLK_GATE_EN` writer - Scan block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the scanner module (llh_scan) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct SCAN_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `INIT_CLK_GATE_EN` reader - Initiator block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the initiator module (llh_init). If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct INIT_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl INIT_CLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_CLK_GATE_EN` writer - Initiator block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the initiator module (llh_init). If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct INIT_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `CONN_CLK_GATE_EN` reader - Connection block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the connection module (llh_connch_top) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the engine. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct CONN_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl CONN_CLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONN_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_CLK_GATE_EN` writer - Connection block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the connection module (llh_connch_top) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the engine. If 0, the logic has no control and clock to the module is always turned ON."]
pub struct CONN_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `CORECLK_GATE_EN` reader - Core clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the llh_core module in hard-ware. If 1, the sleep mode/deep sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock is always turned ON."]
pub struct CORECLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl CORECLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORECLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORECLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORECLK_GATE_EN` writer - Core clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the llh_core module in hard-ware. If 1, the sleep mode/deep sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock is always turned ON."]
pub struct CORECLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORECLK_GATE_EN_W<'a> {
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
#[doc = "Field `SYSCLK_GATE_EN` reader - Sysclk gate enable. 1- enable, 0 - disable. Enables clock gating of system clock input to the link layer. If 1, it enables the DSM logic to control the clock gate for system clock input from pin. If 0, the DSM logic has no control and the system clock is always ON."]
pub struct SYSCLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl SYSCLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCLK_GATE_EN` writer - Sysclk gate enable. 1- enable, 0 - disable. Enables clock gating of system clock input to the link layer. If 1, it enables the DSM logic to control the clock gate for system clock input from pin. If 0, the DSM logic has no control and the system clock is always ON."]
pub struct SYSCLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_GATE_EN_W<'a> {
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
#[doc = "Field `PHY_CLK_GATE_EN` reader - Digital PHY clock enable. 1- enable, 0-disable. Enable the Digital PHY to shutdown the clock. When 1, it indicates that controller has an upcoming activity so PHY clock must be turned ON. When 0, it indicates inactivity in the controller."]
pub struct PHY_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl PHY_CLK_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHY_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_CLK_GATE_EN` writer - Digital PHY clock enable. 1- enable, 0-disable. Enable the Digital PHY to shutdown the clock. When 1, it indicates that controller has an upcoming activity so PHY clock must be turned ON. When 0, it indicates inactivity in the controller."]
pub struct PHY_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `LLH_IDLE` reader - Indicates if hardware is doing any transmit/receive operation. This information is used by firmware to decide to program the hardware into deep sleep mode. 1 - LL hardware is idle. 0 - LL hardware is busy. In this case LL hardware will not enter deep sleep mode, even if firmware gives an enter DSM command. (In this situation hardware generates dsm exit interrupt to inform firmware that DSM entry was not successful)."]
pub struct LLH_IDLE_R(crate::FieldReader<bool, bool>);
impl LLH_IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LLH_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLH_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPO_CLK_FREQ_SEL` reader - Clock frequency select. 0 - 32KHz, 1 - 32.768KHz. Base frequency of the sleep_clk input used for generat-ing the internal reference clock of approximate 16Khz frequency."]
pub struct LPO_CLK_FREQ_SEL_R(crate::FieldReader<bool, bool>);
impl LPO_CLK_FREQ_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPO_CLK_FREQ_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPO_CLK_FREQ_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPO_CLK_FREQ_SEL` writer - Clock frequency select. 0 - 32KHz, 1 - 32.768KHz. Base frequency of the sleep_clk input used for generat-ing the internal reference clock of approximate 16Khz frequency."]
pub struct LPO_CLK_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO_CLK_FREQ_SEL_W<'a> {
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
#[doc = "Field `LPO_SEL_EXTERNAL` reader - Select external sleep clock. 1 - External clock, 0 - inter-nal generated clock. The field is used to select either the low power clock in-put on sleep_clk input pin(of frequency 16.384KHz) di-rectly to run the DSM logic or to use the internal gener-ated reference clock(of 16KHz) for the same."]
pub struct LPO_SEL_EXTERNAL_R(crate::FieldReader<bool, bool>);
impl LPO_SEL_EXTERNAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPO_SEL_EXTERNAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPO_SEL_EXTERNAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPO_SEL_EXTERNAL` writer - Select external sleep clock. 1 - External clock, 0 - inter-nal generated clock. The field is used to select either the low power clock in-put on sleep_clk input pin(of frequency 16.384KHz) di-rectly to run the DSM logic or to use the internal gener-ated reference clock(of 16KHz) for the same."]
pub struct LPO_SEL_EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO_SEL_EXTERNAL_W<'a> {
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
#[doc = "Field `SM_AUTO_WKUP_EN` reader - Enable sleep mode auto wakeup enable. 1- enable, 0 - disable. Enables hardware to automatically wakeup from sleep mode at the instant = wakeup_instant - sm_offset_to_wakeup_instant. The wakeup_insant is the field in the wakeup control register described earlier. The sm_offset_to_wakeup_instant value is the field described in the wakeup configuration register."]
pub struct SM_AUTO_WKUP_EN_R(crate::FieldReader<bool, bool>);
impl SM_AUTO_WKUP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SM_AUTO_WKUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_AUTO_WKUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_AUTO_WKUP_EN` writer - Enable sleep mode auto wakeup enable. 1- enable, 0 - disable. Enables hardware to automatically wakeup from sleep mode at the instant = wakeup_instant - sm_offset_to_wakeup_instant. The wakeup_insant is the field in the wakeup control register described earlier. The sm_offset_to_wakeup_instant value is the field described in the wakeup configuration register."]
pub struct SM_AUTO_WKUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_AUTO_WKUP_EN_W<'a> {
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
#[doc = "Field `SM_INTR_EN` reader - Enable SM exit interrupt. 1 - enable, 0 - disable. Enables hardware to generate an interrupt while exiting sleep mode - irrespective of whether it is initiated by hardware or firmware. The interrupt is captured and stored till it gets cleared. Disabling this bit mask the sleep mode exit event from hardware & firmware. This feature is not available. FW should never set this bit"]
pub struct SM_INTR_EN_R(crate::FieldReader<bool, bool>);
impl SM_INTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SM_INTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_INTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_INTR_EN` writer - Enable SM exit interrupt. 1 - enable, 0 - disable. Enables hardware to generate an interrupt while exiting sleep mode - irrespective of whether it is initiated by hardware or firmware. The interrupt is captured and stored till it gets cleared. Disabling this bit mask the sleep mode exit event from hardware & firmware. This feature is not available. FW should never set this bit"]
pub struct SM_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_INTR_EN_W<'a> {
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
#[doc = "Field `DEEP_SLEEP_AUTO_WKUP_DISABLE` reader - Disable Auto Wakeup in DEEP_SLEEP mode. 1 - Disable Auto Wakeup 0 - Auto Wakeup enabled"]
pub struct DEEP_SLEEP_AUTO_WKUP_DISABLE_R(crate::FieldReader<bool, bool>);
impl DEEP_SLEEP_AUTO_WKUP_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEP_SLEEP_AUTO_WKUP_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLEEP_AUTO_WKUP_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEP_SLEEP_AUTO_WKUP_DISABLE` writer - Disable Auto Wakeup in DEEP_SLEEP mode. 1 - Disable Auto Wakeup 0 - Auto Wakeup enabled"]
pub struct DEEP_SLEEP_AUTO_WKUP_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_AUTO_WKUP_DISABLE_W<'a> {
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
#[doc = "Field `SLEEP_MODE_EN` reader - Enable sleep mode. 1 - enable, 0 - disable. Enables hardware to control sleep mode operation. This feature is not available. FW should never set this bit"]
pub struct SLEEP_MODE_EN_R(crate::FieldReader<bool, bool>);
impl SLEEP_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_MODE_EN` writer - Enable sleep mode. 1 - enable, 0 - disable. Enables hardware to control sleep mode operation. This feature is not available. FW should never set this bit"]
pub struct SLEEP_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_MODE_EN_W<'a> {
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
#[doc = "Field `DEEP_SLEEP_MODE_EN` reader - Enable deep sleep mode. 1 - enable, 0 - disable. Enables hardware logic related to deep sleep mode to control the deep sleep mode operation. If disabled, the related logic is not executed and hardware cannot enter deep sleep mode."]
pub struct DEEP_SLEEP_MODE_EN_R(crate::FieldReader<bool, bool>);
impl DEEP_SLEEP_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEP_SLEEP_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLEEP_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEP_SLEEP_MODE_EN` writer - Enable deep sleep mode. 1 - enable, 0 - disable. Enables hardware logic related to deep sleep mode to control the deep sleep mode operation. If disabled, the related logic is not executed and hardware cannot enter deep sleep mode."]
pub struct DEEP_SLEEP_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_MODE_EN_W<'a> {
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
    #[doc = "Bit 0 - Advertiser block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the advertiser module (llh_adv) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn adv_clk_gate_en(&self) -> ADV_CLK_GATE_EN_R {
        ADV_CLK_GATE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scan block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the scanner module (llh_scan) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn scan_clk_gate_en(&self) -> SCAN_CLK_GATE_EN_R {
        SCAN_CLK_GATE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initiator block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the initiator module (llh_init). If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn init_clk_gate_en(&self) -> INIT_CLK_GATE_EN_R {
        INIT_CLK_GATE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Connection block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the connection module (llh_connch_top) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the engine. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn conn_clk_gate_en(&self) -> CONN_CLK_GATE_EN_R {
        CONN_CLK_GATE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Core clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the llh_core module in hard-ware. If 1, the sleep mode/deep sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock is always turned ON."]
    #[inline(always)]
    pub fn coreclk_gate_en(&self) -> CORECLK_GATE_EN_R {
        CORECLK_GATE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sysclk gate enable. 1- enable, 0 - disable. Enables clock gating of system clock input to the link layer. If 1, it enables the DSM logic to control the clock gate for system clock input from pin. If 0, the DSM logic has no control and the system clock is always ON."]
    #[inline(always)]
    pub fn sysclk_gate_en(&self) -> SYSCLK_GATE_EN_R {
        SYSCLK_GATE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Digital PHY clock enable. 1- enable, 0-disable. Enable the Digital PHY to shutdown the clock. When 1, it indicates that controller has an upcoming activity so PHY clock must be turned ON. When 0, it indicates inactivity in the controller."]
    #[inline(always)]
    pub fn phy_clk_gate_en(&self) -> PHY_CLK_GATE_EN_R {
        PHY_CLK_GATE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates if hardware is doing any transmit/receive operation. This information is used by firmware to decide to program the hardware into deep sleep mode. 1 - LL hardware is idle. 0 - LL hardware is busy. In this case LL hardware will not enter deep sleep mode, even if firmware gives an enter DSM command. (In this situation hardware generates dsm exit interrupt to inform firmware that DSM entry was not successful)."]
    #[inline(always)]
    pub fn llh_idle(&self) -> LLH_IDLE_R {
        LLH_IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clock frequency select. 0 - 32KHz, 1 - 32.768KHz. Base frequency of the sleep_clk input used for generat-ing the internal reference clock of approximate 16Khz frequency."]
    #[inline(always)]
    pub fn lpo_clk_freq_sel(&self) -> LPO_CLK_FREQ_SEL_R {
        LPO_CLK_FREQ_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select external sleep clock. 1 - External clock, 0 - inter-nal generated clock. The field is used to select either the low power clock in-put on sleep_clk input pin(of frequency 16.384KHz) di-rectly to run the DSM logic or to use the internal gener-ated reference clock(of 16KHz) for the same."]
    #[inline(always)]
    pub fn lpo_sel_external(&self) -> LPO_SEL_EXTERNAL_R {
        LPO_SEL_EXTERNAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable sleep mode auto wakeup enable. 1- enable, 0 - disable. Enables hardware to automatically wakeup from sleep mode at the instant = wakeup_instant - sm_offset_to_wakeup_instant. The wakeup_insant is the field in the wakeup control register described earlier. The sm_offset_to_wakeup_instant value is the field described in the wakeup configuration register."]
    #[inline(always)]
    pub fn sm_auto_wkup_en(&self) -> SM_AUTO_WKUP_EN_R {
        SM_AUTO_WKUP_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable SM exit interrupt. 1 - enable, 0 - disable. Enables hardware to generate an interrupt while exiting sleep mode - irrespective of whether it is initiated by hardware or firmware. The interrupt is captured and stored till it gets cleared. Disabling this bit mask the sleep mode exit event from hardware & firmware. This feature is not available. FW should never set this bit"]
    #[inline(always)]
    pub fn sm_intr_en(&self) -> SM_INTR_EN_R {
        SM_INTR_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Auto Wakeup in DEEP_SLEEP mode. 1 - Disable Auto Wakeup 0 - Auto Wakeup enabled"]
    #[inline(always)]
    pub fn deep_sleep_auto_wkup_disable(&self) -> DEEP_SLEEP_AUTO_WKUP_DISABLE_R {
        DEEP_SLEEP_AUTO_WKUP_DISABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable sleep mode. 1 - enable, 0 - disable. Enables hardware to control sleep mode operation. This feature is not available. FW should never set this bit"]
    #[inline(always)]
    pub fn sleep_mode_en(&self) -> SLEEP_MODE_EN_R {
        SLEEP_MODE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable deep sleep mode. 1 - enable, 0 - disable. Enables hardware logic related to deep sleep mode to control the deep sleep mode operation. If disabled, the related logic is not executed and hardware cannot enter deep sleep mode."]
    #[inline(always)]
    pub fn deep_sleep_mode_en(&self) -> DEEP_SLEEP_MODE_EN_R {
        DEEP_SLEEP_MODE_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advertiser block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the advertiser module (llh_adv) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn adv_clk_gate_en(&mut self) -> ADV_CLK_GATE_EN_W {
        ADV_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 1 - Scan block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the scanner module (llh_scan) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn scan_clk_gate_en(&mut self) -> SCAN_CLK_GATE_EN_W {
        SCAN_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 2 - Initiator block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the initiator module (llh_init). If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn init_clk_gate_en(&mut self) -> INIT_CLK_GATE_EN_W {
        INIT_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 3 - Connection block clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the connection module (llh_connch_top) in hardware. If 1, the sleep mode logic can control the clock gate to shutdown/wakeup the clock to the engine. If 0, the logic has no control and clock to the module is always turned ON."]
    #[inline(always)]
    pub fn conn_clk_gate_en(&mut self) -> CONN_CLK_GATE_EN_W {
        CONN_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Core clock gate enable. 1 - enable, 0 - disable. Enables gating of clock to the llh_core module in hard-ware. If 1, the sleep mode/deep sleep mode logic can control the clock gate to shutdown/wakeup the clock to the module. If 0, the logic has no control and clock is always turned ON."]
    #[inline(always)]
    pub fn coreclk_gate_en(&mut self) -> CORECLK_GATE_EN_W {
        CORECLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 5 - Sysclk gate enable. 1- enable, 0 - disable. Enables clock gating of system clock input to the link layer. If 1, it enables the DSM logic to control the clock gate for system clock input from pin. If 0, the DSM logic has no control and the system clock is always ON."]
    #[inline(always)]
    pub fn sysclk_gate_en(&mut self) -> SYSCLK_GATE_EN_W {
        SYSCLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 6 - Digital PHY clock enable. 1- enable, 0-disable. Enable the Digital PHY to shutdown the clock. When 1, it indicates that controller has an upcoming activity so PHY clock must be turned ON. When 0, it indicates inactivity in the controller."]
    #[inline(always)]
    pub fn phy_clk_gate_en(&mut self) -> PHY_CLK_GATE_EN_W {
        PHY_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 8 - Clock frequency select. 0 - 32KHz, 1 - 32.768KHz. Base frequency of the sleep_clk input used for generat-ing the internal reference clock of approximate 16Khz frequency."]
    #[inline(always)]
    pub fn lpo_clk_freq_sel(&mut self) -> LPO_CLK_FREQ_SEL_W {
        LPO_CLK_FREQ_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Select external sleep clock. 1 - External clock, 0 - inter-nal generated clock. The field is used to select either the low power clock in-put on sleep_clk input pin(of frequency 16.384KHz) di-rectly to run the DSM logic or to use the internal gener-ated reference clock(of 16KHz) for the same."]
    #[inline(always)]
    pub fn lpo_sel_external(&mut self) -> LPO_SEL_EXTERNAL_W {
        LPO_SEL_EXTERNAL_W { w: self }
    }
    #[doc = "Bit 10 - Enable sleep mode auto wakeup enable. 1- enable, 0 - disable. Enables hardware to automatically wakeup from sleep mode at the instant = wakeup_instant - sm_offset_to_wakeup_instant. The wakeup_insant is the field in the wakeup control register described earlier. The sm_offset_to_wakeup_instant value is the field described in the wakeup configuration register."]
    #[inline(always)]
    pub fn sm_auto_wkup_en(&mut self) -> SM_AUTO_WKUP_EN_W {
        SM_AUTO_WKUP_EN_W { w: self }
    }
    #[doc = "Bit 12 - Enable SM exit interrupt. 1 - enable, 0 - disable. Enables hardware to generate an interrupt while exiting sleep mode - irrespective of whether it is initiated by hardware or firmware. The interrupt is captured and stored till it gets cleared. Disabling this bit mask the sleep mode exit event from hardware & firmware. This feature is not available. FW should never set this bit"]
    #[inline(always)]
    pub fn sm_intr_en(&mut self) -> SM_INTR_EN_W {
        SM_INTR_EN_W { w: self }
    }
    #[doc = "Bit 13 - Disable Auto Wakeup in DEEP_SLEEP mode. 1 - Disable Auto Wakeup 0 - Auto Wakeup enabled"]
    #[inline(always)]
    pub fn deep_sleep_auto_wkup_disable(&mut self) -> DEEP_SLEEP_AUTO_WKUP_DISABLE_W {
        DEEP_SLEEP_AUTO_WKUP_DISABLE_W { w: self }
    }
    #[doc = "Bit 14 - Enable sleep mode. 1 - enable, 0 - disable. Enables hardware to control sleep mode operation. This feature is not available. FW should never set this bit"]
    #[inline(always)]
    pub fn sleep_mode_en(&mut self) -> SLEEP_MODE_EN_W {
        SLEEP_MODE_EN_W { w: self }
    }
    #[doc = "Bit 15 - Enable deep sleep mode. 1 - enable, 0 - disable. Enables hardware logic related to deep sleep mode to control the deep sleep mode operation. If disabled, the related logic is not executed and hardware cannot enter deep sleep mode."]
    #[inline(always)]
    pub fn deep_sleep_mode_en(&mut self) -> DEEP_SLEEP_MODE_EN_W {
        DEEP_SLEEP_MODE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_config](index.html) module"]
pub struct CLOCK_CONFIG_SPEC;
impl crate::RegisterSpec for CLOCK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_config::R](R) reader structure"]
impl crate::Readable for CLOCK_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_config::W](W) writer structure"]
impl crate::Writable for CLOCK_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_CONFIG to value 0x80"]
impl crate::Resettable for CLOCK_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
