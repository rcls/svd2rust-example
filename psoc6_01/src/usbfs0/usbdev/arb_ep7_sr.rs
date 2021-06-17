#[doc = "Register `ARB_EP7_SR` reader"]
pub struct R(crate::R<ARB_EP7_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_EP7_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_EP7_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_EP7_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_EP7_SR` writer"]
pub struct W(crate::W<ARB_EP7_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_EP7_SR_SPEC>;
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
impl From<crate::W<ARB_EP7_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_EP7_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_BUF_FULL` reader - IN Endpoint Local Buffer Full Interrupt"]
pub struct IN_BUF_FULL_R(crate::FieldReader<bool, bool>);
impl IN_BUF_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_BUF_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_BUF_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_BUF_FULL` writer - IN Endpoint Local Buffer Full Interrupt"]
pub struct IN_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BUF_FULL_W<'a> {
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
#[doc = "Field `DMA_GNT` reader - Endpoint DMA Grant Interrupt"]
pub struct DMA_GNT_R(crate::FieldReader<bool, bool>);
impl DMA_GNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_GNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_GNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_GNT` writer - Endpoint DMA Grant Interrupt"]
pub struct DMA_GNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_GNT_W<'a> {
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
#[doc = "Field `BUF_OVER` reader - Endpoint Buffer Overflow Interrupt"]
pub struct BUF_OVER_R(crate::FieldReader<bool, bool>);
impl BUF_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_OVER` writer - Endpoint Buffer Overflow Interrupt"]
pub struct BUF_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVER_W<'a> {
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
#[doc = "Field `BUF_UNDER` reader - Endpoint Buffer Underflow Interrupt"]
pub struct BUF_UNDER_R(crate::FieldReader<bool, bool>);
impl BUF_UNDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUF_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF_UNDER` writer - Endpoint Buffer Underflow Interrupt"]
pub struct BUF_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_UNDER_W<'a> {
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
#[doc = "Field `DMA_TERMIN` reader - Endpoint DMA Terminated Interrupt"]
pub struct DMA_TERMIN_R(crate::FieldReader<bool, bool>);
impl DMA_TERMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_TERMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_TERMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_TERMIN` writer - Endpoint DMA Terminated Interrupt"]
pub struct DMA_TERMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TERMIN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&self) -> IN_BUF_FULL_R {
        IN_BUF_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&self) -> DMA_GNT_R {
        DMA_GNT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&self) -> BUF_OVER_R {
        BUF_OVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&self) -> BUF_UNDER_R {
        BUF_UNDER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&self) -> DMA_TERMIN_R {
        DMA_TERMIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&mut self) -> IN_BUF_FULL_W {
        IN_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&mut self) -> DMA_GNT_W {
        DMA_GNT_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&mut self) -> BUF_OVER_W {
        BUF_OVER_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&mut self) -> BUF_UNDER_W {
        BUF_UNDER_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&mut self) -> DMA_TERMIN_W {
        DMA_TERMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep7_sr](index.html) module"]
pub struct ARB_EP7_SR_SPEC;
impl crate::RegisterSpec for ARB_EP7_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_ep7_sr::R](R) reader structure"]
impl crate::Readable for ARB_EP7_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_ep7_sr::W](W) writer structure"]
impl crate::Writable for ARB_EP7_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_EP7_SR to value 0"]
impl crate::Resettable for ARB_EP7_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
