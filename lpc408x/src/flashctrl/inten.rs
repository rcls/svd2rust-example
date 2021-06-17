#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EE_RW_DONE` reader - EEPROM read/write operation finished interrupt enable bit. Bit is: - set when 1 is written to the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
pub struct EE_RW_DONE_R(crate::FieldReader<bool, bool>);
impl EE_RW_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE_RW_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE_RW_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE_PROG_DONE` reader - EEPROM program operation finished interrupt enable bit. Bit is: - set when 1 is written in the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
pub struct EE_PROG_DONE_R(crate::FieldReader<bool, bool>);
impl EE_PROG_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE_PROG_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE_PROG_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt enable bit. Bit is: - set when 1 is written to the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline(always)]
    pub fn ee_rw_done(&self) -> EE_RW_DONE_R {
        EE_RW_DONE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt enable bit. Bit is: - set when 1 is written in the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline(always)]
    pub fn ee_prog_done(&self) -> EE_PROG_DONE_R {
        EE_PROG_DONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "EEPROM interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
