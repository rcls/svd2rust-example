#[doc = "Register `ENBLDCHNS` reader"]
pub struct R(crate::R<ENBLDCHNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENBLDCHNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENBLDCHNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENBLDCHNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLEDCHANNELS0` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS0_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS1` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS1_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS2` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS2_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS3` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS3_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS4` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS4_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS5` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS5_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS6` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS6_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLEDCHANNELS7` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub struct ENABLEDCHANNELS7_R(crate::FieldReader<bool, bool>);
impl ENABLEDCHANNELS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLEDCHANNELS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels0(&self) -> ENABLEDCHANNELS0_R {
        ENABLEDCHANNELS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels1(&self) -> ENABLEDCHANNELS1_R {
        ENABLEDCHANNELS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels2(&self) -> ENABLEDCHANNELS2_R {
        ENABLEDCHANNELS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels3(&self) -> ENABLEDCHANNELS3_R {
        ENABLEDCHANNELS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels4(&self) -> ENABLEDCHANNELS4_R {
        ENABLEDCHANNELS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels5(&self) -> ENABLEDCHANNELS5_R {
        ENABLEDCHANNELS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels6(&self) -> ENABLEDCHANNELS6_R {
        ENABLEDCHANNELS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels7(&self) -> ENABLEDCHANNELS7_R {
        ENABLEDCHANNELS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "DMA Enabled Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enbldchns](index.html) module"]
pub struct ENBLDCHNS_SPEC;
impl crate::RegisterSpec for ENBLDCHNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enbldchns::R](R) reader structure"]
impl crate::Readable for ENBLDCHNS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENBLDCHNS to value 0"]
impl crate::Resettable for ENBLDCHNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
