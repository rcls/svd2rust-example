#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIG_DONE` reader - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
pub struct SIG_DONE_R(crate::FieldReader<bool, bool>);
impl SIG_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIG_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_OF_RDWR` reader - EEPROM read/write operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written in the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
pub struct END_OF_RDWR_R(crate::FieldReader<bool, bool>);
impl END_OF_RDWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_OF_RDWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_OF_RDWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_OF_PROG1` reader - EEPROM program operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written to the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
pub struct END_OF_PROG1_R(crate::FieldReader<bool, bool>);
impl END_OF_PROG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_OF_PROG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_OF_PROG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written in the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline(always)]
    pub fn end_of_rdwr(&self) -> END_OF_RDWR_R {
        END_OF_RDWR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written to the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline(always)]
    pub fn end_of_prog1(&self) -> END_OF_PROG1_R {
        END_OF_PROG1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "Signature generation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
