#[doc = "Register `DMACREQSEL` reader"]
pub struct R(crate::R<DMACREQSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACREQSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACREQSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACREQSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACREQSEL` writer"]
pub struct W(crate::W<DMACREQSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACREQSEL_SPEC>;
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
impl From<crate::W<DMACREQSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACREQSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASEL00` reader - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
pub struct DMASEL00_R(crate::FieldReader<bool, bool>);
impl DMASEL00_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL00_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL00` writer - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
pub struct DMASEL00_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL00_W<'a> {
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
#[doc = "Field `DMASEL01` reader - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
pub struct DMASEL01_R(crate::FieldReader<bool, bool>);
impl DMASEL01_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL01` writer - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
pub struct DMASEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL01_W<'a> {
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
#[doc = "Field `DMASEL02` reader - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
pub struct DMASEL02_R(crate::FieldReader<bool, bool>);
impl DMASEL02_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL02_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL02_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL02` writer - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
pub struct DMASEL02_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL02_W<'a> {
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
#[doc = "Field `DMASEL03` reader - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
pub struct DMASEL03_R(crate::FieldReader<bool, bool>);
impl DMASEL03_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL03_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL03_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL03` writer - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
pub struct DMASEL03_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL03_W<'a> {
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
#[doc = "Field `DMASEL04` reader - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
pub struct DMASEL04_R(crate::FieldReader<bool, bool>);
impl DMASEL04_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL04_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL04_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL04` writer - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
pub struct DMASEL04_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL04_W<'a> {
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
#[doc = "Field `DMASEL05` reader - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
pub struct DMASEL05_R(crate::FieldReader<bool, bool>);
impl DMASEL05_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL05_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL05_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL05` writer - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
pub struct DMASEL05_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL05_W<'a> {
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
#[doc = "Field `DMASEL06` reader - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
pub struct DMASEL06_R(crate::FieldReader<bool, bool>);
impl DMASEL06_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL06_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL06_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL06` writer - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
pub struct DMASEL06_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL06_W<'a> {
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
#[doc = "Field `DMASEL07` reader - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
pub struct DMASEL07_R(crate::FieldReader<bool, bool>);
impl DMASEL07_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL07_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL07_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL07` writer - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
pub struct DMASEL07_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL07_W<'a> {
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
#[doc = "Field `DMASEL10` reader - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
pub struct DMASEL10_R(crate::FieldReader<bool, bool>);
impl DMASEL10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL10` writer - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
pub struct DMASEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL10_W<'a> {
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
#[doc = "Field `DMASEL11` reader - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
pub struct DMASEL11_R(crate::FieldReader<bool, bool>);
impl DMASEL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL11` writer - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
pub struct DMASEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL11_W<'a> {
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
#[doc = "Field `DMASEL12` reader - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
pub struct DMASEL12_R(crate::FieldReader<bool, bool>);
impl DMASEL12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL12` writer - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
pub struct DMASEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL12_W<'a> {
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
#[doc = "Field `DMASEL13` reader - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
pub struct DMASEL13_R(crate::FieldReader<bool, bool>);
impl DMASEL13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL13` writer - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
pub struct DMASEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL13_W<'a> {
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
#[doc = "Field `DMASEL14` reader - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
pub struct DMASEL14_R(crate::FieldReader<bool, bool>);
impl DMASEL14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL14` writer - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
pub struct DMASEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL14_W<'a> {
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
#[doc = "Field `DMASEL15` reader - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
pub struct DMASEL15_R(crate::FieldReader<bool, bool>);
impl DMASEL15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASEL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL15` writer - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
pub struct DMASEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL15_W<'a> {
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
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&self) -> DMASEL00_R {
        DMASEL00_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&self) -> DMASEL01_R {
        DMASEL01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&self) -> DMASEL02_R {
        DMASEL02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&self) -> DMASEL03_R {
        DMASEL03_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&self) -> DMASEL04_R {
        DMASEL04_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&self) -> DMASEL05_R {
        DMASEL05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&self) -> DMASEL06_R {
        DMASEL06_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&self) -> DMASEL07_R {
        DMASEL07_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> DMASEL10_R {
        DMASEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> DMASEL11_R {
        DMASEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> DMASEL12_R {
        DMASEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> DMASEL13_R {
        DMASEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> DMASEL14_R {
        DMASEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> DMASEL15_R {
        DMASEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&mut self) -> DMASEL00_W {
        DMASEL00_W { w: self }
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&mut self) -> DMASEL01_W {
        DMASEL01_W { w: self }
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&mut self) -> DMASEL02_W {
        DMASEL02_W { w: self }
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&mut self) -> DMASEL03_W {
        DMASEL03_W { w: self }
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&mut self) -> DMASEL04_W {
        DMASEL04_W { w: self }
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&mut self) -> DMASEL05_W {
        DMASEL05_W { w: self }
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&mut self) -> DMASEL06_W {
        DMASEL06_W { w: self }
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&mut self) -> DMASEL07_W {
        DMASEL07_W { w: self }
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&mut self) -> DMASEL10_W {
        DMASEL10_W { w: self }
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&mut self) -> DMASEL11_W {
        DMASEL11_W { w: self }
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&mut self) -> DMASEL12_W {
        DMASEL12_W { w: self }
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&mut self) -> DMASEL13_W {
        DMASEL13_W { w: self }
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&mut self) -> DMASEL14_W {
        DMASEL14_W { w: self }
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&mut self) -> DMASEL15_W {
        DMASEL15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacreqsel](index.html) module"]
pub struct DMACREQSEL_SPEC;
impl crate::RegisterSpec for DMACREQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacreqsel::R](R) reader structure"]
impl crate::Readable for DMACREQSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacreqsel::W](W) writer structure"]
impl crate::Writable for DMACREQSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACREQSEL to value 0"]
impl crate::Resettable for DMACREQSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
