#[doc = "Register `AHBSMENR` reader"]
pub struct R(crate::R<AHBSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBSMENR` writer"]
pub struct W(crate::W<AHBSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBSMENR_SPEC>;
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
impl From<crate::W<AHBSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBSMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPSMEN` reader - Crypto clock enable during sleep mode bit"]
pub struct CRYPSMEN_R(crate::FieldReader<bool, bool>);
impl CRYPSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYPSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPSMEN` writer - Crypto clock enable during sleep mode bit"]
pub struct CRYPSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPSMEN_W<'a> {
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
#[doc = "Field `RNGSMEN` reader - Random Number Generator clock enable during sleep mode bit"]
pub struct RNGSMEN_R(crate::FieldReader<bool, bool>);
impl RNGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGSMEN` writer - Random Number Generator clock enable during sleep mode bit"]
pub struct RNGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSMEN_W<'a> {
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
#[doc = "Field `TOUCHSMEN` reader - Touch Sensing clock enable during sleep mode bit"]
pub struct TOUCHSMEN_R(crate::FieldReader<bool, bool>);
impl TOUCHSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCHSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCHSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCHSMEN` writer - Touch Sensing clock enable during sleep mode bit"]
pub struct TOUCHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCHSMEN_W<'a> {
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
#[doc = "Field `CRCSMEN` reader - CRC clock enable during sleep mode bit"]
pub struct CRCSMEN_R(crate::FieldReader<bool, bool>);
impl CRCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clock enable during sleep mode bit"]
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
#[doc = "Field `SRAMSMEN` reader - SRAM interface clock enable during sleep mode bit"]
pub struct SRAMSMEN_R(crate::FieldReader<bool, bool>);
impl SRAMSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAMSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAMSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMSMEN` writer - SRAM interface clock enable during sleep mode bit"]
pub struct SRAMSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMSMEN_W<'a> {
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
#[doc = "Field `MIFSMEN` reader - NVM interface clock enable during sleep mode bit"]
pub struct MIFSMEN_R(crate::FieldReader<bool, bool>);
impl MIFSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIFSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIFSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIFSMEN` writer - NVM interface clock enable during sleep mode bit"]
pub struct MIFSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIFSMEN_W<'a> {
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
#[doc = "Field `DMASMEN` reader - DMA clock enable during sleep mode bit"]
pub struct DMASMEN_R(crate::FieldReader<bool, bool>);
impl DMASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASMEN` writer - DMA clock enable during sleep mode bit"]
pub struct DMASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASMEN_W<'a> {
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
impl R {
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crypsmen(&self) -> CRYPSMEN_R {
        CRYPSMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Random Number Generator clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn touchsmen(&self) -> TOUCHSMEN_R {
        TOUCHSMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&self) -> MIFSMEN_R {
        MIFSMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crypsmen(&mut self) -> CRYPSMEN_W {
        CRYPSMEN_W { w: self }
    }
    #[doc = "Bit 20 - Random Number Generator clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W {
        RNGSMEN_W { w: self }
    }
    #[doc = "Bit 16 - Touch Sensing clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn touchsmen(&mut self) -> TOUCHSMEN_W {
        TOUCHSMEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W {
        SRAMSMEN_W { w: self }
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&mut self) -> MIFSMEN_W {
        MIFSMEN_W { w: self }
    }
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DMASMEN_W {
        DMASMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbsmenr](index.html) module"]
pub struct AHBSMENR_SPEC;
impl crate::RegisterSpec for AHBSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbsmenr::R](R) reader structure"]
impl crate::Readable for AHBSMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbsmenr::W](W) writer structure"]
impl crate::Writable for AHBSMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBSMENR to value 0x0111_1301"]
impl crate::Resettable for AHBSMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0111_1301
    }
}
