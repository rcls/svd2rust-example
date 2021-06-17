#[doc = "Register `PCONP0` reader"]
pub struct R(crate::R<PCONP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONP0` writer"]
pub struct W(crate::W<PCONP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONP0_SPEC>;
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
impl From<crate::W<PCONP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLCD` reader - LCD controller power/clock control bit."]
pub struct PCLCD_R(crate::FieldReader<bool, bool>);
impl PCLCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLCD` writer - LCD controller power/clock control bit."]
pub struct PCLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLCD_W<'a> {
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
#[doc = "Field `PCTIM0` reader - Timer/Counter 0 power/clock control bit."]
pub struct PCTIM0_R(crate::FieldReader<bool, bool>);
impl PCTIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCTIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCTIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTIM0` writer - Timer/Counter 0 power/clock control bit."]
pub struct PCTIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM0_W<'a> {
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
#[doc = "Field `PCTIM1` reader - Timer/Counter 1 power/clock control bit."]
pub struct PCTIM1_R(crate::FieldReader<bool, bool>);
impl PCTIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCTIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCTIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTIM1` writer - Timer/Counter 1 power/clock control bit."]
pub struct PCTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM1_W<'a> {
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
#[doc = "Field `PCUART0` reader - UART0 power/clock control bit."]
pub struct PCUART0_R(crate::FieldReader<bool, bool>);
impl PCUART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUART0` writer - UART0 power/clock control bit."]
pub struct PCUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART0_W<'a> {
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
#[doc = "Field `PCUART1` reader - UART1 power/clock control bit."]
pub struct PCUART1_R(crate::FieldReader<bool, bool>);
impl PCUART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUART1` writer - UART1 power/clock control bit."]
pub struct PCUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART1_W<'a> {
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
#[doc = "Field `PCPWM0` reader - PWM0 power/clock control bit."]
pub struct PCPWM0_R(crate::FieldReader<bool, bool>);
impl PCPWM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCPWM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCPWM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCPWM0` writer - PWM0 power/clock control bit."]
pub struct PCPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPWM0_W<'a> {
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
#[doc = "Field `PCPWM1` reader - PWM1 power/clock control bit."]
pub struct PCPWM1_R(crate::FieldReader<bool, bool>);
impl PCPWM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCPWM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCPWM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCPWM1` writer - PWM1 power/clock control bit."]
pub struct PCPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPWM1_W<'a> {
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
#[doc = "Field `PCI2C0` reader - I2C0 interface power/clock control bit."]
pub struct PCI2C0_R(crate::FieldReader<bool, bool>);
impl PCI2C0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCI2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCI2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCI2C0` writer - I2C0 interface power/clock control bit."]
pub struct PCI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C0_W<'a> {
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
#[doc = "Field `PCUART4` reader - UART4 power/clock control bit."]
pub struct PCUART4_R(crate::FieldReader<bool, bool>);
impl PCUART4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUART4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUART4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUART4` writer - UART4 power/clock control bit."]
pub struct PCUART4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART4_W<'a> {
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
#[doc = "Field `PCRTC` reader - RTC and Event Monitor/Recorder power/clock control bit."]
pub struct PCRTC_R(crate::FieldReader<bool, bool>);
impl PCRTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCRTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCRTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCRTC` writer - RTC and Event Monitor/Recorder power/clock control bit."]
pub struct PCRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRTC_W<'a> {
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
#[doc = "Field `PCSSP1` reader - SSP 1 interface power/clock control bit."]
pub struct PCSSP1_R(crate::FieldReader<bool, bool>);
impl PCSSP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSSP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSSP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSP1` writer - SSP 1 interface power/clock control bit."]
pub struct PCSSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSP1_W<'a> {
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
#[doc = "Field `PCEMC` reader - External Memory Controller power/clock control bit."]
pub struct PCEMC_R(crate::FieldReader<bool, bool>);
impl PCEMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCEMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCEMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCEMC` writer - External Memory Controller power/clock control bit."]
pub struct PCEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEMC_W<'a> {
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
#[doc = "Field `PCADC` reader - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before attempting to set PDN."]
pub struct PCADC_R(crate::FieldReader<bool, bool>);
impl PCADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCADC` writer - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before attempting to set PDN."]
pub struct PCADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCADC_W<'a> {
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
#[doc = "Field `PCCAN1` reader - CAN Controller 1 power/clock control bit."]
pub struct PCCAN1_R(crate::FieldReader<bool, bool>);
impl PCCAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCCAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCCAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCCAN1` writer - CAN Controller 1 power/clock control bit."]
pub struct PCCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCAN1_W<'a> {
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
#[doc = "Field `PCCAN2` reader - CAN Controller 2 power/clock control bit."]
pub struct PCCAN2_R(crate::FieldReader<bool, bool>);
impl PCCAN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCCAN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCCAN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCCAN2` writer - CAN Controller 2 power/clock control bit."]
pub struct PCCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCAN2_W<'a> {
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
#[doc = "Field `PCGPIO` reader - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub struct PCGPIO_R(crate::FieldReader<bool, bool>);
impl PCGPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCGPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCGPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCGPIO` writer - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub struct PCGPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PCGPIO_W<'a> {
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
#[doc = "Field `PCSPIFI` reader - SPI Flash Interface power/clock control bit."]
pub struct PCSPIFI_R(crate::FieldReader<bool, bool>);
impl PCSPIFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSPIFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSPIFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSPIFI` writer - SPI Flash Interface power/clock control bit."]
pub struct PCSPIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSPIFI_W<'a> {
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
#[doc = "Field `PCMCPWM` reader - Motor Control PWM power/clock control bit."]
pub struct PCMCPWM_R(crate::FieldReader<bool, bool>);
impl PCMCPWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMCPWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMCPWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMCPWM` writer - Motor Control PWM power/clock control bit."]
pub struct PCMCPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMCPWM_W<'a> {
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
#[doc = "Field `PCQEI` reader - Quadrature Encoder Interface power/clock control bit."]
pub struct PCQEI_R(crate::FieldReader<bool, bool>);
impl PCQEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCQEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCQEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCQEI` writer - Quadrature Encoder Interface power/clock control bit."]
pub struct PCQEI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCQEI_W<'a> {
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
#[doc = "Field `PCI2C1` reader - I2C1 interface power/clock control bit."]
pub struct PCI2C1_R(crate::FieldReader<bool, bool>);
impl PCI2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCI2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCI2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCI2C1` writer - I2C1 interface power/clock control bit."]
pub struct PCI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C1_W<'a> {
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
#[doc = "Field `PCSSP2` reader - SSP2 interface power/clock control bit."]
pub struct PCSSP2_R(crate::FieldReader<bool, bool>);
impl PCSSP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSSP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSSP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSP2` writer - SSP2 interface power/clock control bit."]
pub struct PCSSP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSP2_W<'a> {
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
#[doc = "Field `PCSSP0` reader - SSP0 interface power/clock control bit."]
pub struct PCSSP0_R(crate::FieldReader<bool, bool>);
impl PCSSP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSSP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSSP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSP0` writer - SSP0 interface power/clock control bit."]
pub struct PCSSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSP0_W<'a> {
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
#[doc = "Field `PCTIM2` reader - Timer 2 power/clock control bit."]
pub struct PCTIM2_R(crate::FieldReader<bool, bool>);
impl PCTIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCTIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCTIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTIM2` writer - Timer 2 power/clock control bit."]
pub struct PCTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM2_W<'a> {
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
#[doc = "Field `PCTIM3` reader - Timer 3 power/clock control bit."]
pub struct PCTIM3_R(crate::FieldReader<bool, bool>);
impl PCTIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCTIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCTIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCTIM3` writer - Timer 3 power/clock control bit."]
pub struct PCTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTIM3_W<'a> {
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
#[doc = "Field `PCUART2` reader - UART 2 power/clock control bit."]
pub struct PCUART2_R(crate::FieldReader<bool, bool>);
impl PCUART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUART2` writer - UART 2 power/clock control bit."]
pub struct PCUART2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART2_W<'a> {
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
#[doc = "Field `PCUART3` reader - UART 3 power/clock control bit."]
pub struct PCUART3_R(crate::FieldReader<bool, bool>);
impl PCUART3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUART3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUART3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUART3` writer - UART 3 power/clock control bit."]
pub struct PCUART3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUART3_W<'a> {
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
#[doc = "Field `PCI2C2` reader - I2C interface 2 power/clock control bit."]
pub struct PCI2C2_R(crate::FieldReader<bool, bool>);
impl PCI2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCI2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCI2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCI2C2` writer - I2C interface 2 power/clock control bit."]
pub struct PCI2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2C2_W<'a> {
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
#[doc = "Field `PCI2S` reader - I2S interface power/clock control bit."]
pub struct PCI2S_R(crate::FieldReader<bool, bool>);
impl PCI2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCI2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCI2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCI2S` writer - I2S interface power/clock control bit."]
pub struct PCI2S_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI2S_W<'a> {
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
#[doc = "Field `PCSDC` reader - SD Card interface power/clock control bit."]
pub struct PCSDC_R(crate::FieldReader<bool, bool>);
impl PCSDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSDC` writer - SD Card interface power/clock control bit."]
pub struct PCSDC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSDC_W<'a> {
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
#[doc = "Field `PCGPDMA` reader - GPDMA function power/clock control bit."]
pub struct PCGPDMA_R(crate::FieldReader<bool, bool>);
impl PCGPDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCGPDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCGPDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCGPDMA` writer - GPDMA function power/clock control bit."]
pub struct PCGPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCGPDMA_W<'a> {
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
#[doc = "Field `PCENET` reader - Ethernet block power/clock control bit."]
pub struct PCENET_R(crate::FieldReader<bool, bool>);
impl PCENET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCENET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCENET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCENET` writer - Ethernet block power/clock control bit."]
pub struct PCENET_W<'a> {
    w: &'a mut W,
}
impl<'a> PCENET_W<'a> {
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
#[doc = "Field `PCUSB` reader - USB interface power/clock control bit."]
pub struct PCUSB_R(crate::FieldReader<bool, bool>);
impl PCUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCUSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCUSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCUSB` writer - USB interface power/clock control bit."]
pub struct PCUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PCUSB_W<'a> {
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
    #[doc = "Bit 0 - LCD controller power/clock control bit."]
    #[inline(always)]
    pub fn pclcd(&self) -> PCLCD_R {
        PCLCD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&self) -> PCTIM0_R {
        PCTIM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&self) -> PCTIM1_R {
        PCTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&self) -> PCUART0_R {
        PCUART0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&self) -> PCUART1_R {
        PCUART1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM0 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm0(&self) -> PCPWM0_R {
        PCPWM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&self) -> PCPWM1_R {
        PCPWM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&self) -> PCI2C0_R {
        PCI2C0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART4 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart4(&self) -> PCUART4_R {
        PCUART4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&self) -> PCRTC_R {
        PCRTC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&self) -> PCSSP1_R {
        PCSSP1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Memory Controller power/clock control bit."]
    #[inline(always)]
    pub fn pcemc(&self) -> PCEMC_R {
        PCEMC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before attempting to set PDN."]
    #[inline(always)]
    pub fn pcadc(&self) -> PCADC_R {
        PCADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&self) -> PCCAN1_R {
        PCCAN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&self) -> PCCAN2_R {
        PCCAN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&self) -> PCGPIO_R {
        PCGPIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Flash Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspifi(&self) -> PCSPIFI_R {
        PCSPIFI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM power/clock control bit."]
    #[inline(always)]
    pub fn pcmcpwm(&self) -> PCMCPWM_R {
        PCMCPWM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&self) -> PCQEI_R {
        PCQEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&self) -> PCI2C1_R {
        PCI2C1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SSP2 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp2(&self) -> PCSSP2_R {
        PCSSP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&self) -> PCSSP0_R {
        PCSSP0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&self) -> PCTIM2_R {
        PCTIM2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&self) -> PCTIM3_R {
        PCTIM3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&self) -> PCUART2_R {
        PCUART2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&self) -> PCUART3_R {
        PCUART3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&self) -> PCI2C2_R {
        PCI2C2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&self) -> PCI2S_R {
        PCI2S_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SD Card interface power/clock control bit."]
    #[inline(always)]
    pub fn pcsdc(&self) -> PCSDC_R {
        PCSDC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&self) -> PCGPDMA_R {
        PCGPDMA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&self) -> PCENET_R {
        PCENET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&self) -> PCUSB_R {
        PCUSB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller power/clock control bit."]
    #[inline(always)]
    pub fn pclcd(&mut self) -> PCLCD_W {
        PCLCD_W { w: self }
    }
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&mut self) -> PCTIM0_W {
        PCTIM0_W { w: self }
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&mut self) -> PCTIM1_W {
        PCTIM1_W { w: self }
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&mut self) -> PCUART0_W {
        PCUART0_W { w: self }
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&mut self) -> PCUART1_W {
        PCUART1_W { w: self }
    }
    #[doc = "Bit 5 - PWM0 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm0(&mut self) -> PCPWM0_W {
        PCPWM0_W { w: self }
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&mut self) -> PCPWM1_W {
        PCPWM1_W { w: self }
    }
    #[doc = "Bit 7 - I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&mut self) -> PCI2C0_W {
        PCI2C0_W { w: self }
    }
    #[doc = "Bit 8 - UART4 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart4(&mut self) -> PCUART4_W {
        PCUART4_W { w: self }
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&mut self) -> PCRTC_W {
        PCRTC_W { w: self }
    }
    #[doc = "Bit 10 - SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&mut self) -> PCSSP1_W {
        PCSSP1_W { w: self }
    }
    #[doc = "Bit 11 - External Memory Controller power/clock control bit."]
    #[inline(always)]
    pub fn pcemc(&mut self) -> PCEMC_W {
        PCEMC_W { w: self }
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before attempting to set PDN."]
    #[inline(always)]
    pub fn pcadc(&mut self) -> PCADC_W {
        PCADC_W { w: self }
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&mut self) -> PCCAN1_W {
        PCCAN1_W { w: self }
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&mut self) -> PCCAN2_W {
        PCCAN2_W { w: self }
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&mut self) -> PCGPIO_W {
        PCGPIO_W { w: self }
    }
    #[doc = "Bit 16 - SPI Flash Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspifi(&mut self) -> PCSPIFI_W {
        PCSPIFI_W { w: self }
    }
    #[doc = "Bit 17 - Motor Control PWM power/clock control bit."]
    #[inline(always)]
    pub fn pcmcpwm(&mut self) -> PCMCPWM_W {
        PCMCPWM_W { w: self }
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&mut self) -> PCQEI_W {
        PCQEI_W { w: self }
    }
    #[doc = "Bit 19 - I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&mut self) -> PCI2C1_W {
        PCI2C1_W { w: self }
    }
    #[doc = "Bit 20 - SSP2 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp2(&mut self) -> PCSSP2_W {
        PCSSP2_W { w: self }
    }
    #[doc = "Bit 21 - SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&mut self) -> PCSSP0_W {
        PCSSP0_W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&mut self) -> PCTIM2_W {
        PCTIM2_W { w: self }
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&mut self) -> PCTIM3_W {
        PCTIM3_W { w: self }
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&mut self) -> PCUART2_W {
        PCUART2_W { w: self }
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&mut self) -> PCUART3_W {
        PCUART3_W { w: self }
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&mut self) -> PCI2C2_W {
        PCI2C2_W { w: self }
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&mut self) -> PCI2S_W {
        PCI2S_W { w: self }
    }
    #[doc = "Bit 28 - SD Card interface power/clock control bit."]
    #[inline(always)]
    pub fn pcsdc(&mut self) -> PCSDC_W {
        PCSDC_W { w: self }
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&mut self) -> PCGPDMA_W {
        PCGPDMA_W { w: self }
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&mut self) -> PCENET_W {
        PCENET_W { w: self }
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&mut self) -> PCUSB_W {
        PCUSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control for Peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconp0](index.html) module"]
pub struct PCONP0_SPEC;
impl crate::RegisterSpec for PCONP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pconp0::R](R) reader structure"]
impl crate::Readable for PCONP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pconp0::W](W) writer structure"]
impl crate::Writable for PCONP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONP0 to value 0x0408_829e"]
impl crate::Resettable for PCONP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0408_829e
    }
}
