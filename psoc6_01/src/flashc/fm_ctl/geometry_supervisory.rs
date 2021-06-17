#[doc = "Register `GEOMETRY_SUPERVISORY` reader"]
pub struct R(crate::R<GEOMETRY_SUPERVISORY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEOMETRY_SUPERVISORY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEOMETRY_SUPERVISORY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEOMETRY_SUPERVISORY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WORD_SIZE_LOG2` reader - Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
pub struct WORD_SIZE_LOG2_R(crate::FieldReader<u8, u8>);
impl WORD_SIZE_LOG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WORD_SIZE_LOG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORD_SIZE_LOG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGE_SIZE_LOG2` reader - Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
pub struct PAGE_SIZE_LOG2_R(crate::FieldReader<u8, u8>);
impl PAGE_SIZE_LOG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAGE_SIZE_LOG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE_SIZE_LOG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROW_COUNT` reader - Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
pub struct ROW_COUNT_R(crate::FieldReader<u16, u16>);
impl ROW_COUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        ROW_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANK_COUNT` reader - Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
pub struct BANK_COUNT_R(crate::FieldReader<u8, u8>);
impl BANK_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANK_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANK_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub fn word_size_log2(&self) -> WORD_SIZE_LOG2_R {
        WORD_SIZE_LOG2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub fn page_size_log2(&self) -> PAGE_SIZE_LOG2_R {
        PAGE_SIZE_LOG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub fn row_count(&self) -> ROW_COUNT_R {
        ROW_COUNT_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub fn bank_count(&self) -> BANK_COUNT_R {
        BANK_COUNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Supervisory flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry_supervisory](index.html) module"]
pub struct GEOMETRY_SUPERVISORY_SPEC;
impl crate::RegisterSpec for GEOMETRY_SUPERVISORY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [geometry_supervisory::R](R) reader structure"]
impl crate::Readable for GEOMETRY_SUPERVISORY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GEOMETRY_SUPERVISORY to value 0"]
impl crate::Resettable for GEOMETRY_SUPERVISORY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
