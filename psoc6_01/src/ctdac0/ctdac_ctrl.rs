#[doc = "Register `CTDAC_CTRL` reader"]
pub struct R(crate::R<CTDAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTDAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTDAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTDAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTDAC_CTRL` writer"]
pub struct W(crate::W<CTDAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTDAC_CTRL_SPEC>;
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
impl From<crate::W<CTDAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTDAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEGLITCH_CNT` reader - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
pub struct DEGLITCH_CNT_R(crate::FieldReader<u8, u8>);
impl DEGLITCH_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEGLITCH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEGLITCH_CNT` writer - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
pub struct DEGLITCH_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `DEGLITCH_CO6` reader - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
pub struct DEGLITCH_CO6_R(crate::FieldReader<bool, bool>);
impl DEGLITCH_CO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEGLITCH_CO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_CO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEGLITCH_CO6` writer - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
pub struct DEGLITCH_CO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_CO6_W<'a> {
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
#[doc = "Field `DEGLITCH_COS` reader - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
pub struct DEGLITCH_COS_R(crate::FieldReader<bool, bool>);
impl DEGLITCH_COS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEGLITCH_COS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_COS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEGLITCH_COS` writer - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
pub struct DEGLITCH_COS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_COS_W<'a> {
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
#[doc = "Field `OUT_EN` reader - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
pub struct OUT_EN_R(crate::FieldReader<bool, bool>);
impl OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EN` writer - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
#[doc = "Field `CTDAC_RANGE` reader - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
pub struct CTDAC_RANGE_R(crate::FieldReader<bool, bool>);
impl CTDAC_RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDAC_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDAC_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC_RANGE` writer - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
pub struct CTDAC_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDAC_RANGE_W<'a> {
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
#[doc = "DAC mode, this determines the Value decoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTDAC_MODE_A {
    #[doc = "0: Unsigned 12-bit VDAC, i.e. no value decoding."]
    UNSIGNED12 = 0,
    #[doc = "1: Virtual signed 12-bits' VDAC. Value decoding: \nadd 0x800 to the 12-bit Value (=invert MSB), to convert the lowest signed number 0x800 to the lowest unsigned number 0x000. This is the same as the SAR handles 12-bit 'virtual' signed numbers."]
    VIRT_SIGNED12 = 1,
    #[doc = "2: N/A"]
    RSVD2 = 2,
    #[doc = "3: N/A"]
    RSVD3 = 3,
}
impl From<CTDAC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTDAC_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTDAC_MODE` reader - DAC mode, this determines the Value decoding"]
pub struct CTDAC_MODE_R(crate::FieldReader<u8, CTDAC_MODE_A>);
impl CTDAC_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTDAC_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTDAC_MODE_A {
        match self.bits {
            0 => CTDAC_MODE_A::UNSIGNED12,
            1 => CTDAC_MODE_A::VIRT_SIGNED12,
            2 => CTDAC_MODE_A::RSVD2,
            3 => CTDAC_MODE_A::RSVD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED12`"]
    #[inline(always)]
    pub fn is_unsigned12(&self) -> bool {
        **self == CTDAC_MODE_A::UNSIGNED12
    }
    #[doc = "Checks if the value of the field is `VIRT_SIGNED12`"]
    #[inline(always)]
    pub fn is_virt_signed12(&self) -> bool {
        **self == CTDAC_MODE_A::VIRT_SIGNED12
    }
    #[doc = "Checks if the value of the field is `RSVD2`"]
    #[inline(always)]
    pub fn is_rsvd2(&self) -> bool {
        **self == CTDAC_MODE_A::RSVD2
    }
    #[doc = "Checks if the value of the field is `RSVD3`"]
    #[inline(always)]
    pub fn is_rsvd3(&self) -> bool {
        **self == CTDAC_MODE_A::RSVD3
    }
}
impl core::ops::Deref for CTDAC_MODE_R {
    type Target = crate::FieldReader<u8, CTDAC_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC_MODE` writer - DAC mode, this determines the Value decoding"]
pub struct CTDAC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDAC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTDAC_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Unsigned 12-bit VDAC, i.e. no value decoding."]
    #[inline(always)]
    pub fn unsigned12(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::UNSIGNED12)
    }
    #[doc = "Virtual signed 12-bits' VDAC. Value decoding: add 0x800 to the 12-bit Value (=invert MSB), to convert the lowest signed number 0x800 to the lowest unsigned number 0x000. This is the same as the SAR handles 12-bit 'virtual' signed numbers."]
    #[inline(always)]
    pub fn virt_signed12(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::VIRT_SIGNED12)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd2(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::RSVD2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd3(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::RSVD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `DISABLED_MODE` reader - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
pub struct DISABLED_MODE_R(crate::FieldReader<bool, bool>);
impl DISABLED_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLED_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLED_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLED_MODE` writer - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
pub struct DISABLED_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_MODE_W<'a> {
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
#[doc = "Field `DSI_STROBE_EN` reader - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
pub struct DSI_STROBE_EN_R(crate::FieldReader<bool, bool>);
impl DSI_STROBE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_STROBE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_STROBE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_STROBE_EN` writer - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
pub struct DSI_STROBE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_STROBE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DSI_STROBE_LEVEL` reader - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
pub struct DSI_STROBE_LEVEL_R(crate::FieldReader<bool, bool>);
impl DSI_STROBE_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_STROBE_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_STROBE_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_STROBE_LEVEL` writer - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
pub struct DSI_STROBE_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_STROBE_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub struct DEEPSLEEP_ON_R(crate::FieldReader<bool, bool>);
impl DEEPSLEEP_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPSLEEP_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEPSLEEP_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub struct DEEPSLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&self) -> DEGLITCH_CNT_R {
        DEGLITCH_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&self) -> DEGLITCH_CO6_R {
        DEGLITCH_CO6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&self) -> DEGLITCH_COS_R {
        DEGLITCH_COS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&self) -> CTDAC_RANGE_R {
        CTDAC_RANGE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&self) -> CTDAC_MODE_R {
        CTDAC_MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&self) -> DISABLED_MODE_R {
        DISABLED_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&self) -> DSI_STROBE_EN_R {
        DSI_STROBE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&self) -> DSI_STROBE_LEVEL_R {
        DSI_STROBE_LEVEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&mut self) -> DEGLITCH_CNT_W {
        DEGLITCH_CNT_W { w: self }
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&mut self) -> DEGLITCH_CO6_W {
        DEGLITCH_CO6_W { w: self }
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&mut self) -> DEGLITCH_COS_W {
        DEGLITCH_COS_W { w: self }
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&mut self) -> CTDAC_RANGE_W {
        CTDAC_RANGE_W { w: self }
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&mut self) -> CTDAC_MODE_W {
        CTDAC_MODE_W { w: self }
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&mut self) -> DISABLED_MODE_W {
        DISABLED_MODE_W { w: self }
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&mut self) -> DSI_STROBE_EN_W {
        DSI_STROBE_EN_W { w: self }
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&mut self) -> DSI_STROBE_LEVEL_W {
        DSI_STROBE_LEVEL_W { w: self }
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W { w: self }
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CTDAC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_ctrl](index.html) module"]
pub struct CTDAC_CTRL_SPEC;
impl crate::RegisterSpec for CTDAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctdac_ctrl::R](R) reader structure"]
impl crate::Readable for CTDAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctdac_ctrl::W](W) writer structure"]
impl crate::Writable for CTDAC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTDAC_CTRL to value 0"]
impl crate::Resettable for CTDAC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
