#[doc = "Register `PLLSTAT%s` reader"]
pub struct R(crate::R<PLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSEL` reader - Read-back for the PLL Multiplier value. This is the value currently used by the related PLL."]
pub struct MSEL_R(crate::FieldReader<u8, u8>);
impl MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` reader - Read-back for the PLL Divider value. This is the value currently used by the related PLL."]
pub struct PSEL_R(crate::FieldReader<u8, u8>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLE_STAT` reader - Read-back for the PLL Enable bit. When one, the related PLL is currently activated. When zero, the related PLL is turned off. This bit is automatically cleared when Power-down mode is activated."]
pub struct PLLE_STAT_R(crate::FieldReader<bool, bool>);
impl PLLE_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLE_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLE_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLOCK` reader - Reflects the PLL Lock status. When zero, the related PLL is not locked. When one, the related PLL is locked onto the requested frequency."]
pub struct PLOCK_R(crate::FieldReader<bool, bool>);
impl PLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Read-back for the PLL Multiplier value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL Divider value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL Enable bit. When one, the related PLL is currently activated. When zero, the related PLL is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle_stat(&self) -> PLLE_STAT_R {
        PLLE_STAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL Lock status. When zero, the related PLL is not locked. When one, the related PLL is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "PLL0 Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllstat](index.html) module"]
pub struct PLLSTAT_SPEC;
impl crate::RegisterSpec for PLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllstat::R](R) reader structure"]
impl crate::Readable for PLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLLSTAT%s to value 0"]
impl crate::Resettable for PLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
