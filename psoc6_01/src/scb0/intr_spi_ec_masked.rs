#[doc = "Register `INTR_SPI_EC_MASKED` reader"]
pub struct R(crate::R<INTR_SPI_EC_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPI_EC_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPI_EC_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPI_EC_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKE_UP` reader - Logical and of corresponding request and mask bits."]
pub struct WAKE_UP_R(crate::FieldReader<bool, bool>);
impl WAKE_UP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct EZ_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EZ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct EZ_WRITE_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_WRITE_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EZ_WRITE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_WRITE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_READ_STOP` reader - Logical and of corresponding request and mask bits."]
pub struct EZ_READ_STOP_R(crate::FieldReader<bool, bool>);
impl EZ_READ_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EZ_READ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_READ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Externally clocked SPI interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_spi_ec_masked](index.html) module"]
pub struct INTR_SPI_EC_MASKED_SPEC;
impl crate::RegisterSpec for INTR_SPI_EC_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_spi_ec_masked::R](R) reader structure"]
impl crate::Readable for INTR_SPI_EC_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_SPI_EC_MASKED to value 0"]
impl crate::Resettable for INTR_SPI_EC_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
