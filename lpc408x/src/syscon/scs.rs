#[doc = "Register `SCS` reader"]
pub struct R(crate::R<SCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCS` writer"]
pub struct W(crate::W<SCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCS_SPEC>;
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
impl From<crate::W<SCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 9.9 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCSC_A {
    #[doc = "0: Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    STATIC_MEMORY_ADDRES = 0,
    #[doc = "1: Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    STATIC_MEMORY_ADDRES = 1,
}
impl From<EMCSC_A> for bool {
    #[inline(always)]
    fn from(variant: EMCSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCSC` reader - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 9.9 in the EMC chapter."]
pub struct EMCSC_R(crate::FieldReader<bool, EMCSC_A>);
impl EMCSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMCSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCSC_A {
        match self.bits {
            false => EMCSC_A::STATIC_MEMORY_ADDRES,
            true => EMCSC_A::STATIC_MEMORY_ADDRES,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC_MEMORY_ADDRES`"]
    #[inline(always)]
    pub fn is_static_memory_addres(&self) -> bool {
        **self == EMCSC_A::STATIC_MEMORY_ADDRES
    }
    #[doc = "Checks if the value of the field is `STATIC_MEMORY_ADDRES`"]
    #[inline(always)]
    pub fn is_static_memory_addres(&self) -> bool {
        **self == EMCSC_A::STATIC_MEMORY_ADDRES
    }
}
impl core::ops::Deref for EMCSC_R {
    type Target = crate::FieldReader<bool, EMCSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCSC` writer - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 9.9 in the EMC chapter."]
pub struct EMCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn static_memory_addres(self) -> &'a mut W {
        self.variant(EMCSC_A::STATIC_MEMORY_ADDRES)
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn static_memory_addres(self) -> &'a mut W {
        self.variant(EMCSC_A::STATIC_MEMORY_ADDRES)
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
#[doc = "EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 9.8 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCRD_A {
    #[doc = "0: Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    BOTH_EMC_RESETS_ARE_ = 0,
    #[doc = "1: Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    MANY_PORTIONS_OF_THE = 1,
}
impl From<EMCRD_A> for bool {
    #[inline(always)]
    fn from(variant: EMCRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCRD` reader - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 9.8 in the EMC chapter."]
pub struct EMCRD_R(crate::FieldReader<bool, EMCRD_A>);
impl EMCRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMCRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCRD_A {
        match self.bits {
            false => EMCRD_A::BOTH_EMC_RESETS_ARE_,
            true => EMCRD_A::MANY_PORTIONS_OF_THE,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH_EMC_RESETS_ARE_`"]
    #[inline(always)]
    pub fn is_both_emc_resets_are_(&self) -> bool {
        **self == EMCRD_A::BOTH_EMC_RESETS_ARE_
    }
    #[doc = "Checks if the value of the field is `MANY_PORTIONS_OF_THE`"]
    #[inline(always)]
    pub fn is_many_portions_of_the(&self) -> bool {
        **self == EMCRD_A::MANY_PORTIONS_OF_THE
    }
}
impl core::ops::Deref for EMCRD_R {
    type Target = crate::FieldReader<bool, EMCRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCRD` writer - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 9.8 in the EMC chapter."]
pub struct EMCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCRD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn both_emc_resets_are_(self) -> &'a mut W {
        self.variant(EMCRD_A::BOTH_EMC_RESETS_ARE_)
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn many_portions_of_the(self) -> &'a mut W {
        self.variant(EMCRD_A::MANY_PORTIONS_OF_THE)
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
#[doc = "External Memory Controller burst control. Also see Section 9.10 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCBC_A {
    #[doc = "0: Burst enabled."]
    BURST_ENABLED_ = 0,
    #[doc = "1: Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BURST_DISABLED_THIS = 1,
}
impl From<EMCBC_A> for bool {
    #[inline(always)]
    fn from(variant: EMCBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCBC` reader - External Memory Controller burst control. Also see Section 9.10 in the EMC chapter."]
pub struct EMCBC_R(crate::FieldReader<bool, EMCBC_A>);
impl EMCBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMCBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCBC_A {
        match self.bits {
            false => EMCBC_A::BURST_ENABLED_,
            true => EMCBC_A::BURST_DISABLED_THIS,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_ENABLED_`"]
    #[inline(always)]
    pub fn is_burst_enabled_(&self) -> bool {
        **self == EMCBC_A::BURST_ENABLED_
    }
    #[doc = "Checks if the value of the field is `BURST_DISABLED_THIS`"]
    #[inline(always)]
    pub fn is_burst_disabled_this(&self) -> bool {
        **self == EMCBC_A::BURST_DISABLED_THIS
    }
}
impl core::ops::Deref for EMCBC_R {
    type Target = crate::FieldReader<bool, EMCBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCBC` writer - External Memory Controller burst control. Also see Section 9.10 in the EMC chapter."]
pub struct EMCBC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCBC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn burst_enabled_(self) -> &'a mut W {
        self.variant(EMCBC_A::BURST_ENABLED_)
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn burst_disabled_this(self) -> &'a mut W {
        self.variant(EMCBC_A::BURST_DISABLED_THIS)
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
#[doc = "MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCIPWRAL_A {
    #[doc = "0: SD_PWR is active low (inverted output of the SD Card interface block)."]
    SD_PWR_IS_ACTIVE_LOW = 0,
    #[doc = "1: SD_PWR is active high (follows the output of the SD Card interface block)."]
    SD_PWR_IS_ACTIVE_HIG = 1,
}
impl From<MCIPWRAL_A> for bool {
    #[inline(always)]
    fn from(variant: MCIPWRAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCIPWRAL` reader - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub struct MCIPWRAL_R(crate::FieldReader<bool, MCIPWRAL_A>);
impl MCIPWRAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCIPWRAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCIPWRAL_A {
        match self.bits {
            false => MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW,
            true => MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `SD_PWR_IS_ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_low(&self) -> bool {
        **self == MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `SD_PWR_IS_ACTIVE_HIG`"]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_hig(&self) -> bool {
        **self == MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG
    }
}
impl core::ops::Deref for MCIPWRAL_R {
    type Target = crate::FieldReader<bool, MCIPWRAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCIPWRAL` writer - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub struct MCIPWRAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCIPWRAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCIPWRAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_low(self) -> &'a mut W {
        self.variant(MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW)
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_hig(self) -> &'a mut W {
        self.variant(MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG)
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
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRS_A {
    #[doc = "0: The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    THE_FREQUENCY_RANGE_ = 0,
    #[doc = "1: The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    THE_FREQUENCY_RANGE_ = 1,
}
impl From<OSCRS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRS` reader - Main oscillator range select."]
pub struct OSCRS_R(crate::FieldReader<bool, OSCRS_A>);
impl OSCRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCRS_A {
        match self.bits {
            false => OSCRS_A::THE_FREQUENCY_RANGE_,
            true => OSCRS_A::THE_FREQUENCY_RANGE_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_FREQUENCY_RANGE_`"]
    #[inline(always)]
    pub fn is_the_frequency_range_(&self) -> bool {
        **self == OSCRS_A::THE_FREQUENCY_RANGE_
    }
    #[doc = "Checks if the value of the field is `THE_FREQUENCY_RANGE_`"]
    #[inline(always)]
    pub fn is_the_frequency_range_(&self) -> bool {
        **self == OSCRS_A::THE_FREQUENCY_RANGE_
    }
}
impl core::ops::Deref for OSCRS_R {
    type Target = crate::FieldReader<bool, OSCRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCRS` writer - Main oscillator range select."]
pub struct OSCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn the_frequency_range_(self) -> &'a mut W {
        self.variant(OSCRS_A::THE_FREQUENCY_RANGE_)
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn the_frequency_range_(self) -> &'a mut W {
        self.variant(OSCRS_A::THE_FREQUENCY_RANGE_)
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
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCEN_A {
    #[doc = "0: The main oscillator is disabled."]
    THE_MAIN_OSCILLATOR_ = 0,
    #[doc = "1: The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    THE_MAIN_OSCILLATOR_ = 1,
}
impl From<OSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub struct OSCEN_R(crate::FieldReader<bool, OSCEN_A>);
impl OSCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCEN_A {
        match self.bits {
            false => OSCEN_A::THE_MAIN_OSCILLATOR_,
            true => OSCEN_A::THE_MAIN_OSCILLATOR_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MAIN_OSCILLATOR_`"]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        **self == OSCEN_A::THE_MAIN_OSCILLATOR_
    }
    #[doc = "Checks if the value of the field is `THE_MAIN_OSCILLATOR_`"]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        **self == OSCEN_A::THE_MAIN_OSCILLATOR_
    }
}
impl core::ops::Deref for OSCEN_R {
    type Target = crate::FieldReader<bool, OSCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub struct OSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut W {
        self.variant(OSCEN_A::THE_MAIN_OSCILLATOR_)
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut W {
        self.variant(OSCEN_A::THE_MAIN_OSCILLATOR_)
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
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTAT_A {
    #[doc = "0: The main oscillator is not ready to be used as a clock source."]
    THE_MAIN_OSCILLATOR_ = 0,
    #[doc = "1: The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    THE_MAIN_OSCILLATOR_ = 1,
}
impl From<OSCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub struct OSCSTAT_R(crate::FieldReader<bool, OSCSTAT_A>);
impl OSCSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSTAT_A {
        match self.bits {
            false => OSCSTAT_A::THE_MAIN_OSCILLATOR_,
            true => OSCSTAT_A::THE_MAIN_OSCILLATOR_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_MAIN_OSCILLATOR_`"]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        **self == OSCSTAT_A::THE_MAIN_OSCILLATOR_
    }
    #[doc = "Checks if the value of the field is `THE_MAIN_OSCILLATOR_`"]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        **self == OSCSTAT_A::THE_MAIN_OSCILLATOR_
    }
}
impl core::ops::Deref for OSCSTAT_R {
    type Target = crate::FieldReader<bool, OSCSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub struct OSCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSTAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut W {
        self.variant(OSCSTAT_A::THE_MAIN_OSCILLATOR_)
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut W {
        self.variant(OSCSTAT_A::THE_MAIN_OSCILLATOR_)
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
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 9.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 9.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 9.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&self) -> MCIPWRAL_R {
        MCIPWRAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&self) -> OSCRS_R {
        OSCRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OSCSTAT_R {
        OSCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 9.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EMCSC_W {
        EMCSC_W { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 9.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EMCRD_W {
        EMCRD_W { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 9.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EMCBC_W {
        EMCBC_W { w: self }
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&mut self) -> MCIPWRAL_W {
        MCIPWRAL_W { w: self }
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&mut self) -> OSCRS_W {
        OSCRS_W { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> OSCEN_W {
        OSCEN_W { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> OSCSTAT_W {
        OSCSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs](index.html) module"]
pub struct SCS_SPEC;
impl crate::RegisterSpec for SCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scs::R](R) reader structure"]
impl crate::Readable for SCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scs::W](W) writer structure"]
impl crate::Writable for SCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCS to value 0"]
impl crate::Resettable for SCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
