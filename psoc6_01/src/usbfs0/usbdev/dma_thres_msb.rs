#[doc = "Register `DMA_THRES_MSB` reader"]
pub struct R(crate::R<DMA_THRES_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_THRES_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_THRES_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_THRES_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_THRES_MSB` writer"]
pub struct W(crate::W<DMA_THRES_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_THRES_MSB_SPEC>;
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
impl From<crate::W<DMA_THRES_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_THRES_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_THS_MSB` reader - DMA Threshold count"]
pub struct DMA_THS_MSB_R(crate::FieldReader<bool, bool>);
impl DMA_THS_MSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_THS_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_THS_MSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_THS_MSB` writer - DMA Threshold count"]
pub struct DMA_THS_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_THS_MSB_W<'a> {
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
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths_msb(&self) -> DMA_THS_MSB_R {
        DMA_THS_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths_msb(&mut self) -> DMA_THS_MSB_W {
        DMA_THS_MSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Burst / Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_thres_msb](index.html) module"]
pub struct DMA_THRES_MSB_SPEC;
impl crate::RegisterSpec for DMA_THRES_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_thres_msb::R](R) reader structure"]
impl crate::Readable for DMA_THRES_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_thres_msb::W](W) writer structure"]
impl crate::Writable for DMA_THRES_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_THRES_MSB to value 0"]
impl crate::Resettable for DMA_THRES_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
