#[doc = "Register `MT_CFG` reader"]
pub struct R(crate::R<MT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_CFG` writer"]
pub struct W(crate::W<MT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_CFG_SPEC>;
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
impl From<crate::W<MT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_BLERD` reader - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
pub struct ENABLE_BLERD_R(crate::FieldReader<bool, bool>);
impl ENABLE_BLERD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_BLERD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_BLERD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_BLERD` writer - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
pub struct ENABLE_BLERD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BLERD_W<'a> {
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
#[doc = "Field `DEEPSLEEP_EXIT_CFG` reader - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
pub struct DEEPSLEEP_EXIT_CFG_R(crate::FieldReader<bool, bool>);
impl DEEPSLEEP_EXIT_CFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPSLEEP_EXIT_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEPSLEEP_EXIT_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPSLEEP_EXIT_CFG` writer - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
pub struct DEEPSLEEP_EXIT_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_EXIT_CFG_W<'a> {
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
#[doc = "Field `DEEPSLEEP_EXITED` reader - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
pub struct DEEPSLEEP_EXITED_R(crate::FieldReader<bool, bool>);
impl DEEPSLEEP_EXITED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPSLEEP_EXITED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEPSLEEP_EXITED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPSLEEP_EXITED` writer - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
pub struct DEEPSLEEP_EXITED_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_EXITED_W<'a> {
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
#[doc = "Field `ACT_LDO_NOT_BUCK` reader - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
pub struct ACT_LDO_NOT_BUCK_R(crate::FieldReader<bool, bool>);
impl ACT_LDO_NOT_BUCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACT_LDO_NOT_BUCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_LDO_NOT_BUCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_LDO_NOT_BUCK` writer - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
pub struct ACT_LDO_NOT_BUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_LDO_NOT_BUCK_W<'a> {
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
#[doc = "Field `OVERRIDE_HVLDO_BYPASS` reader - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
pub struct OVERRIDE_HVLDO_BYPASS_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_HVLDO_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_HVLDO_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_HVLDO_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_HVLDO_BYPASS` writer - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
pub struct OVERRIDE_HVLDO_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_HVLDO_BYPASS_W<'a> {
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
#[doc = "Field `HVLDO_BYPASS` reader - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
pub struct HVLDO_BYPASS_R(crate::FieldReader<bool, bool>);
impl HVLDO_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLDO_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLDO_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_BYPASS` writer - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
pub struct HVLDO_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_BYPASS_W<'a> {
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
#[doc = "Field `OVERRIDE_ACT_REGULATOR` reader - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
pub struct OVERRIDE_ACT_REGULATOR_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_ACT_REGULATOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_ACT_REGULATOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_ACT_REGULATOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_ACT_REGULATOR` writer - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
pub struct OVERRIDE_ACT_REGULATOR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_ACT_REGULATOR_W<'a> {
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
#[doc = "Field `ACT_REGULATOR_EN` reader - Override value for ACT_LDO_EN/BUCK_EN"]
pub struct ACT_REGULATOR_EN_R(crate::FieldReader<bool, bool>);
impl ACT_REGULATOR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REGULATOR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REGULATOR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REGULATOR_EN` writer - Override value for ACT_LDO_EN/BUCK_EN"]
pub struct ACT_REGULATOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REGULATOR_EN_W<'a> {
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
#[doc = "Field `OVERRIDE_DIG_REGULATOR` reader - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
pub struct OVERRIDE_DIG_REGULATOR_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_DIG_REGULATOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_DIG_REGULATOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_DIG_REGULATOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_DIG_REGULATOR` writer - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
pub struct OVERRIDE_DIG_REGULATOR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_DIG_REGULATOR_W<'a> {
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
#[doc = "Field `DIG_REGULATOR_EN` reader - Override value for digital regulator of CYBLERD55"]
pub struct DIG_REGULATOR_EN_R(crate::FieldReader<bool, bool>);
impl DIG_REGULATOR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_REGULATOR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_REGULATOR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_REGULATOR_EN` writer - Override value for digital regulator of CYBLERD55"]
pub struct DIG_REGULATOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_REGULATOR_EN_W<'a> {
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
#[doc = "Field `OVERRIDE_RET_SWITCH` reader - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
pub struct OVERRIDE_RET_SWITCH_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_RET_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_RET_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_RET_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_RET_SWITCH` writer - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
pub struct OVERRIDE_RET_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_RET_SWITCH_W<'a> {
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
#[doc = "Field `RET_SWITCH` reader - Override value for RET_SWITCH"]
pub struct RET_SWITCH_R(crate::FieldReader<bool, bool>);
impl RET_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RET_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RET_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RET_SWITCH` writer - Override value for RET_SWITCH"]
pub struct RET_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_SWITCH_W<'a> {
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
#[doc = "Field `OVERRIDE_ISOLATE` reader - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
pub struct OVERRIDE_ISOLATE_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_ISOLATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_ISOLATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_ISOLATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_ISOLATE` writer - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
pub struct OVERRIDE_ISOLATE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_ISOLATE_W<'a> {
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
#[doc = "Field `ISOLATE_N` reader - Override value for isolation to CYBLERD55"]
pub struct ISOLATE_N_R(crate::FieldReader<bool, bool>);
impl ISOLATE_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOLATE_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOLATE_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOLATE_N` writer - Override value for isolation to CYBLERD55"]
pub struct ISOLATE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOLATE_N_W<'a> {
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
#[doc = "Field `OVERRIDE_LL_CLK_EN` reader - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
pub struct OVERRIDE_LL_CLK_EN_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_LL_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_LL_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_LL_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_LL_CLK_EN` writer - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
pub struct OVERRIDE_LL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_LL_CLK_EN_W<'a> {
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
#[doc = "Field `LL_CLK_EN` reader - Override value for LL Clock gate"]
pub struct LL_CLK_EN_R(crate::FieldReader<bool, bool>);
impl LL_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LL_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LL_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LL_CLK_EN` writer - Override value for LL Clock gate"]
pub struct LL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_CLK_EN_W<'a> {
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
#[doc = "Field `OVERRIDE_HVLDO_EN` reader - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
pub struct OVERRIDE_HVLDO_EN_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_HVLDO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_HVLDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_HVLDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_HVLDO_EN` writer - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
pub struct OVERRIDE_HVLDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_HVLDO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `HVLDO_EN` reader - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
pub struct HVLDO_EN_R(crate::FieldReader<bool, bool>);
impl HVLDO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_EN` writer - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
pub struct HVLDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DPSLP_ECO_ON` reader - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
pub struct DPSLP_ECO_ON_R(crate::FieldReader<bool, bool>);
impl DPSLP_ECO_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_ECO_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_ECO_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_ECO_ON` writer - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
pub struct DPSLP_ECO_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_ECO_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OVERRIDE_RESET_N` reader - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
pub struct OVERRIDE_RESET_N_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_RESET_N` writer - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
pub struct OVERRIDE_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RESET_N` reader - Overrie value for CYBLERD55 RESET_N"]
pub struct RESET_N_R(crate::FieldReader<bool, bool>);
impl RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_N` writer - Overrie value for CYBLERD55 RESET_N"]
pub struct RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `OVERRIDE_XTAL_EN` reader - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
pub struct OVERRIDE_XTAL_EN_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_XTAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_XTAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_XTAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_XTAL_EN` writer - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
pub struct OVERRIDE_XTAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_XTAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `XTAL_EN` reader - Overrie value for CYBLERD55 XTAL_EN"]
pub struct XTAL_EN_R(crate::FieldReader<bool, bool>);
impl XTAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_EN` writer - Overrie value for CYBLERD55 XTAL_EN"]
pub struct XTAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `OVERRIDE_CLK_EN` reader - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
pub struct OVERRIDE_CLK_EN_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_CLK_EN` writer - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
pub struct OVERRIDE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `BLERD_CLK_EN` reader - Overrie value for CYBLERD55 CLK_EN"]
pub struct BLERD_CLK_EN_R(crate::FieldReader<bool, bool>);
impl BLERD_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLERD_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLERD_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLERD_CLK_EN` writer - Overrie value for CYBLERD55 CLK_EN"]
pub struct BLERD_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `OVERRIDE_RET_LDO_OL` reader - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
pub struct OVERRIDE_RET_LDO_OL_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_RET_LDO_OL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_RET_LDO_OL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_RET_LDO_OL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE_RET_LDO_OL` writer - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
pub struct OVERRIDE_RET_LDO_OL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_RET_LDO_OL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `RET_LDO_OL` reader - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
pub struct RET_LDO_OL_R(crate::FieldReader<bool, bool>);
impl RET_LDO_OL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RET_LDO_OL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RET_LDO_OL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RET_LDO_OL` writer - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
pub struct RET_LDO_OL_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_LDO_OL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `HVLDO_POR_HV` reader - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
pub struct HVLDO_POR_HV_R(crate::FieldReader<bool, bool>);
impl HVLDO_POR_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        HVLDO_POR_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HVLDO_POR_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HVLDO_POR_HV` writer - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
pub struct HVLDO_POR_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_POR_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
    #[inline(always)]
    pub fn enable_blerd(&self) -> ENABLE_BLERD_R {
        ENABLE_BLERD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
    #[inline(always)]
    pub fn deepsleep_exit_cfg(&self) -> DEEPSLEEP_EXIT_CFG_R {
        DEEPSLEEP_EXIT_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
    #[inline(always)]
    pub fn deepsleep_exited(&self) -> DEEPSLEEP_EXITED_R {
        DEEPSLEEP_EXITED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
    #[inline(always)]
    pub fn act_ldo_not_buck(&self) -> ACT_LDO_NOT_BUCK_R {
        ACT_LDO_NOT_BUCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
    #[inline(always)]
    pub fn override_hvldo_bypass(&self) -> OVERRIDE_HVLDO_BYPASS_R {
        OVERRIDE_HVLDO_BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
    #[inline(always)]
    pub fn hvldo_bypass(&self) -> HVLDO_BYPASS_R {
        HVLDO_BYPASS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_act_regulator(&self) -> OVERRIDE_ACT_REGULATOR_R {
        OVERRIDE_ACT_REGULATOR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Override value for ACT_LDO_EN/BUCK_EN"]
    #[inline(always)]
    pub fn act_regulator_en(&self) -> ACT_REGULATOR_EN_R {
        ACT_REGULATOR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_dig_regulator(&self) -> OVERRIDE_DIG_REGULATOR_R {
        OVERRIDE_DIG_REGULATOR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Override value for digital regulator of CYBLERD55"]
    #[inline(always)]
    pub fn dig_regulator_en(&self) -> DIG_REGULATOR_EN_R {
        DIG_REGULATOR_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
    #[inline(always)]
    pub fn override_ret_switch(&self) -> OVERRIDE_RET_SWITCH_R {
        OVERRIDE_RET_SWITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Override value for RET_SWITCH"]
    #[inline(always)]
    pub fn ret_switch(&self) -> RET_SWITCH_R {
        RET_SWITCH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
    #[inline(always)]
    pub fn override_isolate(&self) -> OVERRIDE_ISOLATE_R {
        OVERRIDE_ISOLATE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Override value for isolation to CYBLERD55"]
    #[inline(always)]
    pub fn isolate_n(&self) -> ISOLATE_N_R {
        ISOLATE_N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
    #[inline(always)]
    pub fn override_ll_clk_en(&self) -> OVERRIDE_LL_CLK_EN_R {
        OVERRIDE_LL_CLK_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Override value for LL Clock gate"]
    #[inline(always)]
    pub fn ll_clk_en(&self) -> LL_CLK_EN_R {
        LL_CLK_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
    #[inline(always)]
    pub fn override_hvldo_en(&self) -> OVERRIDE_HVLDO_EN_R {
        OVERRIDE_HVLDO_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
    #[inline(always)]
    pub fn hvldo_en(&self) -> HVLDO_EN_R {
        HVLDO_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
    #[inline(always)]
    pub fn dpslp_eco_on(&self) -> DPSLP_ECO_ON_R {
        DPSLP_ECO_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
    #[inline(always)]
    pub fn override_reset_n(&self) -> OVERRIDE_RESET_N_R {
        OVERRIDE_RESET_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Overrie value for CYBLERD55 RESET_N"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
    #[inline(always)]
    pub fn override_xtal_en(&self) -> OVERRIDE_XTAL_EN_R {
        OVERRIDE_XTAL_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Overrie value for CYBLERD55 XTAL_EN"]
    #[inline(always)]
    pub fn xtal_en(&self) -> XTAL_EN_R {
        XTAL_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_clk_en(&self) -> OVERRIDE_CLK_EN_R {
        OVERRIDE_CLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Overrie value for CYBLERD55 CLK_EN"]
    #[inline(always)]
    pub fn blerd_clk_en(&self) -> BLERD_CLK_EN_R {
        BLERD_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_ret_ldo_ol(&self) -> OVERRIDE_RET_LDO_OL_R {
        OVERRIDE_RET_LDO_OL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
    #[inline(always)]
    pub fn ret_ldo_ol(&self) -> RET_LDO_OL_R {
        RET_LDO_OL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
    #[inline(always)]
    pub fn hvldo_por_hv(&self) -> HVLDO_POR_HV_R {
        HVLDO_POR_HV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
    #[inline(always)]
    pub fn enable_blerd(&mut self) -> ENABLE_BLERD_W {
        ENABLE_BLERD_W { w: self }
    }
    #[doc = "Bit 1 - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
    #[inline(always)]
    pub fn deepsleep_exit_cfg(&mut self) -> DEEPSLEEP_EXIT_CFG_W {
        DEEPSLEEP_EXIT_CFG_W { w: self }
    }
    #[doc = "Bit 2 - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
    #[inline(always)]
    pub fn deepsleep_exited(&mut self) -> DEEPSLEEP_EXITED_W {
        DEEPSLEEP_EXITED_W { w: self }
    }
    #[doc = "Bit 3 - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
    #[inline(always)]
    pub fn act_ldo_not_buck(&mut self) -> ACT_LDO_NOT_BUCK_W {
        ACT_LDO_NOT_BUCK_W { w: self }
    }
    #[doc = "Bit 4 - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
    #[inline(always)]
    pub fn override_hvldo_bypass(&mut self) -> OVERRIDE_HVLDO_BYPASS_W {
        OVERRIDE_HVLDO_BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
    #[inline(always)]
    pub fn hvldo_bypass(&mut self) -> HVLDO_BYPASS_W {
        HVLDO_BYPASS_W { w: self }
    }
    #[doc = "Bit 6 - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_act_regulator(&mut self) -> OVERRIDE_ACT_REGULATOR_W {
        OVERRIDE_ACT_REGULATOR_W { w: self }
    }
    #[doc = "Bit 7 - Override value for ACT_LDO_EN/BUCK_EN"]
    #[inline(always)]
    pub fn act_regulator_en(&mut self) -> ACT_REGULATOR_EN_W {
        ACT_REGULATOR_EN_W { w: self }
    }
    #[doc = "Bit 8 - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_dig_regulator(&mut self) -> OVERRIDE_DIG_REGULATOR_W {
        OVERRIDE_DIG_REGULATOR_W { w: self }
    }
    #[doc = "Bit 9 - Override value for digital regulator of CYBLERD55"]
    #[inline(always)]
    pub fn dig_regulator_en(&mut self) -> DIG_REGULATOR_EN_W {
        DIG_REGULATOR_EN_W { w: self }
    }
    #[doc = "Bit 10 - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
    #[inline(always)]
    pub fn override_ret_switch(&mut self) -> OVERRIDE_RET_SWITCH_W {
        OVERRIDE_RET_SWITCH_W { w: self }
    }
    #[doc = "Bit 11 - Override value for RET_SWITCH"]
    #[inline(always)]
    pub fn ret_switch(&mut self) -> RET_SWITCH_W {
        RET_SWITCH_W { w: self }
    }
    #[doc = "Bit 12 - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
    #[inline(always)]
    pub fn override_isolate(&mut self) -> OVERRIDE_ISOLATE_W {
        OVERRIDE_ISOLATE_W { w: self }
    }
    #[doc = "Bit 13 - Override value for isolation to CYBLERD55"]
    #[inline(always)]
    pub fn isolate_n(&mut self) -> ISOLATE_N_W {
        ISOLATE_N_W { w: self }
    }
    #[doc = "Bit 14 - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
    #[inline(always)]
    pub fn override_ll_clk_en(&mut self) -> OVERRIDE_LL_CLK_EN_W {
        OVERRIDE_LL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15 - Override value for LL Clock gate"]
    #[inline(always)]
    pub fn ll_clk_en(&mut self) -> LL_CLK_EN_W {
        LL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16 - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
    #[inline(always)]
    pub fn override_hvldo_en(&mut self) -> OVERRIDE_HVLDO_EN_W {
        OVERRIDE_HVLDO_EN_W { w: self }
    }
    #[doc = "Bit 17 - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
    #[inline(always)]
    pub fn hvldo_en(&mut self) -> HVLDO_EN_W {
        HVLDO_EN_W { w: self }
    }
    #[doc = "Bit 18 - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
    #[inline(always)]
    pub fn dpslp_eco_on(&mut self) -> DPSLP_ECO_ON_W {
        DPSLP_ECO_ON_W { w: self }
    }
    #[doc = "Bit 19 - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
    #[inline(always)]
    pub fn override_reset_n(&mut self) -> OVERRIDE_RESET_N_W {
        OVERRIDE_RESET_N_W { w: self }
    }
    #[doc = "Bit 20 - Overrie value for CYBLERD55 RESET_N"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W {
        RESET_N_W { w: self }
    }
    #[doc = "Bit 21 - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
    #[inline(always)]
    pub fn override_xtal_en(&mut self) -> OVERRIDE_XTAL_EN_W {
        OVERRIDE_XTAL_EN_W { w: self }
    }
    #[doc = "Bit 22 - Overrie value for CYBLERD55 XTAL_EN"]
    #[inline(always)]
    pub fn xtal_en(&mut self) -> XTAL_EN_W {
        XTAL_EN_W { w: self }
    }
    #[doc = "Bit 23 - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_clk_en(&mut self) -> OVERRIDE_CLK_EN_W {
        OVERRIDE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 24 - Overrie value for CYBLERD55 CLK_EN"]
    #[inline(always)]
    pub fn blerd_clk_en(&mut self) -> BLERD_CLK_EN_W {
        BLERD_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25 - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_ret_ldo_ol(&mut self) -> OVERRIDE_RET_LDO_OL_W {
        OVERRIDE_RET_LDO_OL_W { w: self }
    }
    #[doc = "Bit 26 - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
    #[inline(always)]
    pub fn ret_ldo_ol(&mut self) -> RET_LDO_OL_W {
        RET_LDO_OL_W { w: self }
    }
    #[doc = "Bit 27 - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
    #[inline(always)]
    pub fn hvldo_por_hv(&mut self) -> HVLDO_POR_HV_W {
        HVLDO_POR_HV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_cfg](index.html) module"]
pub struct MT_CFG_SPEC;
impl crate::RegisterSpec for MT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_cfg::R](R) reader structure"]
impl crate::Readable for MT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_cfg::W](W) writer structure"]
impl crate::Writable for MT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_CFG to value 0x0810_0000"]
impl crate::Resettable for MT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0810_0000
    }
}
