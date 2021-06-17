#[doc = "Register `AREF_CTRL` reader"]
pub struct R(crate::R<AREF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AREF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AREF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AREF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AREF_CTRL` writer"]
pub struct W(crate::W<AREF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AREF_CTRL_SPEC>;
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
impl From<crate::W<AREF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AREF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control bit to trade off AREF settling and noise performance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREF_MODE_A {
    #[doc = "0: Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    NORMAL = 0,
    #[doc = "1: High noise fast startup mode (meets fast mode settling and noise specifications)"]
    FAST_START = 1,
}
impl From<AREF_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AREF_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AREF_MODE` reader - Control bit to trade off AREF settling and noise performance"]
pub struct AREF_MODE_R(crate::FieldReader<bool, AREF_MODE_A>);
impl AREF_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AREF_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AREF_MODE_A {
        match self.bits {
            false => AREF_MODE_A::NORMAL,
            true => AREF_MODE_A::FAST_START,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == AREF_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FAST_START`"]
    #[inline(always)]
    pub fn is_fast_start(&self) -> bool {
        **self == AREF_MODE_A::FAST_START
    }
}
impl core::ops::Deref for AREF_MODE_R {
    type Target = crate::FieldReader<bool, AREF_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AREF_MODE` writer - Control bit to trade off AREF settling and noise performance"]
pub struct AREF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AREF_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AREF_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AREF_MODE_A::NORMAL)
    }
    #[doc = "High noise fast startup mode (meets fast mode settling and noise specifications)"]
    #[inline(always)]
    pub fn fast_start(self) -> &'a mut W {
        self.variant(AREF_MODE_A::FAST_START)
    }
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
#[doc = "Field `AREF_BIAS_SCALE` reader - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub struct AREF_BIAS_SCALE_R(crate::FieldReader<u8, u8>);
impl AREF_BIAS_SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AREF_BIAS_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AREF_BIAS_SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AREF_BIAS_SCALE` writer - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub struct AREF_BIAS_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> AREF_BIAS_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `AREF_RMB` reader - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub struct AREF_RMB_R(crate::FieldReader<u8, u8>);
impl AREF_RMB_R {
    pub(crate) fn new(bits: u8) -> Self {
        AREF_RMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AREF_RMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AREF_RMB` writer - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub struct AREF_RMB_W<'a> {
    w: &'a mut W,
}
impl<'a> AREF_RMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `CTB_IPTAT_SCALE` reader - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub struct CTB_IPTAT_SCALE_R(crate::FieldReader<bool, bool>);
impl CTB_IPTAT_SCALE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB_IPTAT_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB_IPTAT_SCALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB_IPTAT_SCALE` writer - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub struct CTB_IPTAT_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB_IPTAT_SCALE_W<'a> {
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
#[doc = "Field `CTB_IPTAT_REDIRECT` reader - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
pub struct CTB_IPTAT_REDIRECT_R(crate::FieldReader<u8, u8>);
impl CTB_IPTAT_REDIRECT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTB_IPTAT_REDIRECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB_IPTAT_REDIRECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB_IPTAT_REDIRECT` writer - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
pub struct CTB_IPTAT_REDIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB_IPTAT_REDIRECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "iztat current select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IZTAT_SEL_A {
    #[doc = "0: Use 250nA IZTAT from SRSS"]
    SRSS = 0,
    #[doc = "1: Use locally generated 250nA"]
    LOCAL = 1,
}
impl From<IZTAT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IZTAT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IZTAT_SEL` reader - iztat current select control"]
pub struct IZTAT_SEL_R(crate::FieldReader<bool, IZTAT_SEL_A>);
impl IZTAT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IZTAT_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IZTAT_SEL_A {
        match self.bits {
            false => IZTAT_SEL_A::SRSS,
            true => IZTAT_SEL_A::LOCAL,
        }
    }
    #[doc = "Checks if the value of the field is `SRSS`"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        **self == IZTAT_SEL_A::SRSS
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        **self == IZTAT_SEL_A::LOCAL
    }
}
impl core::ops::Deref for IZTAT_SEL_R {
    type Target = crate::FieldReader<bool, IZTAT_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IZTAT_SEL` writer - iztat current select control"]
