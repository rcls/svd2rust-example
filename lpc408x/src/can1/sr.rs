#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBS_1` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub struct RBS_1_R(crate::FieldReader<bool, bool>);
impl RBS_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBS_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOS_1` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub struct DOS_1_R(crate::FieldReader<bool, bool>);
impl DOS_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOS_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS1_1_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TBS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS1_1` reader - Transmit Buffer Status 1."]
pub struct TBS1_1_R(crate::FieldReader<bool, TBS1_1_A>);
impl TBS1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBS1_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS1_1_A {
        match self.bits {
            false => TBS1_1_A::LOCKED_SOFTWARE_CAN,
            true => TBS1_1_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        **self == TBS1_1_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        **self == TBS1_1_A::RELEASED_SOFTWARE_M
    }
}
impl core::ops::Deref for TBS1_1_R {
    type Target = crate::FieldReader<bool, TBS1_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS1_1_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TCS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS1_1` reader - Transmission Complete Status."]
pub struct TCS1_1_R(crate::FieldReader<bool, TCS1_1_A>);
impl TCS1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCS1_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS1_1_A {
        match self.bits {
            false => TCS1_1_A::INCOMPLETE_THE_PREV,
            true => TCS1_1_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        **self == TCS1_1_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        **self == TCS1_1_A::COMPLETE_THE_PREVIO
    }
}
impl core::ops::Deref for TCS1_1_R {
    type Target = crate::FieldReader<bool, TCS1_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS_1` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub struct RS_1_R(crate::FieldReader<bool, bool>);
impl RS_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS1_1_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 1."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_1` reader - Transmit Status 1."]
pub struct TS1_1_R(crate::FieldReader<bool, TS1_1_A>);
impl TS1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS1_1_A {
        match self.bits {
            false => TS1_1_A::IDLE_THERE_IS_NO_TR,
            true => TS1_1_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        **self == TS1_1_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        **self == TS1_1_A::TRANSMIT_THE_CAN_CO
    }
}
impl core::ops::Deref for TS1_1_R {
    type Target = crate::FieldReader<bool, TS1_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES_1` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub struct ES_1_R(crate::FieldReader<bool, bool>);
impl ES_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_1` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub struct BS_1_R(crate::FieldReader<bool, bool>);
impl BS_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBS_2` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub struct RBS_2_R(crate::FieldReader<bool, bool>);
impl RBS_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBS_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOS_2` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub struct DOS_2_R(crate::FieldReader<bool, bool>);
impl DOS_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOS_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS2_2_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TBS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS2_2` reader - Transmit Buffer Status 2."]
pub struct TBS2_2_R(crate::FieldReader<bool, TBS2_2_A>);
impl TBS2_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBS2_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS2_2_A {
        match self.bits {
            false => TBS2_2_A::LOCKED_SOFTWARE_CAN,
            true => TBS2_2_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        **self == TBS2_2_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        **self == TBS2_2_A::RELEASED_SOFTWARE_M
    }
}
impl core::ops::Deref for TBS2_2_R {
    type Target = crate::FieldReader<bool, TBS2_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS2_2_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TCS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS2_2` reader - Transmission Complete Status."]
pub struct TCS2_2_R(crate::FieldReader<bool, TCS2_2_A>);
impl TCS2_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCS2_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS2_2_A {
        match self.bits {
            false => TCS2_2_A::INCOMPLETE_THE_PREV,
            true => TCS2_2_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        **self == TCS2_2_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        **self == TCS2_2_A::COMPLETE_THE_PREVIO
    }
}
impl core::ops::Deref for TCS2_2_R {
    type Target = crate::FieldReader<bool, TCS2_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS_2` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub struct RS_2_R(crate::FieldReader<bool, bool>);
impl RS_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS2_2_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 2."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS2_2` reader - Transmit Status 2."]
pub struct TS2_2_R(crate::FieldReader<bool, TS2_2_A>);
impl TS2_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS2_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS2_2_A {
        match self.bits {
            false => TS2_2_A::IDLE_THERE_IS_NO_TR,
            true => TS2_2_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        **self == TS2_2_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        **self == TS2_2_A::TRANSMIT_THE_CAN_CO
    }
}
impl core::ops::Deref for TS2_2_R {
    type Target = crate::FieldReader<bool, TS2_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES_2` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub struct ES_2_R(crate::FieldReader<bool, bool>);
impl ES_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_2` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub struct BS_2_R(crate::FieldReader<bool, bool>);
impl BS_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBS_3` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub struct RBS_3_R(crate::FieldReader<bool, bool>);
impl RBS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOS_3` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub struct DOS_3_R(crate::FieldReader<bool, bool>);
impl DOS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS3_3_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TBS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS3_3` reader - Transmit Buffer Status 3."]
pub struct TBS3_3_R(crate::FieldReader<bool, TBS3_3_A>);
impl TBS3_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBS3_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS3_3_A {
        match self.bits {
            false => TBS3_3_A::LOCKED_SOFTWARE_CAN,
            true => TBS3_3_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        **self == TBS3_3_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        **self == TBS3_3_A::RELEASED_SOFTWARE_M
    }
}
impl core::ops::Deref for TBS3_3_R {
    type Target = crate::FieldReader<bool, TBS3_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS3_3_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TCS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS3_3` reader - Transmission Complete Status."]
pub struct TCS3_3_R(crate::FieldReader<bool, TCS3_3_A>);
impl TCS3_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCS3_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS3_3_A {
        match self.bits {
            false => TCS3_3_A::INCOMPLETE_THE_PREV,
            true => TCS3_3_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        **self == TCS3_3_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        **self == TCS3_3_A::COMPLETE_THE_PREVIO
    }
}
impl core::ops::Deref for TCS3_3_R {
    type Target = crate::FieldReader<bool, TCS3_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS_3` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub struct RS_3_R(crate::FieldReader<bool, bool>);
impl RS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS3_3_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 3."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS3_3` reader - Transmit Status 3."]
pub struct TS3_3_R(crate::FieldReader<bool, TS3_3_A>);
impl TS3_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS3_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS3_3_A {
        match self.bits {
            false => TS3_3_A::IDLE_THERE_IS_NO_TR,
            true => TS3_3_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        **self == TS3_3_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        **self == TS3_3_A::TRANSMIT_THE_CAN_CO
    }
}
impl core::ops::Deref for TS3_3_R {
    type Target = crate::FieldReader<bool, TS3_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ES_3` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub struct ES_3_R(crate::FieldReader<bool, bool>);
impl ES_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ES_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ES_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BS_3` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub struct BS_3_R(crate::FieldReader<bool, bool>);
impl BS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_1(&self) -> RBS_1_R {
        RBS_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_1(&self) -> DOS_1_R {
        DOS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status 1."]
    #[inline(always)]
    pub fn tbs1_1(&self) -> TBS1_1_R {
        TBS1_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs1_1(&self) -> TCS1_1_R {
        TCS1_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_1(&self) -> RS_1_R {
        RS_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Status 1."]
    #[inline(always)]
    pub fn ts1_1(&self) -> TS1_1_R {
        TS1_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_1(&self) -> ES_1_R {
        ES_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_1(&self) -> BS_1_R {
        BS_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_2(&self) -> RBS_2_R {
        RBS_2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_2(&self) -> DOS_2_R {
        DOS_2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Buffer Status 2."]
    #[inline(always)]
    pub fn tbs2_2(&self) -> TBS2_2_R {
        TBS2_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs2_2(&self) -> TCS2_2_R {
        TCS2_2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_2(&self) -> RS_2_R {
        RS_2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status 2."]
    #[inline(always)]
    pub fn ts2_2(&self) -> TS2_2_R {
        TS2_2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_2(&self) -> ES_2_R {
        ES_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_2(&self) -> BS_2_R {
        BS_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_3(&self) -> RBS_3_R {
        RBS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_3(&self) -> DOS_3_R {
        DOS_3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Buffer Status 3."]
    #[inline(always)]
    pub fn tbs3_3(&self) -> TBS3_3_R {
        TBS3_3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs3_3(&self) -> TCS3_3_R {
        TCS3_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_3(&self) -> RS_3_R {
        RS_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Status 3."]
    #[inline(always)]
    pub fn ts3_3(&self) -> TS3_3_R {
        TS3_3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_3(&self) -> ES_3_R {
        ES_3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_3(&self) -> BS_3_R {
        BS_3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x003c_3c3c"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003c_3c3c
    }
}
