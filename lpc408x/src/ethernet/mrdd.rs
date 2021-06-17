#[doc = "Register `MRDD` reader"]
pub struct R(crate::R<MRDD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRDD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRDD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRDD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READDATA` reader - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
pub struct READDATA_R(crate::FieldReader<u16, u16>);
impl READDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        READDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
    #[inline(always)]
    pub fn readdata(&self) -> READDATA_R {
        READDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MII Mgmt Read Data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdd](index.html) module"]
pub struct MRDD_SPEC;
impl crate::RegisterSpec for MRDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrdd::R](R) reader structure"]
impl crate::Readable for MRDD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRDD to value 0"]
impl crate::Resettable for MRDD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
