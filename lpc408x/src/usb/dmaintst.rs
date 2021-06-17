#[doc = "Register `DMAINTST` reader"]
pub struct R(crate::R<DMAINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "End of Transfer Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_A {
    #[doc = "0: All bits in the USBEoTIntSt register are 0."]
    ALL_BITS_IN_THE_USBE = 0,
    #[doc = "1: At least one bit in the USBEoTIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - End of Transfer Interrupt bit."]
pub struct EOT_R(crate::FieldReader<bool, EOT_A>);
impl EOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::ALL_BITS_IN_THE_USBE,
            true => EOT_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBE`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbe(&self) -> bool {
        **self == EOT_A::ALL_BITS_IN_THE_USBE
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        **self == EOT_A::AT_LEAST_ONE_BIT_IN_
    }
}
impl core::ops::Deref for EOT_R {
    type Target = crate::FieldReader<bool, EOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "New DD Request Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDR_A {
    #[doc = "0: All bits in the USBNDDRIntSt register are 0."]
    ALL_BITS_IN_THE_USBN = 0,
    #[doc = "1: At least one bit in the USBNDDRIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<NDDR_A> for bool {
    #[inline(always)]
    fn from(variant: NDDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDDR` reader - New DD Request Interrupt bit."]
pub struct NDDR_R(crate::FieldReader<bool, NDDR_A>);
impl NDDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NDDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDDR_A {
        match self.bits {
            false => NDDR_A::ALL_BITS_IN_THE_USBN,
            true => NDDR_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBN`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbn(&self) -> bool {
        **self == NDDR_A::ALL_BITS_IN_THE_USBN
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        **self == NDDR_A::AT_LEAST_ONE_BIT_IN_
    }
}
impl core::ops::Deref for NDDR_R {
    type Target = crate::FieldReader<bool, NDDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System Error Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: All bits in the USBSysErrIntSt register are 0."]
    ALL_BITS_IN_THE_USBS = 0,
    #[doc = "1: At least one bit in the USBSysErrIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - System Error Interrupt bit."]
pub struct ERR_R(crate::FieldReader<bool, ERR_A>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::ALL_BITS_IN_THE_USBS,
            true => ERR_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBS`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbs(&self) -> bool {
        **self == ERR_A::ALL_BITS_IN_THE_USBS
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        **self == ERR_A::AT_LEAST_ONE_BIT_IN_
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "USB DMA Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaintst](index.html) module"]
pub struct DMAINTST_SPEC;
impl crate::RegisterSpec for DMAINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaintst::R](R) reader structure"]
impl crate::Readable for DMAINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAINTST to value 0"]
impl crate::Resettable for DMAINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
