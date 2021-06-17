#[doc = "Register `SYNCCTRL` reader"]
pub struct R(crate::R<SYNCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCCTRL` writer"]
pub struct W(crate::W<SYNCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCCTRL_SPEC>;
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
impl From<crate::W<SYNCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Enables synchronous mode."]
pub struct SYNC_R(crate::FieldReader<bool, SYNC_A>);
impl SYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLED,
            true => SYNC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYNC_A::ENABLED
    }
}
impl core::ops::Deref for SYNC_R {
    type Target = crate::FieldReader<bool, SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC` writer - Enables synchronous mode."]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNC_A::ENABLED)
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
#[doc = "Clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRC_A {
    #[doc = "0: Synchronous slave mode (SCLK in)"]
    SYNCHRONOUS_SLAVE_MO = 0,
    #[doc = "1: Synchronous master mode (SCLK out)"]
    SYNCHRONOUS_MASTER_M = 1,
}
impl From<CSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRC` reader - Clock source select."]
pub struct CSRC_R(crate::FieldReader<bool, CSRC_A>);
impl CSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRC_A {
        match self.bits {
            false => CSRC_A::SYNCHRONOUS_SLAVE_MO,
            true => CSRC_A::SYNCHRONOUS_MASTER_M,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_SLAVE_MO`"]
    #[inline(always)]
    pub fn is_synchronous_slave_mo(&self) -> bool {
        **self == CSRC_A::SYNCHRONOUS_SLAVE_MO
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MASTER_M`"]
    #[inline(always)]
    pub fn is_synchronous_master_m(&self) -> bool {
        **self == CSRC_A::SYNCHRONOUS_MASTER_M
    }
}
impl core::ops::Deref for CSRC_R {
    type Target = crate::FieldReader<bool, CSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRC` writer - Clock source select."]
pub struct CSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn synchronous_slave_mo(self) -> &'a mut W {
        self.variant(CSRC_A::SYNCHRONOUS_SLAVE_MO)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn synchronous_master_m(self) -> &'a mut W {
        self.variant(CSRC_A::SYNCHRONOUS_MASTER_M)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Falling edge sampling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FES_A {
    #[doc = "0: RxD is sampled on the rising edge of SCLK"]
    RXD_IS_SAMPLED_ON_TH = 0,
    #[doc = "1: RxD is sampled on the falling edge of SCLK"]
    RXD_IS_SAMPLED_ON_TH = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FES` reader - Falling edge sampling."]
pub struct FES_R(crate::FieldReader<bool, FES_A>);
impl FES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::RXD_IS_SAMPLED_ON_TH,
            true => FES_A::RXD_IS_SAMPLED_ON_TH,
        }
    }
    #[doc = "Checks if the value of the field is `RXD_IS_SAMPLED_ON_TH`"]
    #[inline(always)]
    pub fn is_rxd_is_sampled_on_th(&self) -> bool {
        **self == FES_A::RXD_IS_SAMPLED_ON_TH
    }
    #[doc = "Checks if the value of the field is `RXD_IS_SAMPLED_ON_TH`"]
    #[inline(always)]
    pub fn is_rxd_is_sampled_on_th(&self) -> bool {
        **self == FES_A::RXD_IS_SAMPLED_ON_TH
    }
}
impl core::ops::Deref for FES_R {
    type Target = crate::FieldReader<bool, FES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FES` writer - Falling edge sampling."]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn rxd_is_sampled_on_th(self) -> &'a mut W {
        self.variant(FES_A::RXD_IS_SAMPLED_ON_TH)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn rxd_is_sampled_on_th(self) -> &'a mut W {
        self.variant(FES_A::RXD_IS_SAMPLED_ON_TH)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Transmit synchronization bypass in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASS_A {
    #[doc = "0: The input clock is synchronized prior to being used in clock edge detection logic."]
    THE_INPUT_CLOCK_IS_S = 0,
    #[doc = "1: The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    THE_INPUT_CLOCK_IS_N = 1,
}
impl From<TSBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: TSBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSBYPASS` reader - Transmit synchronization bypass in synchronous slave mode."]
pub struct TSBYPASS_R(crate::FieldReader<bool, TSBYPASS_A>);
impl TSBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSBYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSBYPASS_A {
        match self.bits {
            false => TSBYPASS_A::THE_INPUT_CLOCK_IS_S,
            true => TSBYPASS_A::THE_INPUT_CLOCK_IS_N,
        }
    }
    #[doc = "Checks if the value of the field is `THE_INPUT_CLOCK_IS_S`"]
    #[inline(always)]
    pub fn is_the_input_clock_is_s(&self) -> bool {
        **self == TSBYPASS_A::THE_INPUT_CLOCK_IS_S
    }
    #[doc = "Checks if the value of the field is `THE_INPUT_CLOCK_IS_N`"]
    #[inline(always)]
    pub fn is_the_input_clock_is_n(&self) -> bool {
        **self == TSBYPASS_A::THE_INPUT_CLOCK_IS_N
    }
}
impl core::ops::Deref for TSBYPASS_R {
    type Target = crate::FieldReader<bool, TSBYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSBYPASS` writer - Transmit synchronization bypass in synchronous slave mode."]
pub struct TSBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSBYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    #[inline(always)]
    pub fn the_input_clock_is_s(self) -> &'a mut W {
        self.variant(TSBYPASS_A::THE_INPUT_CLOCK_IS_S)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn the_input_clock_is_n(self) -> &'a mut W {
        self.variant(TSBYPASS_A::THE_INPUT_CLOCK_IS_N)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Continuous master clock enable (used only when CSRC is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCEN_A {
    #[doc = "0: SCLK cycles only when characters are being sent on TxD"]
    SCLK_CYCLES_ONLY_WHE = 0,
    #[doc = "1: SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    SCLK_RUNS_CONTINUOUS = 1,
}
impl From<CSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCEN` reader - Continuous master clock enable (used only when CSRC is 1)"]
pub struct CSCEN_R(crate::FieldReader<bool, CSCEN_A>);
impl CSCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCEN_A {
        match self.bits {
            false => CSCEN_A::SCLK_CYCLES_ONLY_WHE,
            true => CSCEN_A::SCLK_RUNS_CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_CYCLES_ONLY_WHE`"]
    #[inline(always)]
    pub fn is_sclk_cycles_only_whe(&self) -> bool {
        **self == CSCEN_A::SCLK_CYCLES_ONLY_WHE
    }
    #[doc = "Checks if the value of the field is `SCLK_RUNS_CONTINUOUS`"]
    #[inline(always)]
    pub fn is_sclk_runs_continuous(&self) -> bool {
        **self == CSCEN_A::SCLK_RUNS_CONTINUOUS
    }
}
impl core::ops::Deref for CSCEN_R {
    type Target = crate::FieldReader<bool, CSCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCEN` writer - Continuous master clock enable (used only when CSRC is 1)"]
pub struct CSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn sclk_cycles_only_whe(self) -> &'a mut W {
        self.variant(CSCEN_A::SCLK_CYCLES_ONLY_WHE)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn sclk_runs_continuous(self) -> &'a mut W {
        self.variant(CSCEN_A::SCLK_RUNS_CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Start/stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSSDIS_A {
    #[doc = "0: Send start and stop bits as in other modes."]
    SEND_START_AND_STOP_ = 0,
    #[doc = "1: Do not send start/stop bits."]
    NOSTARTSTOPBIT = 1,
}
impl From<SSSDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSSDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSSDIS` reader - Start/stop bits"]
pub struct SSSDIS_R(crate::FieldReader<bool, SSSDIS_A>);
impl SSSDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSSDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSSDIS_A {
        match self.bits {
            false => SSSDIS_A::SEND_START_AND_STOP_,
            true => SSSDIS_A::NOSTARTSTOPBIT,
        }
    }
    #[doc = "Checks if the value of the field is `SEND_START_AND_STOP_`"]
    #[inline(always)]
    pub fn is_send_start_and_stop_(&self) -> bool {
        **self == SSSDIS_A::SEND_START_AND_STOP_
    }
    #[doc = "Checks if the value of the field is `NOSTARTSTOPBIT`"]
    #[inline(always)]
    pub fn is_nostartstopbit(&self) -> bool {
        **self == SSSDIS_A::NOSTARTSTOPBIT
    }
}
impl core::ops::Deref for SSSDIS_R {
    type Target = crate::FieldReader<bool, SSSDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSSDIS` writer - Start/stop bits"]
pub struct SSSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSSDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSSDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn send_start_and_stop_(self) -> &'a mut W {
        self.variant(SSSDIS_A::SEND_START_AND_STOP_)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn nostartstopbit(self) -> &'a mut W {
        self.variant(SSSDIS_A::NOSTARTSTOPBIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Continuous clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLR_A {
    #[doc = "0: CSCEN is under software control."]
    CSCEN_IS_UNDER_SOFTW = 0,
    #[doc = "1: Hardware clears CSCEN after each character is received."]
    HARDWARE_CLEARS_CSCE = 1,
}
impl From<CCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCLR` reader - Continuous clock clear"]
pub struct CCCLR_R(crate::FieldReader<bool, CCCLR_A>);
impl CCCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCLR_A {
        match self.bits {
            false => CCCLR_A::CSCEN_IS_UNDER_SOFTW,
            true => CCCLR_A::HARDWARE_CLEARS_CSCE,
        }
    }
    #[doc = "Checks if the value of the field is `CSCEN_IS_UNDER_SOFTW`"]
    #[inline(always)]
    pub fn is_cscen_is_under_softw(&self) -> bool {
        **self == CCCLR_A::CSCEN_IS_UNDER_SOFTW
    }
    #[doc = "Checks if the value of the field is `HARDWARE_CLEARS_CSCE`"]
    #[inline(always)]
    pub fn is_hardware_clears_csce(&self) -> bool {
        **self == CCCLR_A::HARDWARE_CLEARS_CSCE
    }
}
impl core::ops::Deref for CCCLR_R {
    type Target = crate::FieldReader<bool, CCCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCCLR` writer - Continuous clock clear"]
pub struct CCCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn cscen_is_under_softw(self) -> &'a mut W {
        self.variant(CCCLR_A::CSCEN_IS_UNDER_SOFTW)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn hardware_clears_csce(self) -> &'a mut W {
        self.variant(CCCLR_A::HARDWARE_CLEARS_CSCE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&self) -> CSRC_R {
        CSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&self) -> TSBYPASS_R {
        TSBYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&self) -> CSCEN_R {
        CSCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn sssdis(&self) -> SSSDIS_R {
        SSSDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&self) -> CCCLR_R {
        CCCLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&mut self) -> CSRC_W {
        CSRC_W { w: self }
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&mut self) -> TSBYPASS_W {
        TSBYPASS_W { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&mut self) -> CSCEN_W {
        CSCEN_W { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn sssdis(&mut self) -> SSSDIS_W {
        SSSDIS_W { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&mut self) -> CCCLR_W {
        CCCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncctrl](index.html) module"]
pub struct SYNCCTRL_SPEC;
impl crate::RegisterSpec for SYNCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncctrl::R](R) reader structure"]
impl crate::Readable for SYNCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncctrl::W](W) writer structure"]
impl crate::Writable for SYNCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNCCTRL to value 0"]
impl crate::Resettable for SYNCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
