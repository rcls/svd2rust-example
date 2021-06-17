#[doc = "Register `TXSR` reader"]
pub struct R(crate::R<TXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS1` reader - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
pub struct TS1_R(crate::FieldReader<bool, bool>);
impl TS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS2` reader - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
pub struct TS2_R(crate::FieldReader<bool, bool>);
impl TS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBS1` reader - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
pub struct TBS1_R(crate::FieldReader<bool, bool>);
impl TBS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBS2` reader - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
pub struct TBS2_R(crate::FieldReader<bool, bool>);
impl TBS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCS1` reader - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
pub struct TCS1_R(crate::FieldReader<bool, bool>);
impl TCS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCS2` reader - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
pub struct TCS2_R(crate::FieldReader<bool, bool>);
impl TCS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
    #[inline(always)]
    pub fn tbs1(&self) -> TBS1_R {
        TBS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
    #[inline(always)]
    pub fn tbs2(&self) -> TBS2_R {
        TBS2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
    #[inline(always)]
    pub fn tcs1(&self) -> TCS1_R {
        TCS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
    #[inline(always)]
    pub fn tcs2(&self) -> TCS2_R {
        TCS2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "CAN Central Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsr](index.html) module"]
pub struct TXSR_SPEC;
impl crate::RegisterSpec for TXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txsr::R](R) reader structure"]
impl crate::Readable for TXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXSR to value 0x0003_0300"]
impl crate::Resettable for TXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0300
    }
}
