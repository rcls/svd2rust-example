#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receiver Data Ready. UnLSR\\[0\\]
is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR_A {
    #[doc = "0: The UARTn receiver FIFO is empty."]
    THE_UARTN_RECEIVER_F = 0,
    #[doc = "1: The UARTn receiver FIFO is not empty."]
    THE_UARTN_RECEIVER_F = 1,
}
impl From<RDR_A> for bool {
    #[inline(always)]
    fn from(variant: RDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready. UnLSR\\[0\\]
is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty."]
pub struct RDR_R(crate::FieldReader<bool, RDR_A>);
impl RDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDR_A {
        match self.bits {
            false => RDR_A::THE_UARTN_RECEIVER_F,
            true => RDR_A::THE_UARTN_RECEIVER_F,
        }
    }
    #[doc = "Checks if the value of the field is `THE_UARTN_RECEIVER_F`"]
    #[inline(always)]
    pub fn is_the_uartn_receiver_f(&self) -> bool {
        **self == RDR_A::THE_UARTN_RECEIVER_F
    }
    #[doc = "Checks if the value of the field is `THE_UARTN_RECEIVER_F`"]
    #[inline(always)]
    pub fn is_the_uartn_receiver_f(&self) -> bool {
        **self == RDR_A::THE_UARTN_RECEIVER_F
    }
}
impl core::ops::Deref for RDR_R {
    type Target = crate::FieldReader<bool, RDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\]
is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: Overrun error status is inactive."]
    OVERRUN_ERROR_STATUS = 0,
    #[doc = "1: Overrun error status is active."]
    OVERRUN_ERROR_STATUS = 1,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\]
