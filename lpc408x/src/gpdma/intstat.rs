#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTAT0` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT0_R(crate::FieldReader<bool, bool>);
impl INTSTAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT1` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT1_R(crate::FieldReader<bool, bool>);
impl INTSTAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT2` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT2_R(crate::FieldReader<bool, bool>);
impl INTSTAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT3` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT3_R(crate::FieldReader<bool, bool>);
impl INTSTAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT4` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT4_R(crate::FieldReader<bool, bool>);
impl INTSTAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT5` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT5_R(crate::FieldReader<bool, bool>);
impl INTSTAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT6` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT6_R(crate::FieldReader<bool, bool>);
impl INTSTAT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTSTAT7` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub struct INTSTAT7_R(crate::FieldReader<bool, bool>);
impl INTSTAT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTAT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat0(&self) -> INTSTAT0_R {
        INTSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat1(&self) -> INTSTAT1_R {
        INTSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat2(&self) -> INTSTAT2_R {
        INTSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat3(&self) -> INTSTAT3_R {
        INTSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat4(&self) -> INTSTAT4_R {
        INTSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat5(&self) -> INTSTAT5_R {
        INTSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat6(&self) -> INTSTAT6_R {
        INTSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat7(&self) -> INTSTAT7_R {
        INTSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
