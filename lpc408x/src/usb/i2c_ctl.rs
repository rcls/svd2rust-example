#[doc = "Register `I2C_CTL` reader"]
pub struct R(crate::R<I2C_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTL` writer"]
pub struct W(crate::W<I2C_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTL_SPEC>;
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
impl From<crate::W<I2C_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIE_A {
    #[doc = "0: Disable the TDI interrupt."]
    DISABLE_THE_TDI_INTE = 0,
    #[doc = "1: Enable the TDI interrupt."]
    ENABLE_THE_TDI_INTER = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIE` reader - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub struct TDIE_R(crate::FieldReader<bool, TDIE_A>);
impl TDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::DISABLE_THE_TDI_INTE,
            true => TDIE_A::ENABLE_THE_TDI_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_TDI_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_tdi_inte(&self) -> bool {
        **self == TDIE_A::DISABLE_THE_TDI_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_TDI_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_tdi_inter(&self) -> bool {
        **self == TDIE_A::ENABLE_THE_TDI_INTER
    }
}
impl core::ops::Deref for TDIE_R {
    type Target = crate::FieldReader<bool, TDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIE` writer - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub struct TDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn disable_the_tdi_inte(self) -> &'a mut W {
        self.variant(TDIE_A::DISABLE_THE_TDI_INTE)
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn enable_the_tdi_inter(self) -> &'a mut W {
        self.variant(TDIE_A::ENABLE_THE_TDI_INTER)
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
#[doc = "Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIE_A {
    #[doc = "0: Disable the AFI."]
    DISABLE_THE_AFI_ = 0,
    #[doc = "1: Enable the AFI."]
    ENABLE_THE_AFI_ = 1,
}
impl From<AFIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFIE` reader - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub struct AFIE_R(crate::FieldReader<bool, AFIE_A>);
impl AFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFIE_A {
        match self.bits {
            false => AFIE_A::DISABLE_THE_AFI_,
            true => AFIE_A::ENABLE_THE_AFI_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_AFI_`"]
    #[inline(always)]
    pub fn is_disable_the_afi_(&self) -> bool {
        **self == AFIE_A::DISABLE_THE_AFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_AFI_`"]
    #[inline(always)]
    pub fn is_enable_the_afi_(&self) -> bool {
        **self == AFIE_A::ENABLE_THE_AFI_
    }
}
impl core::ops::Deref for AFIE_R {
    type Target = crate::FieldReader<bool, AFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFIE` writer - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub struct AFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn disable_the_afi_(self) -> &'a mut W {
        self.variant(AFIE_A::DISABLE_THE_AFI_)
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn enable_the_afi_(self) -> &'a mut W {
        self.variant(AFIE_A::ENABLE_THE_AFI_)
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
#[doc = "Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIE_A {
    #[doc = "0: Disable the NAI."]
    DISABLE_THE_NAI_ = 0,
    #[doc = "1: Enable the NAI."]
    ENABLE_THE_NAI_ = 1,
}
impl From<NAIE_A> for bool {
    #[inline(always)]
    fn from(variant: NAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAIE` reader - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub struct NAIE_R(crate::FieldReader<bool, NAIE_A>);
impl NAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAIE_A {
        match self.bits {
            false => NAIE_A::DISABLE_THE_NAI_,
            true => NAIE_A::ENABLE_THE_NAI_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_NAI_`"]
    #[inline(always)]
    pub fn is_disable_the_nai_(&self) -> bool {
        **self == NAIE_A::DISABLE_THE_NAI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_NAI_`"]
    #[inline(always)]
    pub fn is_enable_the_nai_(&self) -> bool {
        **self == NAIE_A::ENABLE_THE_NAI_
    }
}
impl core::ops::Deref for NAIE_R {
    type Target = crate::FieldReader<bool, NAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAIE` writer - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub struct NAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn disable_the_nai_(self) -> &'a mut W {
        self.variant(NAIE_A::DISABLE_THE_NAI_)
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn enable_the_nai_(self) -> &'a mut W {
        self.variant(NAIE_A::ENABLE_THE_NAI_)
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
#[doc = "Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIE_A {
    #[doc = "0: Disable the DRMI interrupt."]
    DISABLE_THE_DRMI_INT = 0,
    #[doc = "1: Enable the DRMI interrupt."]
    ENABLE_THE_DRMI_INTE = 1,
}
impl From<DRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DRMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRMIE` reader - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub struct DRMIE_R(crate::FieldReader<bool, DRMIE_A>);
impl DRMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRMIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRMIE_A {
        match self.bits {
            false => DRMIE_A::DISABLE_THE_DRMI_INT,
            true => DRMIE_A::ENABLE_THE_DRMI_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_DRMI_INT`"]
    #[inline(always)]
    pub fn is_disable_the_drmi_int(&self) -> bool {
        **self == DRMIE_A::DISABLE_THE_DRMI_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DRMI_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_drmi_inte(&self) -> bool {
        **self == DRMIE_A::ENABLE_THE_DRMI_INTE
    }
}
impl core::ops::Deref for DRMIE_R {
    type Target = crate::FieldReader<bool, DRMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRMIE` writer - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub struct DRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn disable_the_drmi_int(self) -> &'a mut W {
        self.variant(DRMIE_A::DISABLE_THE_DRMI_INT)
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn enable_the_drmi_inte(self) -> &'a mut W {
        self.variant(DRMIE_A::ENABLE_THE_DRMI_INTE)
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
#[doc = "Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIE_A {
    #[doc = "0: Disable the DRSI interrupt."]
    DISABLE_THE_DRSI_INT = 0,
    #[doc = "1: Enable the DRSI interrupt."]
    ENABLE_THE_DRSI_INTE = 1,
}
impl From<DRSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DRSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRSIE` reader - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub struct DRSIE_R(crate::FieldReader<bool, DRSIE_A>);
impl DRSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRSIE_A {
        match self.bits {
            false => DRSIE_A::DISABLE_THE_DRSI_INT,
            true => DRSIE_A::ENABLE_THE_DRSI_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_DRSI_INT`"]
    #[inline(always)]
    pub fn is_disable_the_drsi_int(&self) -> bool {
        **self == DRSIE_A::DISABLE_THE_DRSI_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DRSI_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_drsi_inte(&self) -> bool {
        **self == DRSIE_A::ENABLE_THE_DRSI_INTE
    }
}
impl core::ops::Deref for DRSIE_R {
    type Target = crate::FieldReader<bool, DRSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRSIE` writer - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub struct DRSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn disable_the_drsi_int(self) -> &'a mut W {
        self.variant(DRSIE_A::DISABLE_THE_DRSI_INT)
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn enable_the_drsi_inte(self) -> &'a mut W {
        self.variant(DRSIE_A::ENABLE_THE_DRSI_INTE)
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
#[doc = "Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFIE_A {
    #[doc = "0: Disable the RFFI."]
    DISABLE_THE_RFFI_ = 0,
    #[doc = "1: Enable the RFFI."]
    ENABLE_THE_RFFI_ = 1,
}
impl From<REFIE_A> for bool {
    #[inline(always)]
    fn from(variant: REFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFIE` reader - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub struct REFIE_R(crate::FieldReader<bool, REFIE_A>);
impl REFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFIE_A {
        match self.bits {
            false => REFIE_A::DISABLE_THE_RFFI_,
            true => REFIE_A::ENABLE_THE_RFFI_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RFFI_`"]
    #[inline(always)]
    pub fn is_disable_the_rffi_(&self) -> bool {
        **self == REFIE_A::DISABLE_THE_RFFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RFFI_`"]
    #[inline(always)]
    pub fn is_enable_the_rffi_(&self) -> bool {
        **self == REFIE_A::ENABLE_THE_RFFI_
    }
}
impl core::ops::Deref for REFIE_R {
    type Target = crate::FieldReader<bool, REFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFIE` writer - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub struct REFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn disable_the_rffi_(self) -> &'a mut W {
        self.variant(REFIE_A::DISABLE_THE_RFFI_)
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn enable_the_rffi_(self) -> &'a mut W {
        self.variant(REFIE_A::ENABLE_THE_RFFI_)
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
#[doc = "Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDAIE_A {
    #[doc = "0: Disable the DAI."]
    DISABLE_THE_DAI_ = 0,
    #[doc = "1: Enable the DAI."]
    ENABLE_THE_DAI_ = 1,
}
impl From<RFDAIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFDAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDAIE` reader - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub struct RFDAIE_R(crate::FieldReader<bool, RFDAIE_A>);
impl RFDAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFDAIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDAIE_A {
        match self.bits {
            false => RFDAIE_A::DISABLE_THE_DAI_,
            true => RFDAIE_A::ENABLE_THE_DAI_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_DAI_`"]
    #[inline(always)]
    pub fn is_disable_the_dai_(&self) -> bool {
        **self == RFDAIE_A::DISABLE_THE_DAI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DAI_`"]
    #[inline(always)]
    pub fn is_enable_the_dai_(&self) -> bool {
        **self == RFDAIE_A::ENABLE_THE_DAI_
    }
}
impl core::ops::Deref for RFDAIE_R {
    type Target = crate::FieldReader<bool, RFDAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFDAIE` writer - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub struct RFDAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn disable_the_dai_(self) -> &'a mut W {
        self.variant(RFDAIE_A::DISABLE_THE_DAI_)
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn enable_the_dai_(self) -> &'a mut W {
        self.variant(RFDAIE_A::ENABLE_THE_DAI_)
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
#[doc = "Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFIE_A {
    #[doc = "0: Disable the TFFI."]
    DISABLE_THE_TFFI_ = 0,
    #[doc = "1: Enable the TFFI."]
    ENABLE_THE_TFFI_ = 1,
}
impl From<TFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFIE` reader - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub struct TFFIE_R(crate::FieldReader<bool, TFFIE_A>);
impl TFFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFIE_A {
        match self.bits {
            false => TFFIE_A::DISABLE_THE_TFFI_,
            true => TFFIE_A::ENABLE_THE_TFFI_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_TFFI_`"]
    #[inline(always)]
    pub fn is_disable_the_tffi_(&self) -> bool {
        **self == TFFIE_A::DISABLE_THE_TFFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_TFFI_`"]
    #[inline(always)]
    pub fn is_enable_the_tffi_(&self) -> bool {
        **self == TFFIE_A::ENABLE_THE_TFFI_
    }
}
impl core::ops::Deref for TFFIE_R {
    type Target = crate::FieldReader<bool, TFFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFFIE` writer - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub struct TFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn disable_the_tffi_(self) -> &'a mut W {
        self.variant(TFFIE_A::DISABLE_THE_TFFI_)
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn enable_the_tffi_(self) -> &'a mut W {
        self.variant(TFFIE_A::ENABLE_THE_TFFI_)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_A {
    #[doc = "0: See the text."]
    SEE_THE_TEXT_ = 0,
    #[doc = "1: Reset the I2C to idle state. Self clearing."]
    RESET_THE_I2C_TO_IDL = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRST` reader - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub struct SRST_R(crate::FieldReader<bool, SRST_A>);
impl SRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::SEE_THE_TEXT_,
            true => SRST_A::RESET_THE_I2C_TO_IDL,
        }
    }
    #[doc = "Checks if the value of the field is `SEE_THE_TEXT_`"]
    #[inline(always)]
    pub fn is_see_the_text_(&self) -> bool {
        **self == SRST_A::SEE_THE_TEXT_
    }
    #[doc = "Checks if the value of the field is `RESET_THE_I2C_TO_IDL`"]
    #[inline(always)]
    pub fn is_reset_the_i2c_to_idl(&self) -> bool {
        **self == SRST_A::RESET_THE_I2C_TO_IDL
    }
}
impl core::ops::Deref for SRST_R {
    type Target = crate::FieldReader<bool, SRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRST` writer - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "See the text."]
    #[inline(always)]
    pub fn see_the_text_(self) -> &'a mut W {
        self.variant(SRST_A::SEE_THE_TEXT_)
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn reset_the_i2c_to_idl(self) -> &'a mut W {
        self.variant(SRST_A::RESET_THE_I2C_TO_IDL)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&self) -> AFIE_R {
        AFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&self) -> NAIE_R {
        NAIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&self) -> DRMIE_R {
        DRMIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&self) -> DRSIE_R {
        DRSIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&self) -> REFIE_R {
        REFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&self) -> RFDAIE_R {
        RFDAIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&mut self) -> TDIE_W {
        TDIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&mut self) -> AFIE_W {
        AFIE_W { w: self }
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&mut self) -> NAIE_W {
        NAIE_W { w: self }
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&mut self) -> DRMIE_W {
        DRMIE_W { w: self }
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&mut self) -> DRSIE_W {
        DRSIE_W { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&mut self) -> REFIE_W {
        REFIE_W { w: self }
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&mut self) -> RFDAIE_W {
        RFDAIE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&mut self) -> TFFIE_W {
        TFFIE_W { w: self }
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctl](index.html) module"]
pub struct I2C_CTL_SPEC;
impl crate::RegisterSpec for I2C_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctl::R](R) reader structure"]
impl crate::Readable for I2C_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctl::W](W) writer structure"]
impl crate::Writable for I2C_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTL to value 0"]
impl crate::Resettable for I2C_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
