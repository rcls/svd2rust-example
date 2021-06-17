#[doc = "Register `RAWINTTCSTAT` reader"]
pub struct R(crate::R<RAWINTTCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAWINTTCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAWINTTCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAWINTTCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAWINTTCSTAT0` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT0_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT1` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT1_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT2` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT2_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT3` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT3_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT4` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT4_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT5` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT5_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT6` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT6_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWINTTCSTAT7` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub struct RAWINTTCSTAT7_R(crate::FieldReader<bool, bool>);
impl RAWINTTCSTAT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAWINTTCSTAT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTAT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat0(&self) -> RAWINTTCSTAT0_R {
        RAWINTTCSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat1(&self) -> RAWINTTCSTAT1_R {
        RAWINTTCSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat2(&self) -> RAWINTTCSTAT2_R {
        RAWINTTCSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat3(&self) -> RAWINTTCSTAT3_R {
        RAWINTTCSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat4(&self) -> RAWINTTCSTAT4_R {
        RAWINTTCSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat5(&self) -> RAWINTTCSTAT5_R {
        RAWINTTCSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat6(&self) -> RAWINTTCSTAT6_R {
        RAWINTTCSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat7(&self) -> RAWINTTCSTAT7_R {
        RAWINTTCSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Raw Interrupt Terminal Count Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawinttcstat](index.html) module"]
pub struct RAWINTTCSTAT_SPEC;
impl crate::RegisterSpec for RAWINTTCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rawinttcstat::R](R) reader structure"]
impl crate::Readable for RAWINTTCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAWINTTCSTAT to value 0"]
impl crate::Resettable for RAWINTTCSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
