#[doc = "Register `EERDATA` reader"]
pub struct R(crate::R<EERDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EERDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EERDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EERDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Read data. In case of: 8-bit read operations: bits \\[7:0\\]
contain read data, others are zero. 16-bit read operations: bits \\[15:0\\]
contain read data, others are zero. 32-bit read operations: bits \\[31:0\\]
contain read data."]
pub struct RDATA_R(crate::FieldReader<u32, u32>);
impl RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Read data. In case of: 8-bit read operations: bits \\[7:0\\]
contain read data, others are zero. 16-bit read operations: bits \\[15:0\\]
contain read data, others are zero. 32-bit read operations: bits \\[31:0\\]
contain read data."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "EEPROM read data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eerdata](index.html) module"]
pub struct EERDATA_SPEC;
impl crate::RegisterSpec for EERDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eerdata::R](R) reader structure"]
impl crate::Readable for EERDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EERDATA to value 0"]
impl crate::Resettable for EERDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
