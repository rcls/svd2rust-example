#[doc = "Register `ERSTATUS` reader"]
pub struct R(crate::R<ERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERSTATUS` writer"]
pub struct W(crate::W<ERSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ERSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0_A {
    #[doc = "0: No event change on channel 0."]
    NO_EVENT_CHANGE_ON_C = 0,
    #[doc = "1: At least one event has occurred on channel 0."]
    AT_LEAST_ONE_EVENT_H = 1,
}
impl From<EV0_A> for bool {
    #[inline(always)]
    fn from(variant: EV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV0_R(crate::FieldReader<bool, EV0_A>);
impl EV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0_A {
        match self.bits {
            false => EV0_A::NO_EVENT_CHANGE_ON_C,
            true => EV0_A::AT_LEAST_ONE_EVENT_H,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT_CHANGE_ON_C`"]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        **self == EV0_A::NO_EVENT_CHANGE_ON_C
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_EVENT_H`"]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        **self == EV0_A::AT_LEAST_ONE_EVENT_H
    }
}
impl core::ops::Deref for EV0_R {
    type Target = crate::FieldReader<bool, EV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV0` writer - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV0_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event change on channel 0."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut W {
        self.variant(EV0_A::NO_EVENT_CHANGE_ON_C)
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut W {
        self.variant(EV0_A::AT_LEAST_ONE_EVENT_H)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1_A {
    #[doc = "0: No event change on channel 1."]
    NO_EVENT_CHANGE_ON_C = 0,
    #[doc = "1: At least one event has occurred on channel 1."]
    AT_LEAST_ONE_EVENT_H = 1,
}
impl From<EV1_A> for bool {
    #[inline(always)]
    fn from(variant: EV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV1_R(crate::FieldReader<bool, EV1_A>);
impl EV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV1_A {
        match self.bits {
            false => EV1_A::NO_EVENT_CHANGE_ON_C,
            true => EV1_A::AT_LEAST_ONE_EVENT_H,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT_CHANGE_ON_C`"]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        **self == EV1_A::NO_EVENT_CHANGE_ON_C
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_EVENT_H`"]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        **self == EV1_A::AT_LEAST_ONE_EVENT_H
    }
}
impl core::ops::Deref for EV1_R {
    type Target = crate::FieldReader<bool, EV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV1` writer - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV1_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event change on channel 1."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut W {
        self.variant(EV1_A::NO_EVENT_CHANGE_ON_C)
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut W {
        self.variant(EV1_A::AT_LEAST_ONE_EVENT_H)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2_A {
    #[doc = "0: No event change on channel 2."]
    NO_EVENT_CHANGE_ON_C = 0,
    #[doc = "1: At least one event has occurred on channel 2."]
    AT_LEAST_ONE_EVENT_H = 1,
}
impl From<EV2_A> for bool {
    #[inline(always)]
    fn from(variant: EV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` reader - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV2_R(crate::FieldReader<bool, EV2_A>);
impl EV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EV2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV2_A {
        match self.bits {
            false => EV2_A::NO_EVENT_CHANGE_ON_C,
            true => EV2_A::AT_LEAST_ONE_EVENT_H,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT_CHANGE_ON_C`"]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        **self == EV2_A::NO_EVENT_CHANGE_ON_C
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_EVENT_H`"]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        **self == EV2_A::AT_LEAST_ONE_EVENT_H
    }
}
impl core::ops::Deref for EV2_R {
    type Target = crate::FieldReader<bool, EV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV2` writer - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct EV2_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No event change on channel 2."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut W {
        self.variant(EV2_A::NO_EVENT_CHANGE_ON_C)
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut W {
        self.variant(EV2_A::AT_LEAST_ONE_EVENT_H)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GP_CLEARED_A {
    #[doc = "0: General purpose registers have not been asynchronous cleared."]
    NOGPCLR = 0,
    #[doc = "1: General purpose registers have been asynchronous cleared."]
    GPCLR = 1,
}
impl From<GP_CLEARED_A> for bool {
    #[inline(always)]
    fn from(variant: GP_CLEARED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GP_CLEARED` reader - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct GP_CLEARED_R(crate::FieldReader<bool, GP_CLEARED_A>);
impl GP_CLEARED_R {
    pub(crate) fn new(bits: bool) -> Self {
        GP_CLEARED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GP_CLEARED_A {
        match self.bits {
            false => GP_CLEARED_A::NOGPCLR,
            true => GP_CLEARED_A::GPCLR,
        }
    }
    #[doc = "Checks if the value of the field is `NOGPCLR`"]
    #[inline(always)]
    pub fn is_nogpclr(&self) -> bool {
        **self == GP_CLEARED_A::NOGPCLR
    }
    #[doc = "Checks if the value of the field is `GPCLR`"]
    #[inline(always)]
    pub fn is_gpclr(&self) -> bool {
        **self == GP_CLEARED_A::GPCLR
    }
}
impl core::ops::Deref for GP_CLEARED_R {
    type Target = crate::FieldReader<bool, GP_CLEARED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_CLEARED` writer - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct GP_CLEARED_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_CLEARED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GP_CLEARED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline(always)]
    pub fn nogpclr(self) -> &'a mut W {
        self.variant(GP_CLEARED_A::NOGPCLR)
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline(always)]
    pub fn gpclr(self) -> &'a mut W {
        self.variant(GP_CLEARED_A::GPCLR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_A {
    #[doc = "0: No interrupt/wakeup request is pending"]
    NO_INTERRUPTWAKEUP_ = 0,
    #[doc = "1: An interrupt/wakeup request is pending."]
    INTWAKEUP_PEND = 1,
}
impl From<WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct WAKEUP_R(crate::FieldReader<bool, WAKEUP_A>);
impl WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_A {
        match self.bits {
            false => WAKEUP_A::NO_INTERRUPTWAKEUP_,
            true => WAKEUP_A::INTWAKEUP_PEND,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPTWAKEUP_`"]
    #[inline(always)]
    pub fn is_no_interruptwakeup_(&self) -> bool {
        **self == WAKEUP_A::NO_INTERRUPTWAKEUP_
    }
    #[doc = "Checks if the value of the field is `INTWAKEUP_PEND`"]
    #[inline(always)]
    pub fn is_intwakeup_pend(&self) -> bool {
        **self == WAKEUP_A::INTWAKEUP_PEND
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, WAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt/wakeup request is pending"]
    #[inline(always)]
    pub fn no_interruptwakeup_(self) -> &'a mut W {
        self.variant(WAKEUP_A::NO_INTERRUPTWAKEUP_)
    }
    #[doc = "An interrupt/wakeup request is pending."]
    #[inline(always)]
    pub fn intwakeup_pend(self) -> &'a mut W {
        self.variant(WAKEUP_A::INTWAKEUP_PEND)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev0(&self) -> EV0_R {
        EV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev1(&self) -> EV1_R {
        EV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev2(&self) -> EV2_R {
        EV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn gp_cleared(&self) -> GP_CLEARED_R {
        GP_CLEARED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev0(&mut self) -> EV0_W {
        EV0_W { w: self }
    }
    #[doc = "Bit 1 - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev1(&mut self) -> EV1_W {
        EV1_W { w: self }
    }
    #[doc = "Bit 2 - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev2(&mut self) -> EV2_W {
        EV2_W { w: self }
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn gp_cleared(&mut self) -> GP_CLEARED_W {
        GP_CLEARED_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Monitor/Recorder Status register. Contains status flags for event channels and other Event Monitor/Recorder conditions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erstatus](index.html) module"]
pub struct ERSTATUS_SPEC;
impl crate::RegisterSpec for ERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erstatus::R](R) reader structure"]
impl crate::Readable for ERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erstatus::W](W) writer structure"]
impl crate::Writable for ERSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERSTATUS to value 0"]
impl crate::Resettable for ERSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
