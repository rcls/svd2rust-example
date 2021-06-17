#[doc = "Register `INTERRSTAT` reader"]
pub struct R(crate::R<INTERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRSTAT0` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT0_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT1` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT1_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT2` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT2_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT3` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT3_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT4` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT4_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT5` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT5_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT6` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT6_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERRSTAT7` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub struct INTERRSTAT7_R(crate::FieldReader<bool, bool>);
impl INTERRSTAT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERRSTAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRSTAT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat0(&self) -> INTERRSTAT0_R {
        INTERRSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat1(&self) -> INTERRSTAT1_R {
        INTERRSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat2(&self) -> INTERRSTAT2_R {
        INTERRSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat3(&self) -> INTERRSTAT3_R {
        INTERRSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat4(&self) -> INTERRSTAT4_R {
        INTERRSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat5(&self) -> INTERRSTAT5_R {
        INTERRSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat6(&self) -> INTERRSTAT6_R {
        INTERRSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat7(&self) -> INTERRSTAT7_R {
        INTERRSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Interrupt Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrstat](index.html) module"]
pub struct INTERRSTAT_SPEC;
impl crate::RegisterSpec for INTERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrstat::R](R) reader structure"]
impl crate::Readable for INTERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERRSTAT to value 0"]
impl crate::Resettable for INTERRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