pub struct IZTAT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IZTAT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IZTAT_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 250nA IZTAT from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut W {
        self.variant(IZTAT_SEL_A::SRSS)
    }
    #[doc = "Use locally generated 250nA"]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(IZTAT_SEL_A::LOCAL)
    }
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
#[doc = "Field `CLOCK_PUMP_PERI_SEL` reader - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub struct CLOCK_PUMP_PERI_SEL_R(crate::FieldReader<bool, bool>);
impl CLOCK_PUMP_PERI_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_PUMP_PERI_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_PUMP_PERI_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_PUMP_PERI_SEL` writer - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub struct CLOCK_PUMP_PERI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_PUMP_PERI_SEL_W<'a> {
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
#[doc = "bandgap voltage select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREF_SEL_A {
    #[doc = "0: Use 0.8V Vref from SRSS"]
    SRSS = 0,
    #[doc = "1: Use locally generated Vref"]
    LOCAL = 1,
    #[doc = "2: Use externally supplied Vref (aref_ext_vref)"]
    EXTERNAL = 2,
}
impl From<VREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREF_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREF_SEL` reader - bandgap voltage select control"]
pub struct VREF_SEL_R(crate::FieldReader<u8, VREF_SEL_A>);
impl VREF_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREF_SEL_A> {
        match self.bits {
            0 => Some(VREF_SEL_A::SRSS),
            1 => Some(VREF_SEL_A::LOCAL),
            2 => Some(VREF_SEL_A::EXTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SRSS`"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        **self == VREF_SEL_A::SRSS
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        **self == VREF_SEL_A::LOCAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == VREF_SEL_A::EXTERNAL
    }
}
impl core::ops::Deref for VREF_SEL_R {
    type Target = crate::FieldReader<u8, VREF_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF_SEL` writer - bandgap voltage select control"]
pub struct VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREF_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use 0.8V Vref from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut W {
        self.variant(VREF_SEL_A::SRSS)
    }
    #[doc = "Use locally generated Vref"]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(VREF_SEL_A::LOCAL)
    }
    #[doc = "Use externally supplied Vref (aref_ext_vref)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(VREF_SEL_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEEPSLEEP_MODE_A {
    #[doc = "0: All blocks 'OFF' in DeepSleep"]
    OFF = 0,
    #[doc = "1: IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    IPTAT = 1,
    #[doc = "2: IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep)\n\n*Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    IPTAT_IZTAT = 2,
    #[doc = "3: IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    IPTAT_IZTAT_VREF = 3,
}
impl From<DEEPSLEEP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEEPSLEEP_MODE` reader - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub struct DEEPSLEEP_MODE_R(crate::FieldReader<u8, DEEPSLEEP_MODE_A>);
impl DEEPSLEEP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEEPSLEEP_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPSLEEP_MODE_A {
        match self.bits {
            0 => DEEPSLEEP_MODE_A::OFF,
            1 => DEEPSLEEP_MODE_A::IPTAT,
            2 => DEEPSLEEP_MODE_A::IPTAT_IZTAT,
            3 => DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DEEPSLEEP_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `IPTAT`"]
    #[inline(always)]
    pub fn is_iptat(&self) -> bool {
        **self == DEEPSLEEP_MODE_A::IPTAT
    }
    #[doc = "Checks if the value of the field is `IPTAT_IZTAT`"]
    #[inline(always)]
    pub fn is_iptat_iztat(&self) -> bool {
        **self == DEEPSLEEP_MODE_A::IPTAT_IZTAT
    }
    #[doc = "Checks if the value of the field is `IPTAT_IZTAT_VREF`"]
    #[inline(always)]
    pub fn is_iptat_iztat_vref(&self) -> bool {
        **self == DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF
    }
}
impl core::ops::Deref for DEEPSLEEP_MODE_R {
    type Target = crate::FieldReader<u8, DEEPSLEEP_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPSLEEP_MODE` writer - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub struct DEEPSLEEP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEPSLEEP_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "All blocks 'OFF' in DeepSleep"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::OFF)
    }
    #[doc = "IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    #[inline(always)]
    pub fn iptat(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT)
    }
    #[doc = "IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    #[inline(always)]
    pub fn iptat_iztat(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT_IZTAT)
    }
    #[doc = "IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    #[inline(always)]
    pub fn iptat_iztat_vref(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
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
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
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
#[doc = "Field `ENABLED` reader - Disable AREF"]
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
#[doc = "Field `ENABLED` writer - Disable AREF"]
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
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn aref_mode(&self) -> AREF_MODE_R {
        AREF_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn aref_bias_scale(&self) -> AREF_BIAS_SCALE_R {
        AREF_BIAS_SCALE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn aref_rmb(&self) -> AREF_RMB_R {
        AREF_RMB_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn ctb_iptat_scale(&self) -> CTB_IPTAT_SCALE_R {
        CTB_IPTAT_SCALE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn ctb_iptat_redirect(&self) -> CTB_IPTAT_REDIRECT_R {
        CTB_IPTAT_REDIRECT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    pub fn iztat_sel(&self) -> IZTAT_SEL_R {
        IZTAT_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn clock_pump_peri_sel(&self) -> CLOCK_PUMP_PERI_SEL_R {
        CLOCK_PUMP_PERI_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VREF_SEL_R {
        VREF_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn deepsleep_mode(&self) -> DEEPSLEEP_MODE_R {
        DEEPSLEEP_MODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable AREF"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn aref_mode(&mut self) -> AREF_MODE_W {
        AREF_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn aref_bias_scale(&mut self) -> AREF_BIAS_SCALE_W {
        AREF_BIAS_SCALE_W { w: self }
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn aref_rmb(&mut self) -> AREF_RMB_W {
        AREF_RMB_W { w: self }
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn ctb_iptat_scale(&mut self) -> CTB_IPTAT_SCALE_W {
        CTB_IPTAT_SCALE_W { w: self }
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn ctb_iptat_redirect(&mut self) -> CTB_IPTAT_REDIRECT_W {
        CTB_IPTAT_REDIRECT_W { w: self }
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    pub fn iztat_sel(&mut self) -> IZTAT_SEL_W {
        IZTAT_SEL_W { w: self }
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn clock_pump_peri_sel(&mut self) -> CLOCK_PUMP_PERI_SEL_W {
        CLOCK_PUMP_PERI_SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VREF_SEL_W {
        VREF_SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn deepsleep_mode(&mut self) -> DEEPSLEEP_MODE_W {
        DEEPSLEEP_MODE_W { w: self }
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W { w: self }
    }
    #[doc = "Bit 31 - Disable AREF"]
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
#[doc = "global AREF control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aref_ctrl](index.html) module"]
pub struct AREF_CTRL_SPEC;
impl crate::RegisterSpec for AREF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aref_ctrl::R](R) reader structure"]
impl crate::Readable for AREF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aref_ctrl::W](W) writer structure"]
impl crate::Writable for AREF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AREF_CTRL to value 0"]
impl crate::Resettable for AREF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
