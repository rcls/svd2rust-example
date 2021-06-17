#[doc = "Register `INTR_HOST_EP_CAUSE_MED` reader"]
pub struct R(crate::R<INTR_HOST_EP_CAUSE_MED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_HOST_EP_CAUSE_MED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_HOST_EP_CAUSE_MED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_HOST_EP_CAUSE_MED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP1DRQ_INT` reader - EP1DRQ interrupt"]
pub struct EP1DRQ_INT_R(crate::FieldReader<bool, bool>);
impl EP1DRQ_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1DRQ_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1DRQ_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1SPK_INT` reader - EP1SPK interrupt"]
pub struct EP1SPK_INT_R(crate::FieldReader<bool, bool>);
impl EP1SPK_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1SPK_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1SPK_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2DRQ_INT` reader - EP2DRQ interrupt"]
pub struct EP2DRQ_INT_R(crate::FieldReader<bool, bool>);
impl EP2DRQ_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2DRQ_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2DRQ_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2SPK_INT` reader - EP2SPK interrupt"]
pub struct EP2SPK_INT_R(crate::FieldReader<bool, bool>);
impl EP2SPK_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2SPK_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2SPK_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - EP1DRQ interrupt"]
    #[inline(always)]
    pub fn ep1drq_int(&self) -> EP1DRQ_INT_R {
        EP1DRQ_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EP1SPK interrupt"]
    #[inline(always)]
    pub fn ep1spk_int(&self) -> EP1SPK_INT_R {
        EP1SPK_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EP2DRQ interrupt"]
    #[inline(always)]
    pub fn ep2drq_int(&self) -> EP2DRQ_INT_R {
        EP2DRQ_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EP2SPK interrupt"]
    #[inline(always)]
    pub fn ep2spk_int(&self) -> EP2SPK_INT_R {
        EP2SPK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause Medium Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_cause_med](index.html) module"]
pub struct INTR_HOST_EP_CAUSE_MED_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_CAUSE_MED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_host_ep_cause_med::R](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_CAUSE_MED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_HOST_EP_CAUSE_MED to value 0"]
impl crate::Resettable for INTR_HOST_EP_CAUSE_MED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
