#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2_LOCK` writer - COMP2_CSR register lock bit"]
pub struct COMP2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_LOCK_W<'a> {
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
#[doc = "Field `COMP2_VALUE` reader - Comparator 2 output status bit"]
pub struct COMP2_VALUE_R(crate::FieldReader<bool, bool>);
impl COMP2_VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit"]
pub struct COMP2_POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP2_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit"]
pub struct COMP2_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_POLARITY_W<'a> {
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
#[doc = "Field `COMP2_INP_SEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2_INP_SEL_R(crate::FieldReader<u8, u8>);
impl COMP2_INP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_INP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_INP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_INP_SEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub struct COMP2_INP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `COMP2_INN_SEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2_INN_SEL_R(crate::FieldReader<u8, u8>);
impl COMP2_INN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_INN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_INN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_INN_SEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub struct COMP2_INN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `COMP2_SPEED` reader - Comparator 2 power mode selection bit"]
pub struct COMP2_SPEED_R(crate::FieldReader<bool, bool>);
impl COMP2_SPEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_SPEED` writer - Comparator 2 power mode selection bit"]
pub struct COMP2_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_SPEED_W<'a> {
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
#[doc = "Field `COMP2_EN` reader - Comparator 2 enable bit"]
pub struct COMP2_EN_R(crate::FieldReader<bool, bool>);
impl COMP2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_EN` writer - Comparator 2 enable bit"]
pub struct COMP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_EN_W<'a> {
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
#[doc = "Field `COMP2_OUT_SEL` reader - COMP2 output select"]
pub struct COMP2_OUT_SEL_R(crate::FieldReader<u8, u8>);
impl COMP2_OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_OUT_SEL` writer - COMP2 output select"]
pub struct COMP2_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inp_sel(&self) -> COMP2_INP_SEL_R {
        COMP2_INP_SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inn_sel(&self) -> COMP2_INN_SEL_R {
        COMP2_INN_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2_speed(&self) -> COMP2_SPEED_R {
        COMP2_SPEED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - COMP2 output select"]
    #[inline(always)]
    pub fn comp2_out_sel(&self) -> COMP2_OUT_SEL_R {
        COMP2_OUT_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W {
        COMP2_LOCK_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W {
        COMP2_POLARITY_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inp_sel(&mut self) -> COMP2_INP_SEL_W {
        COMP2_INP_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inn_sel(&mut self) -> COMP2_INN_SEL_W {
        COMP2_INN_SEL_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2_speed(&mut self) -> COMP2_SPEED_W {
        COMP2_SPEED_W { w: self }
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&mut self) -> COMP2_EN_W {
        COMP2_EN_W { w: self }
    }
    #[doc = "Bits 12:14 - COMP2 output select"]
    #[inline(always)]
    pub fn comp2_out_sel(&mut self) -> COMP2_OUT_SEL_W {
        COMP2_OUT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
