#[doc = "Register `RSTCON0` reader"]
pub struct R(crate::R<RSTCON0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCON0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCON0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCON0` writer"]
pub struct W(crate::W<RSTCON0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCON0_SPEC>;
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
impl From<crate::W<RSTCON0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCON0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTLCD` reader - LCD controller reset control bit."]
pub struct RSTLCD_R(crate::FieldReader<bool, bool>);
impl RSTLCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTLCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTLCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTLCD` writer - LCD controller reset control bit."]
pub struct RSTLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTLCD_W<'a> {
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
#[doc = "Field `RSTTIM0` reader - Timer/Counter 0 reset control bit."]
pub struct RSTTIM0_R(crate::FieldReader<bool, bool>);
impl RSTTIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTTIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTTIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTTIM0` writer - Timer/Counter 0 reset control bit."]
pub struct RSTTIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM0_W<'a> {
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
#[doc = "Field `RSTTIM1` reader - Timer/Counter 1 reset control bit."]
pub struct RSTTIM1_R(crate::FieldReader<bool, bool>);
impl RSTTIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTTIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTTIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTTIM1` writer - Timer/Counter 1 reset control bit."]
pub struct RSTTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM1_W<'a> {
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
#[doc = "Field `RSTUART0` reader - UART0 reset control bit."]
pub struct RSTUART0_R(crate::FieldReader<bool, bool>);
impl RSTUART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUART0` writer - UART0 reset control bit."]
pub struct RSTUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART0_W<'a> {
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
#[doc = "Field `RSTUART1` reader - UART1 reset control bit."]
pub struct RSTUART1_R(crate::FieldReader<bool, bool>);
impl RSTUART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUART1` writer - UART1 reset control bit."]
pub struct RSTUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART1_W<'a> {
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
#[doc = "Field `RSTPWM0` reader - PWM0 reset control bit."]
pub struct RSTPWM0_R(crate::FieldReader<bool, bool>);
impl RSTPWM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTPWM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTPWM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTPWM0` writer - PWM0 reset control bit."]
pub struct RSTPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPWM0_W<'a> {
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
#[doc = "Field `RSTPWM1` reader - PWM1 reset control bit."]
pub struct RSTPWM1_R(crate::FieldReader<bool, bool>);
impl RSTPWM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTPWM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTPWM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTPWM1` writer - PWM1 reset control bit."]
pub struct RSTPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPWM1_W<'a> {
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
#[doc = "Field `RSTI2C0` reader - The I2C0 interface reset control bit."]
pub struct RSTI2C0_R(crate::FieldReader<bool, bool>);
impl RSTI2C0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTI2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI2C0` writer - The I2C0 interface reset control bit."]
pub struct RSTI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C0_W<'a> {
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
#[doc = "Field `RSTUART4` reader - UART4 reset control bit."]
pub struct RSTUART4_R(crate::FieldReader<bool, bool>);
impl RSTUART4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUART4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUART4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUART4` writer - UART4 reset control bit."]
pub struct RSTUART4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART4_W<'a> {
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
#[doc = "Field `RSTRTC` reader - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 626 \"Register overview: Real-Time Clock (base address 0x4002 4000)\" for details."]
pub struct RSTRTC_R(crate::FieldReader<bool, bool>);
impl RSTRTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTRTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTRTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTRTC` writer - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 626 \"Register overview: Real-Time Clock (base address 0x4002 4000)\" for details."]
pub struct RSTRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRTC_W<'a> {
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
#[doc = "Field `RSTSSP1` reader - The SSP 1 interface reset control bit."]
pub struct RSTSSP1_R(crate::FieldReader<bool, bool>);
impl RSTSSP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSSP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSSP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSSP1` writer - The SSP 1 interface reset control bit."]
pub struct RSTSSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP1_W<'a> {
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
#[doc = "Field `RSTEMC` reader - External Memory Controller reset control bit."]
pub struct RSTEMC_R(crate::FieldReader<bool, bool>);
impl RSTEMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTEMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTEMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTEMC` writer - External Memory Controller reset control bit."]
pub struct RSTEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEMC_W<'a> {
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
#[doc = "Field `RSTADC` reader - A/D converter (ADC) reset control bit."]
pub struct RSTADC_R(crate::FieldReader<bool, bool>);
impl RSTADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTADC` writer - A/D converter (ADC) reset control bit."]
pub struct RSTADC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTADC_W<'a> {
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
#[doc = "Field `RSTCAN1` reader - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTCAN1_R(crate::FieldReader<bool, bool>);
impl RSTCAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTCAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCAN1` writer - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAN1_W<'a> {
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
#[doc = "Field `RSTCAN2` reader - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTCAN2_R(crate::FieldReader<bool, bool>);
impl RSTCAN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTCAN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTCAN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCAN2` writer - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAN2_W<'a> {
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
#[doc = "Field `RSTGPIO` reader - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTGPIO_R(crate::FieldReader<bool, bool>);
impl RSTGPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTGPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTGPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTGPIO` writer - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
pub struct RSTGPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTGPIO_W<'a> {
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
#[doc = "Field `RSTSPIFI` reader - SPI Flash Interface reset control bit."]
pub struct RSTSPIFI_R(crate::FieldReader<bool, bool>);
impl RSTSPIFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSPIFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSPIFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSPIFI` writer - SPI Flash Interface reset control bit."]
pub struct RSTSPIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSPIFI_W<'a> {
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
#[doc = "Field `RSTMCPWM` reader - Motor Control PWM reset control bit."]
pub struct RSTMCPWM_R(crate::FieldReader<bool, bool>);
impl RSTMCPWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTMCPWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTMCPWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTMCPWM` writer - Motor Control PWM reset control bit."]
pub struct RSTMCPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTMCPWM_W<'a> {
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
#[doc = "Field `RSTQEI` reader - Quadrature Encoder Interface reset control bit."]
pub struct RSTQEI_R(crate::FieldReader<bool, bool>);
impl RSTQEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTQEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTQEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTQEI` writer - Quadrature Encoder Interface reset control bit."]
pub struct RSTQEI_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTQEI_W<'a> {
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
#[doc = "Field `RSTI2C1` reader - The I2C1 interface reset control bit."]
pub struct RSTI2C1_R(crate::FieldReader<bool, bool>);
impl RSTI2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTI2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI2C1` writer - The I2C1 interface reset control bit."]
pub struct RSTI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C1_W<'a> {
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
#[doc = "Field `RSTSSP2` reader - The SSP2 interface reset control bit."]
pub struct RSTSSP2_R(crate::FieldReader<bool, bool>);
impl RSTSSP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSSP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSSP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSSP2` writer - The SSP2 interface reset control bit."]
pub struct RSTSSP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP2_W<'a> {
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
#[doc = "Field `RSTSSP0` reader - The SSP0 interface reset control bit."]
pub struct RSTSSP0_R(crate::FieldReader<bool, bool>);
impl RSTSSP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSSP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSSP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSSP0` writer - The SSP0 interface reset control bit."]
pub struct RSTSSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP0_W<'a> {
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
#[doc = "Field `RSTTIM2` reader - Timer 2 reset control bit."]
pub struct RSTTIM2_R(crate::FieldReader<bool, bool>);
impl RSTTIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTTIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTTIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTTIM2` writer - Timer 2 reset control bit."]
pub struct RSTTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM2_W<'a> {
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
#[doc = "Field `RSTTIM3` reader - Timer 3 reset control bit."]
pub struct RSTTIM3_R(crate::FieldReader<bool, bool>);
impl RSTTIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTTIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTTIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTTIM3` writer - Timer 3 reset control bit."]
pub struct RSTTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM3_W<'a> {
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
#[doc = "Field `RSTUART2` reader - UART 2 reset control bit."]
pub struct RSTUART2_R(crate::FieldReader<bool, bool>);
impl RSTUART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUART2` writer - UART 2 reset control bit."]
pub struct RSTUART2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART2_W<'a> {
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
#[doc = "Field `RSTUART3` reader - UART 3 reset control bit."]
pub struct RSTUART3_R(crate::FieldReader<bool, bool>);
impl RSTUART3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUART3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUART3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUART3` writer - UART 3 reset control bit."]
pub struct RSTUART3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART3_W<'a> {
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
#[doc = "Field `RSTI2C2` reader - I2C interface 2 reset control bit."]
pub struct RSTI2C2_R(crate::FieldReader<bool, bool>);
impl RSTI2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTI2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI2C2` writer - I2C interface 2 reset control bit."]
pub struct RSTI2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C2_W<'a> {
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
#[doc = "Field `RSTI2S` reader - I2S interface reset control bit."]
pub struct RSTI2S_R(crate::FieldReader<bool, bool>);
impl RSTI2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTI2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI2S` writer - I2S interface reset control bit."]
pub struct RSTI2S_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2S_W<'a> {
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
#[doc = "Field `RSTSDC` reader - SD Card interface reset control bit."]
pub struct RSTSDC_R(crate::FieldReader<bool, bool>);
impl RSTSDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTSDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSDC` writer - SD Card interface reset control bit."]
pub struct RSTSDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSDC_W<'a> {
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
#[doc = "Field `RSTGPDMA` reader - GPDMA function reset control bit."]
pub struct RSTGPDMA_R(crate::FieldReader<bool, bool>);
impl RSTGPDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTGPDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTGPDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTGPDMA` writer - GPDMA function reset control bit."]
pub struct RSTGPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTGPDMA_W<'a> {
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
#[doc = "Field `RSTENET` reader - Ethernet block reset control bit."]
pub struct RSTENET_R(crate::FieldReader<bool, bool>);
impl RSTENET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTENET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTENET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTENET` writer - Ethernet block reset control bit."]
pub struct RSTENET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTENET_W<'a> {
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
#[doc = "Field `RSTUSB` reader - USB interface reset control bit."]
pub struct RSTUSB_R(crate::FieldReader<bool, bool>);
impl RSTUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTUSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTUSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTUSB` writer - USB interface reset control bit."]
pub struct RSTUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUSB_W<'a> {
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
    #[doc = "Bit 0 - LCD controller reset control bit."]
    #[inline(always)]
    pub fn rstlcd(&self) -> RSTLCD_R {
        RSTLCD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter 0 reset control bit."]
    #[inline(always)]
    pub fn rsttim0(&self) -> RSTTIM0_R {
        RSTTIM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 reset control bit."]
    #[inline(always)]
    pub fn rsttim1(&self) -> RSTTIM1_R {
        RSTTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART0 reset control bit."]
    #[inline(always)]
    pub fn rstuart0(&self) -> RSTUART0_R {
        RSTUART0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART1 reset control bit."]
    #[inline(always)]
    pub fn rstuart1(&self) -> RSTUART1_R {
        RSTUART1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM0 reset control bit."]
    #[inline(always)]
    pub fn rstpwm0(&self) -> RSTPWM0_R {
        RSTPWM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM1 reset control bit."]
    #[inline(always)]
    pub fn rstpwm1(&self) -> RSTPWM1_R {
        RSTPWM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c0(&self) -> RSTI2C0_R {
        RSTI2C0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART4 reset control bit."]
    #[inline(always)]
    pub fn rstuart4(&self) -> RSTUART4_R {
        RSTUART4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 626 \"Register overview: Real-Time Clock (base address 0x4002 4000)\" for details."]
    #[inline(always)]
    pub fn rstrtc(&self) -> RSTRTC_R {
        RSTRTC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp1(&self) -> RSTSSP1_R {
        RSTSSP1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Memory Controller reset control bit."]
    #[inline(always)]
    pub fn rstemc(&self) -> RSTEMC_R {
        RSTEMC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) reset control bit."]
    #[inline(always)]
    pub fn rstadc(&self) -> RSTADC_R {
        RSTADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan1(&self) -> RSTCAN1_R {
        RSTCAN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan2(&self) -> RSTCAN2_R {
        RSTCAN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstgpio(&self) -> RSTGPIO_R {
        RSTGPIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Flash Interface reset control bit."]
    #[inline(always)]
    pub fn rstspifi(&self) -> RSTSPIFI_R {
        RSTSPIFI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM reset control bit."]
    #[inline(always)]
    pub fn rstmcpwm(&self) -> RSTMCPWM_R {
        RSTMCPWM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface reset control bit."]
    #[inline(always)]
    pub fn rstqei(&self) -> RSTQEI_R {
        RSTQEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c1(&self) -> RSTI2C1_R {
        RSTI2C1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The SSP2 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp2(&self) -> RSTSSP2_R {
        RSTSSP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp0(&self) -> RSTSSP0_R {
        RSTSSP0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 reset control bit."]
    #[inline(always)]
    pub fn rsttim2(&self) -> RSTTIM2_R {
        RSTTIM2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer 3 reset control bit."]
    #[inline(always)]
    pub fn rsttim3(&self) -> RSTTIM3_R {
        RSTTIM3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UART 2 reset control bit."]
    #[inline(always)]
    pub fn rstuart2(&self) -> RSTUART2_R {
        RSTUART2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART 3 reset control bit."]
    #[inline(always)]
    pub fn rstuart3(&self) -> RSTUART3_R {
        RSTUART3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 reset control bit."]
    #[inline(always)]
    pub fn rsti2c2(&self) -> RSTI2C2_R {
        RSTI2C2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2S interface reset control bit."]
    #[inline(always)]
    pub fn rsti2s(&self) -> RSTI2S_R {
        RSTI2S_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SD Card interface reset control bit."]
    #[inline(always)]
    pub fn rstsdc(&self) -> RSTSDC_R {
        RSTSDC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPDMA function reset control bit."]
    #[inline(always)]
    pub fn rstgpdma(&self) -> RSTGPDMA_R {
        RSTGPDMA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet block reset control bit."]
    #[inline(always)]
    pub fn rstenet(&self) -> RSTENET_R {
        RSTENET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB interface reset control bit."]
    #[inline(always)]
    pub fn rstusb(&self) -> RSTUSB_R {
        RSTUSB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller reset control bit."]
    #[inline(always)]
    pub fn rstlcd(&mut self) -> RSTLCD_W {
        RSTLCD_W { w: self }
    }
    #[doc = "Bit 1 - Timer/Counter 0 reset control bit."]
    #[inline(always)]
    pub fn rsttim0(&mut self) -> RSTTIM0_W {
        RSTTIM0_W { w: self }
    }
    #[doc = "Bit 2 - Timer/Counter 1 reset control bit."]
    #[inline(always)]
    pub fn rsttim1(&mut self) -> RSTTIM1_W {
        RSTTIM1_W { w: self }
    }
    #[doc = "Bit 3 - UART0 reset control bit."]
    #[inline(always)]
    pub fn rstuart0(&mut self) -> RSTUART0_W {
        RSTUART0_W { w: self }
    }
    #[doc = "Bit 4 - UART1 reset control bit."]
    #[inline(always)]
    pub fn rstuart1(&mut self) -> RSTUART1_W {
        RSTUART1_W { w: self }
    }
    #[doc = "Bit 5 - PWM0 reset control bit."]
    #[inline(always)]
    pub fn rstpwm0(&mut self) -> RSTPWM0_W {
        RSTPWM0_W { w: self }
    }
    #[doc = "Bit 6 - PWM1 reset control bit."]
    #[inline(always)]
    pub fn rstpwm1(&mut self) -> RSTPWM1_W {
        RSTPWM1_W { w: self }
    }
    #[doc = "Bit 7 - The I2C0 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c0(&mut self) -> RSTI2C0_W {
        RSTI2C0_W { w: self }
    }
    #[doc = "Bit 8 - UART4 reset control bit."]
    #[inline(always)]
    pub fn rstuart4(&mut self) -> RSTUART4_W {
        RSTUART4_W { w: self }
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 626 \"Register overview: Real-Time Clock (base address 0x4002 4000)\" for details."]
    #[inline(always)]
    pub fn rstrtc(&mut self) -> RSTRTC_W {
        RSTRTC_W { w: self }
    }
    #[doc = "Bit 10 - The SSP 1 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp1(&mut self) -> RSTSSP1_W {
        RSTSSP1_W { w: self }
    }
    #[doc = "Bit 11 - External Memory Controller reset control bit."]
    #[inline(always)]
    pub fn rstemc(&mut self) -> RSTEMC_W {
        RSTEMC_W { w: self }
    }
    #[doc = "Bit 12 - A/D converter (ADC) reset control bit."]
    #[inline(always)]
    pub fn rstadc(&mut self) -> RSTADC_W {
        RSTADC_W { w: self }
    }
    #[doc = "Bit 13 - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan1(&mut self) -> RSTCAN1_W {
        RSTCAN1_W { w: self }
    }
    #[doc = "Bit 14 - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan2(&mut self) -> RSTCAN2_W {
        RSTCAN2_W { w: self }
    }
    #[doc = "Bit 15 - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstgpio(&mut self) -> RSTGPIO_W {
        RSTGPIO_W { w: self }
    }
    #[doc = "Bit 16 - SPI Flash Interface reset control bit."]
    #[inline(always)]
    pub fn rstspifi(&mut self) -> RSTSPIFI_W {
        RSTSPIFI_W { w: self }
    }
    #[doc = "Bit 17 - Motor Control PWM reset control bit."]
    #[inline(always)]
    pub fn rstmcpwm(&mut self) -> RSTMCPWM_W {
        RSTMCPWM_W { w: self }
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface reset control bit."]
    #[inline(always)]
    pub fn rstqei(&mut self) -> RSTQEI_W {
        RSTQEI_W { w: self }
    }
    #[doc = "Bit 19 - The I2C1 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c1(&mut self) -> RSTI2C1_W {
        RSTI2C1_W { w: self }
    }
    #[doc = "Bit 20 - The SSP2 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp2(&mut self) -> RSTSSP2_W {
        RSTSSP2_W { w: self }
    }
    #[doc = "Bit 21 - The SSP0 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp0(&mut self) -> RSTSSP0_W {
        RSTSSP0_W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 reset control bit."]
    #[inline(always)]
    pub fn rsttim2(&mut self) -> RSTTIM2_W {
        RSTTIM2_W { w: self }
    }
    #[doc = "Bit 23 - Timer 3 reset control bit."]
    #[inline(always)]
    pub fn rsttim3(&mut self) -> RSTTIM3_W {
        RSTTIM3_W { w: self }
    }
    #[doc = "Bit 24 - UART 2 reset control bit."]
    #[inline(always)]
    pub fn rstuart2(&mut self) -> RSTUART2_W {
        RSTUART2_W { w: self }
    }
    #[doc = "Bit 25 - UART 3 reset control bit."]
    #[inline(always)]
    pub fn rstuart3(&mut self) -> RSTUART3_W {
        RSTUART3_W { w: self }
    }
    #[doc = "Bit 26 - I2C interface 2 reset control bit."]
    #[inline(always)]
    pub fn rsti2c2(&mut self) -> RSTI2C2_W {
        RSTI2C2_W { w: self }
    }
    #[doc = "Bit 27 - I2S interface reset control bit."]
    #[inline(always)]
    pub fn rsti2s(&mut self) -> RSTI2S_W {
        RSTI2S_W { w: self }
    }
    #[doc = "Bit 28 - SD Card interface reset control bit."]
    #[inline(always)]
    pub fn rstsdc(&mut self) -> RSTSDC_W {
        RSTSDC_W { w: self }
    }
    #[doc = "Bit 29 - GPDMA function reset control bit."]
    #[inline(always)]
    pub fn rstgpdma(&mut self) -> RSTGPDMA_W {
        RSTGPDMA_W { w: self }
    }
    #[doc = "Bit 30 - Ethernet block reset control bit."]
    #[inline(always)]
    pub fn rstenet(&mut self) -> RSTENET_W {
        RSTENET_W { w: self }
    }
    #[doc = "Bit 31 - USB interface reset control bit."]
    #[inline(always)]
    pub fn rstusb(&mut self) -> RSTUSB_W {
        RSTUSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Individual peripheral reset control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcon0](index.html) module"]
pub struct RSTCON0_SPEC;
impl crate::RegisterSpec for RSTCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcon0::R](R) reader structure"]
impl crate::Readable for RSTCON0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcon0::W](W) writer structure"]
impl crate::Writable for RSTCON0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCON0 to value 0"]
impl crate::Resettable for RSTCON0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
