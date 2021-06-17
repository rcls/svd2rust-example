#[doc = "Register `INTR_USBHOST_MASKED` reader"]
pub struct R(crate::R<INTR_USBHOST_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOFIRQED` reader - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
pub struct SOFIRQED_R(crate::FieldReader<bool, bool>);
impl SOFIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRQED` reader - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
pub struct DIRQED_R(crate::FieldReader<bool, bool>);
impl DIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNNIRQED` reader - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
pub struct CNNIRQED_R(crate::FieldReader<bool, bool>);
impl CNNIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNNIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNNIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPIRQED` reader - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
pub struct CMPIRQED_R(crate::FieldReader<bool, bool>);
impl CMPIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URIRQED` reader - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
pub struct URIRQED_R(crate::FieldReader<bool, bool>);
impl URIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        URIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKIRQED` reader - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
pub struct RWKIRQED_R(crate::FieldReader<bool, bool>);
impl RWKIRQED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKIRQED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKIRQED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_6` reader - N/A"]
pub struct RSVD_6_R(crate::FieldReader<bool, bool>);
impl RSVD_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCANED` reader - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
pub struct TCANED_R(crate::FieldReader<bool, bool>);
impl TCANED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCANED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCANED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub fn sofirqed(&self) -> SOFIRQED_R {
        SOFIRQED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub fn dirqed(&self) -> DIRQED_R {
        DIRQED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub fn cnnirqed(&self) -> CNNIRQED_R {
        CNNIRQED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub fn cmpirqed(&self) -> CMPIRQED_R {
        CMPIRQED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub fn urirqed(&self) -> URIRQED_R {
        URIRQED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub fn rwkirqed(&self) -> RWKIRQED_R {
        RWKIRQED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub fn tcaned(&self) -> TCANED_R {
        TCANED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Interrupt USB Host Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_masked](index.html) module"]
pub struct INTR_USBHOST_MASKED_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_masked::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_USBHOST_MASKED to value 0"]
impl crate::Resettable for INTR_USBHOST_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
