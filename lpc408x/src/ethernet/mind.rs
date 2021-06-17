#[doc = "Register `MIND` reader"]
pub struct R(crate::R<MIND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCANNING` reader - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
pub struct SCANNING_R(crate::FieldReader<bool, bool>);
impl SCANNING_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCANNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCANNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOTVALID` reader - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
pub struct NOTVALID_R(crate::FieldReader<bool, bool>);
impl NOTVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOTVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOTVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIILINKFAIL` reader - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
pub struct MIILINKFAIL_R(crate::FieldReader<bool, bool>);
impl MIILINKFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIILINKFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIILINKFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
    #[inline(always)]
    pub fn scanning(&self) -> SCANNING_R {
        SCANNING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
    #[inline(always)]
    pub fn notvalid(&self) -> NOTVALID_R {
        NOTVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
    #[inline(always)]
    pub fn miilinkfail(&self) -> MIILINKFAIL_R {
        MIILINKFAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "MII Mgmt Indicators register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mind](index.html) module"]
pub struct MIND_SPEC;
impl crate::RegisterSpec for MIND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mind::R](R) reader structure"]
impl crate::Readable for MIND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIND to value 0"]
impl crate::Resettable for MIND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