is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost."]
pub struct OE_R(crate::FieldReader<bool, OE_A>);
impl OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::OVERRUN_ERROR_STATUS,
            true => OE_A::OVERRUN_ERROR_STATUS,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRUN_ERROR_STATUS`"]
    #[inline(always)]
    pub fn is_overrun_error_status(&self) -> bool {
        **self == OE_A::OVERRUN_ERROR_STATUS
    }
    #[doc = "Checks if the value of the field is `OVERRUN_ERROR_STATUS`"]
    #[inline(always)]
    pub fn is_overrun_error_status(&self) -> bool {
        **self == OE_A::OVERRUN_ERROR_STATUS
    }
}
impl core::ops::Deref for OE_R {
    type Target = crate::FieldReader<bool, OE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Parity error status is inactive."]
    PARITY_ERROR_STATUS_ = 0,
    #[doc = "1: Parity error status is active."]
    PARITY_ERROR_STATUS_ = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO."]
pub struct PE_R(crate::FieldReader<bool, PE_A>);
impl PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::PARITY_ERROR_STATUS_,
            true => PE_A::PARITY_ERROR_STATUS_,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_STATUS_`"]
    #[inline(always)]
    pub fn is_parity_error_status_(&self) -> bool {
        **self == PE_A::PARITY_ERROR_STATUS_
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_STATUS_`"]
    #[inline(always)]
    pub fn is_parity_error_status_(&self) -> bool {
        **self == PE_A::PARITY_ERROR_STATUS_
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: Framing error status is inactive."]
    FRAMING_ERROR_STATUS = 0,
    #[doc = "1: Framing error status is active."]
    FRAMING_ERROR_STATUS = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO."]
pub struct FE_R(crate::FieldReader<bool, FE_A>);
impl FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::FRAMING_ERROR_STATUS,
            true => FE_A::FRAMING_ERROR_STATUS,
        }
    }
    #[doc = "Checks if the value of the field is `FRAMING_ERROR_STATUS`"]
    #[inline(always)]
    pub fn is_framing_error_status(&self) -> bool {
        **self == FE_A::FRAMING_ERROR_STATUS
    }
    #[doc = "Checks if the value of the field is `FRAMING_ERROR_STATUS`"]
    #[inline(always)]
    pub fn is_framing_error_status(&self) -> bool {
        **self == FE_A::FRAMING_ERROR_STATUS
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BI_A {
    #[doc = "0: Break interrupt status is inactive."]
    BREAK_INTERRUPT_STAT = 0,
    #[doc = "1: Break interrupt status is active."]
    BREAK_INTERRUPT_STAT = 1,
}
impl From<BI_A> for bool {
    #[inline(always)]
    fn from(variant: BI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO."]
pub struct BI_R(crate::FieldReader<bool, BI_A>);
impl BI_R {
    pub(crate) fn new(bits: bool) -> Self {
        BI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BI_A {
        match self.bits {
            false => BI_A::BREAK_INTERRUPT_STAT,
            true => BI_A::BREAK_INTERRUPT_STAT,
        }
    }
    #[doc = "Checks if the value of the field is `BREAK_INTERRUPT_STAT`"]
    #[inline(always)]
    pub fn is_break_interrupt_stat(&self) -> bool {
        **self == BI_A::BREAK_INTERRUPT_STAT
    }
    #[doc = "Checks if the value of the field is `BREAK_INTERRUPT_STAT`"]
    #[inline(always)]
    pub fn is_break_interrupt_stat(&self) -> bool {
        **self == BI_A::BREAK_INTERRUPT_STAT
    }
}
impl core::ops::Deref for BI_R {
    type Target = crate::FieldReader<bool, BI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRE_A {
    #[doc = "0: UnTHR contains valid data."]
    UNTHR_CONTAINS_VALID = 0,
    #[doc = "1: UnTHR is empty."]
    UNTHR_IS_EMPTY_ = 1,
}
impl From<THRE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write."]
pub struct THRE_R(crate::FieldReader<bool, THRE_A>);
impl THRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRE_A {
        match self.bits {
            false => THRE_A::UNTHR_CONTAINS_VALID,
            true => THRE_A::UNTHR_IS_EMPTY_,
        }
    }
    #[doc = "Checks if the value of the field is `UNTHR_CONTAINS_VALID`"]
    #[inline(always)]
    pub fn is_unthr_contains_valid(&self) -> bool {
        **self == THRE_A::UNTHR_CONTAINS_VALID
    }
    #[doc = "Checks if the value of the field is `UNTHR_IS_EMPTY_`"]
    #[inline(always)]
    pub fn is_unthr_is_empty_(&self) -> bool {
        **self == THRE_A::UNTHR_IS_EMPTY_
    }
}
impl core::ops::Deref for THRE_R {
    type Target = crate::FieldReader<bool, THRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMT_A {
    #[doc = "0: UnTHR and/or the UnTSR contains valid data."]
    VALID_DATA = 0,
    #[doc = "1: UnTHR and the UnTSR are empty."]
    EMPTY = 1,
}
impl From<TEMT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data."]
pub struct TEMT_R(crate::FieldReader<bool, TEMT_A>);
impl TEMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMT_A {
        match self.bits {
            false => TEMT_A::VALID_DATA,
            true => TEMT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_DATA`"]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        **self == TEMT_A::VALID_DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TEMT_A::EMPTY
    }
}
impl core::ops::Deref for TEMT_R {
    type Target = crate::FieldReader<bool, TEMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Error in RX FIFO . UnLSR\\[7\\]
is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: UnRBR contains no UARTn RX errors or UnFCR\\[0\\]=0."]
    UNRBR_CONTAINS_NO_UA = 0,
    #[doc = "1: UARTn RBR contains at least one UARTn RX error."]
    UARTN_RBR_CONTAINS_A = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO . UnLSR\\[7\\]
is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO."]
pub struct RXFE_R(crate::FieldReader<bool, RXFE_A>);
impl RXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::UNRBR_CONTAINS_NO_UA,
            true => RXFE_A::UARTN_RBR_CONTAINS_A,
        }
    }
    #[doc = "Checks if the value of the field is `UNRBR_CONTAINS_NO_UA`"]
    #[inline(always)]
    pub fn is_unrbr_contains_no_ua(&self) -> bool {
        **self == RXFE_A::UNRBR_CONTAINS_NO_UA
    }
    #[doc = "Checks if the value of the field is `UARTN_RBR_CONTAINS_A`"]
    #[inline(always)]
    pub fn is_uartn_rbr_contains_a(&self) -> bool {
        **self == RXFE_A::UARTN_RBR_CONTAINS_A
    }
}
impl core::ops::Deref for RXFE_R {
    type Target = crate::FieldReader<bool, RXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. UnLSR\\[0\\]
is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\]
is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO . UnLSR\\[7\\]
is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
