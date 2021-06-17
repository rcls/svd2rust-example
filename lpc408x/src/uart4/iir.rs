#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt status. Note that U4IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating U4IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt is pending."]
    AT_LEAST_ONE_INTERRU = 0,
    #[doc = "1: No interrupt is pending."]
    NO_INTERRUPT_IS_PEND = 1,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status. Note that U4IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating U4IIR\\[3:1\\]."]
pub struct INTSTATUS_R(crate::FieldReader<bool, INTSTATUS_A>);
impl INTSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::AT_LEAST_ONE_INTERRU,
            true => INTSTATUS_A::NO_INTERRUPT_IS_PEND,
        }
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`"]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        **self == INTSTATUS_A::AT_LEAST_ONE_INTERRU
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`"]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        **self == INTSTATUS_A::NO_INTERRUPT_IS_PEND
    }
}
impl core::ops::Deref for INTSTATUS_R {
    type Target = crate::FieldReader<bool, INTSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt identification. U4IER\\[3:1\\]
identifies an interrupt corresponding to the UART4 Rx or TX FIFO. All other combinations of U4IER\\[3:1\\]
not listed below are reserved (000,100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTID_A {
    #[doc = "3: 1   - Receive Line Status (RLS)."]
    _1_RECEIVE_LINE_S = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    _2A__RECEIVE_DATA_AV = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    _2B__CHARACTER_TIME_ = 6,
    #[doc = "1: 3   - THRE Interrupt"]
    _3_THRE_INTERRUPT = 1,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTID` reader - Interrupt identification. U4IER\\[3:1\\]
identifies an interrupt corresponding to the UART4 Rx or TX FIFO. All other combinations of U4IER\\[3:1\\]
not listed below are reserved (000,100,101,111)."]
pub struct INTID_R(crate::FieldReader<u8, INTID_A>);
impl INTID_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTID_A> {
        match self.bits {
            3 => Some(INTID_A::_1_RECEIVE_LINE_S),
            2 => Some(INTID_A::_2A__RECEIVE_DATA_AV),
            6 => Some(INTID_A::_2B__CHARACTER_TIME_),
            1 => Some(INTID_A::_3_THRE_INTERRUPT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_RECEIVE_LINE_S`"]
    #[inline(always)]
    pub fn is_1_receive_line_s(&self) -> bool {
        **self == INTID_A::_1_RECEIVE_LINE_S
    }
    #[doc = "Checks if the value of the field is `_2A__RECEIVE_DATA_AV`"]
    #[inline(always)]
    pub fn is_2a__receive_data_av(&self) -> bool {
        **self == INTID_A::_2A__RECEIVE_DATA_AV
    }
    #[doc = "Checks if the value of the field is `_2B__CHARACTER_TIME_`"]
    #[inline(always)]
    pub fn is_2b__character_time_(&self) -> bool {
        **self == INTID_A::_2B__CHARACTER_TIME_
    }
    #[doc = "Checks if the value of the field is `_3_THRE_INTERRUPT`"]
    #[inline(always)]
    pub fn is_3_thre_interrupt(&self) -> bool {
        **self == INTID_A::_3_THRE_INTERRUPT
    }
}
impl core::ops::Deref for INTID_R {
    type Target = crate::FieldReader<u8, INTID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOENABLE` reader - Copies of U4FCR\\[0\\]."]
pub struct FIFOENABLE_R(crate::FieldReader<u8, u8>);
impl FIFOENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABEOINT` reader - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
pub struct ABEOINT_R(crate::FieldReader<bool, bool>);
impl ABEOINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABEOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABEOINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTOINT` reader - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
pub struct ABTOINT_R(crate::FieldReader<bool, bool>);
impl ABTOINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABTOINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that U4IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating U4IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. U4IER\\[3:1\\]
identifies an interrupt corresponding to the UART4 Rx or TX FIFO. All other combinations of U4IER\\[3:1\\]
not listed below are reserved (000,100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of U4FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
