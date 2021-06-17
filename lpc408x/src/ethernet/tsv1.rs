#[doc = "Register `TSV1` reader"]
pub struct R(crate::R<TSV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBC` reader - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
pub struct TBC_R(crate::FieldReader<u16, u16>);
impl TBC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC` reader - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
pub struct TCC_R(crate::FieldReader<u8, u8>);
impl TCC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Transmit status vector 1 register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsv1](index.html) module"]
pub struct TSV1_SPEC;
impl crate::RegisterSpec for TSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsv1::R](R) reader structure"]
impl crate::Readable for TSV1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSV1 to value 0"]
impl crate::Resettable for TSV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
