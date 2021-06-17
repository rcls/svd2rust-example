#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_EN` reader - Comparator 1 enable bit"]
pub struct COMP1_EN_R(crate::FieldReader<bool, bool>);
impl COMP1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_EN` writer - Comparator 1 enable bit"]
pub struct COMP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_EN_W<'a> {
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
#[doc = "Field `COMP1_INN_SEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1_INN_SEL_R(crate::FieldReader<u8, u8>);
impl COMP1_INN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_INN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_INN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_INN_SEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub struct COMP1_INN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_INN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `COMP1_WM` reader - Comparator 1 window mode selection bit"]
pub struct COMP1_WM_R(crate::FieldReader<bool, bool>);
impl COMP1_WM_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_WM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_WM` writer - Comparator 1 window mode selection bit"]
pub struct COMP1_WM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_WM_W<'a> {
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
#[doc = "Field `COMP1_OUT_SEL` reader - COMP1 output select"]
pub struct COMP1_OUT_SEL_R(crate::FieldReader<u8, u8>);
impl COMP1_OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP1_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_OUT_SEL` writer - COMP1 output select"]
pub struct COMP1_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `COMP1_POLARITY` reader - Comparator 1 polarity selection bit"]
pub struct COMP1_POLARITY_R(crate::FieldReader<bool, bool>);
impl COMP1_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_POLARITY` writer - Comparator 1 polarity selection bit"]
pub struct COMP1_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_POLARITY_W<'a> {
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
#[doc = "Field `COMP1_VALUE` reader - Comparator 1 output status bit"]
pub struct COMP1_VALUE_R(crate::FieldReader<bool, bool>);
impl COMP1_VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_LOCK` writer - COMP1_CSR register lock bit"]
pub struct COMP1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1_inn_sel(&self) -> COMP1_INN_SEL_R {
        COMP1_INN_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1_wm(&self) -> COMP1_WM_R {
        COMP1_WM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - COMP1 output select"]
    #[inline(always)]
    pub fn comp1_out_sel(&self) -> COMP1_OUT_SEL_R {
        COMP1_OUT_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1_en(&mut self) -> COMP1_EN_W {
        COMP1_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1_inn_sel(&mut self) -> COMP1_INN_SEL_W {
        COMP1_INN_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1_wm(&mut self) -> COMP1_WM_W {
        COMP1_WM_W { w: self }
    }
    #[doc = "Bits 12:14 - COMP1 output select"]
    #[inline(always)]
    pub fn comp1_out_sel(&mut self) -> COMP1_OUT_SEL_W {
        COMP1_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W {
        COMP1_POLARITY_W { w: self }
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W {
        COMP1_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
