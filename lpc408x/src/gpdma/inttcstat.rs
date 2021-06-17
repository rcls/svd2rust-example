#[doc = "Register `INTTCSTAT` reader"]
pub struct R(crate::R<INTTCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTTCSTAT0` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT0_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT1` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT1_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT2` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT2_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT3` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT3_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT4` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT4_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT5` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT5_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT6` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT6_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTCSTAT7` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct INTTCSTAT7_R(crate::FieldReader<bool, bool>);
impl INTTCSTAT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTTCSTAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTAT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat0(&self) -> INTTCSTAT0_R {
        INTTCSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat1(&self) -> INTTCSTAT1_R {
        INTTCSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat2(&self) -> INTTCSTAT2_R {
        INTTCSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat3(&self) -> INTTCSTAT3_R {
        INTTCSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat4(&self) -> INTTCSTAT4_R {
        INTTCSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat5(&self) -> INTTCSTAT5_R {
        INTTCSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat6(&self) -> INTTCSTAT6_R {
        INTTCSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat7(&self) -> INTTCSTAT7_R {
        INTTCSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Interrupt Terminal Count Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttcstat](index.html) module"]
pub struct INTTCSTAT_SPEC;
impl crate::RegisterSpec for INTTCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttcstat::R](R) reader structure"]
impl crate::Readable for INTTCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTTCSTAT to value 0"]
impl crate::Resettable for INTTCSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
