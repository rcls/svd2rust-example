#[doc = "Register `RXSR` reader"]
pub struct R(crate::R<RXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RS1` reader - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
pub struct RS1_R(crate::FieldReader<bool, bool>);
impl RS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS2` reader - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
pub struct RS2_R(crate::FieldReader<bool, bool>);
impl RS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB1` reader - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
pub struct RB1_R(crate::FieldReader<bool, bool>);
impl RB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB2` reader - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
pub struct RB2_R(crate::FieldReader<bool, bool>);
impl RB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOS1` reader - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
pub struct DOS1_R(crate::FieldReader<bool, bool>);
impl DOS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOS2` reader - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
pub struct DOS2_R(crate::FieldReader<bool, bool>);
impl DOS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
    #[inline(always)]
    pub fn rb1(&self) -> RB1_R {
        RB1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
    #[inline(always)]
    pub fn rb2(&self) -> RB2_R {
        RB2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
    #[inline(always)]
    pub fn dos1(&self) -> DOS1_R {
        DOS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
    #[inline(always)]
    pub fn dos2(&self) -> DOS2_R {
        DOS2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "CAN Central Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxsr](index.html) module"]
pub struct RXSR_SPEC;
impl crate::RegisterSpec for RXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxsr::R](R) reader structure"]
impl crate::Readable for RXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXSR to value 0"]
impl crate::Resettable for RXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
