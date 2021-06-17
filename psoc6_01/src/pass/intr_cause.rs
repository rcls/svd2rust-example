#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTB0_INT` reader - CTB0 interrupt pending"]
pub struct CTB0_INT_R(crate::FieldReader<bool, bool>);
impl CTB0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB1_INT` reader - CTB1 interrupt pending"]
pub struct CTB1_INT_R(crate::FieldReader<bool, bool>);
impl CTB1_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB1_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB1_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB2_INT` reader - CTB2 interrupt pending"]
pub struct CTB2_INT_R(crate::FieldReader<bool, bool>);
impl CTB2_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB2_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB2_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB3_INT` reader - CTB3 interrupt pending"]
pub struct CTB3_INT_R(crate::FieldReader<bool, bool>);
impl CTB3_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB3_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB3_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC0_INT` reader - CTDAC0 interrupt pending"]
pub struct CTDAC0_INT_R(crate::FieldReader<bool, bool>);
impl CTDAC0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDAC0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDAC0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC1_INT` reader - CTDAC1 interrupt pending"]
pub struct CTDAC1_INT_R(crate::FieldReader<bool, bool>);
impl CTDAC1_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDAC1_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDAC1_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC2_INT` reader - CTDAC2 interrupt pending"]
pub struct CTDAC2_INT_R(crate::FieldReader<bool, bool>);
impl CTDAC2_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDAC2_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDAC2_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDAC3_INT` reader - CTDAC3 interrupt pending"]
pub struct CTDAC3_INT_R(crate::FieldReader<bool, bool>);
impl CTDAC3_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDAC3_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDAC3_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CTB0 interrupt pending"]
    #[inline(always)]
    pub fn ctb0_int(&self) -> CTB0_INT_R {
        CTB0_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTB1 interrupt pending"]
    #[inline(always)]
    pub fn ctb1_int(&self) -> CTB1_INT_R {
        CTB1_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTB2 interrupt pending"]
    #[inline(always)]
    pub fn ctb2_int(&self) -> CTB2_INT_R {
        CTB2_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTB3 interrupt pending"]
    #[inline(always)]
    pub fn ctb3_int(&self) -> CTB3_INT_R {
        CTB3_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTDAC0 interrupt pending"]
    #[inline(always)]
    pub fn ctdac0_int(&self) -> CTDAC0_INT_R {
        CTDAC0_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTDAC1 interrupt pending"]
    #[inline(always)]
    pub fn ctdac1_int(&self) -> CTDAC1_INT_R {
        CTDAC1_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CTDAC2 interrupt pending"]
    #[inline(always)]
    pub fn ctdac2_int(&self) -> CTDAC2_INT_R {
        CTDAC2_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTDAC3 interrupt pending"]
    #[inline(always)]
    pub fn ctdac3_int(&self) -> CTDAC3_INT_R {
        CTDAC3_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
