#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `E1` reader - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
pub struct E1_R(crate::FieldReader<bool, bool>);
impl E1_R {
    pub(crate) fn new(bits: bool) -> Self {
        E1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E2` reader - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
pub struct E2_R(crate::FieldReader<bool, bool>);
impl E2_R {
    pub(crate) fn new(bits: bool) -> Self {
        E2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS1` reader - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
pub struct BS1_R(crate::FieldReader<bool, bool>);
impl BS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS2` reader - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
pub struct BS2_R(crate::FieldReader<bool, bool>);
impl BS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
    #[inline(always)]
    pub fn e1(&self) -> E1_R {
        E1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
    #[inline(always)]
    pub fn e2(&self) -> E2_R {
        E2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "CAN Central Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
