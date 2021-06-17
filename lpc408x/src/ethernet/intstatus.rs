#[doc = "Register `INTSTATUS` reader"]
pub struct R(crate::R<INTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOVERRUNINT` reader - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
pub struct RXOVERRUNINT_R(crate::FieldReader<bool, bool>);
impl RXOVERRUNINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRUNINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRUNINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERRORINT` reader - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
pub struct RXERRORINT_R(crate::FieldReader<bool, bool>);
impl RXERRORINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXERRORINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERRORINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFINISHEDINT` reader - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct RXFINISHEDINT_R(crate::FieldReader<bool, bool>);
impl RXFINISHEDINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFINISHEDINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFINISHEDINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDONEINT` reader - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub struct RXDONEINT_R(crate::FieldReader<bool, bool>);
impl RXDONEINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDONEINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDONEINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRUNINT` reader - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
pub struct TXUNDERRUNINT_R(crate::FieldReader<bool, bool>);
impl TXUNDERRUNINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRUNINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRUNINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERRORINT` reader - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
pub struct TXERRORINT_R(crate::FieldReader<bool, bool>);
impl TXERRORINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERRORINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRORINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFINISHEDINT` reader - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct TXFINISHEDINT_R(crate::FieldReader<bool, bool>);
impl TXFINISHEDINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFINISHEDINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFINISHEDINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDONEINT` reader - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub struct TXDONEINT_R(crate::FieldReader<bool, bool>);
impl TXDONEINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDONEINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDONEINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTINT` reader - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub struct SOFTINT_R(crate::FieldReader<bool, bool>);
impl SOFTINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPINT` reader - Interrupt triggered by a Wake-up event detected by the receive filter."]
pub struct WAKEUPINT_R(crate::FieldReader<bool, bool>);
impl WAKEUPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
    #[inline(always)]
    pub fn rxoverrunint(&self) -> RXOVERRUNINT_R {
        RXOVERRUNINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
    #[inline(always)]
    pub fn rxerrorint(&self) -> RXERRORINT_R {
        RXERRORINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedint(&self) -> RXFINISHEDINT_R {
        RXFINISHEDINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneint(&self) -> RXDONEINT_R {
        RXDONEINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
    #[inline(always)]
    pub fn txunderrunint(&self) -> TXUNDERRUNINT_R {
        TXUNDERRUNINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
    #[inline(always)]
    pub fn txerrorint(&self) -> TXERRORINT_R {
        TXERRORINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedint(&self) -> TXFINISHEDINT_R {
        TXFINISHEDINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneint(&self) -> TXDONEINT_R {
        TXDONEINT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softint(&self) -> SOFTINT_R {
        SOFTINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupint(&self) -> WAKEUPINT_R {
        WAKEUPINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](index.html) module"]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstatus::R](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
