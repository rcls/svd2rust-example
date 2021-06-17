#[doc = "Register `IDENTITY` reader"]
pub struct R(crate::R<IDENTITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDENTITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDENTITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDENTITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P` reader - This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
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
#[doc = "Field `NS` reader - This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
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
#[doc = "Field `PC` reader - This field specifies the protection context of the transfer that reads the register."]
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
#[doc = "Field `MS` reader - This field specifies the bus master identifier of the transfer that reads the register."]
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
impl R {
    #[doc = "Bit 0 - This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context of the transfer that reads the register."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier of the transfer that reads the register."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Identity\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [identity](index.html) module"]
pub struct IDENTITY_SPEC;
impl crate::RegisterSpec for IDENTITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [identity::R](R) reader structure"]
impl crate::Readable for IDENTITY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDENTITY to value 0"]
impl crate::Resettable for IDENTITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
