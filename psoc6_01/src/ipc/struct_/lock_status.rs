#[doc = "Register `LOCK_STATUS` reader"]
pub struct R(crate::R<LOCK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P` reader - This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
pub struct P_R(crate::FieldReader<bool, bool>);
impl P_R {
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` reader - This field specifies the cecure/on-secure access control: '0': secure. '1': non-secure."]
pub struct NS_R(crate::FieldReader<bool, bool>);
impl NS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC` reader - This field specifies the protection context that successfully acquired the lock."]
pub struct PC_R(crate::FieldReader<u8, u8>);
impl PC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS` reader - This field specifies the bus master identifier that successfully acquired the lock."]
pub struct MS_R(crate::FieldReader<u8, u8>);
impl MS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACQUIRED` reader - Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1')."]
pub struct ACQUIRED_R(crate::FieldReader<bool, bool>);
impl ACQUIRED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACQUIRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACQUIRED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This field specifies the cecure/on-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1')."]
    #[inline(always)]
    pub fn acquired(&self) -> ACQUIRED_R {
        ACQUIRED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "IPC lock status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_status](index.html) module"]
pub struct LOCK_STATUS_SPEC;
impl crate::RegisterSpec for LOCK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock_status::R](R) reader structure"]
impl crate::Readable for LOCK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCK_STATUS to value 0"]
impl crate::Resettable for LOCK_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
