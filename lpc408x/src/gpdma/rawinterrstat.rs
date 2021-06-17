#[doc = "Register `RAWINTERRSTAT` reader"]
pub struct R(crate::R<RAWINTERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAWINTERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAWINTERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAWINTERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAWINTERRSTAT0` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT0_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT1` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT1_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT2` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT2_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT3` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT3_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT4` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT4_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT5` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT5_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT6` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT6_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTERRSTAT7` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct RAWINTERRSTAT7_R(crate::FieldReader<bool, bool>);
impl RAWINTERRSTAT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTERRSTAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRSTAT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat0(&self) -> RAWINTERRSTAT0_R {
        RAWINTERRSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat1(&self) -> RAWINTERRSTAT1_R {
        RAWINTERRSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat2(&self) -> RAWINTERRSTAT2_R {
        RAWINTERRSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat3(&self) -> RAWINTERRSTAT3_R {
        RAWINTERRSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat4(&self) -> RAWINTERRSTAT4_R {
        RAWINTERRSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat5(&self) -> RAWINTERRSTAT5_R {
        RAWINTERRSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat6(&self) -> RAWINTERRSTAT6_R {
        RAWINTERRSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat7(&self) -> RAWINTERRSTAT7_R {
        RAWINTERRSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Raw Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawinterrstat](index.html) module"]
pub struct RAWINTERRSTAT_SPEC;
impl crate::RegisterSpec for RAWINTERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rawinterrstat::R](R) reader structure"]
impl crate::Readable for RAWINTERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAWINTERRSTAT to value 0"]
impl crate::Resettable for RAWINTERRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
