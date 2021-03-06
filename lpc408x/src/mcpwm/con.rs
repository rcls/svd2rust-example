#[doc = "Register `CON` reader"]
pub struct R(crate::R<CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Stops/starts timer channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN0_A {
    #[doc = "0: Stop."]
    STOP_ = 0,
    #[doc = "1: Run."]
    RUN_ = 1,
}
impl From<RUN0_A> for bool {
    #[inline(always)]
    fn from(variant: RUN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN0` reader - Stops/starts timer channel 0."]
pub struct RUN0_R(crate::FieldReader<bool, RUN0_A>);
impl RUN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN0_A {
        match self.bits {
            false => RUN0_A::STOP_,
            true => RUN0_A::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        **self == RUN0_A::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        **self == RUN0_A::RUN_
    }
}
impl core::ops::Deref for RUN0_R {
    type Target = crate::FieldReader<bool, RUN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Edge/center aligned operation for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER0_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED_ = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED_ = 1,
}
impl From<CENTER0_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER0` reader - Edge/center aligned operation for channel 0."]
pub struct CENTER0_R(crate::FieldReader<bool, CENTER0_A>);
impl CENTER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CENTER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENTER0_A {
        match self.bits {
            false => CENTER0_A::EDGE_ALIGNED_,
            true => CENTER0_A::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        **self == CENTER0_A::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        **self == CENTER0_A::CENTER_ALIGNED_
    }
}
impl core::ops::Deref for CENTER0_R {
    type Target = crate::FieldReader<bool, CENTER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Selects polarity of the MCOA0 and MCOB0 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA0_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG = 1,
}
impl From<POLA0_A> for bool {
    #[inline(always)]
    fn from(variant: POLA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA0` reader - Selects polarity of the MCOA0 and MCOB0 pins."]
pub struct POLA0_R(crate::FieldReader<bool, POLA0_A>);
impl POLA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLA0_A {
        match self.bits {
            false => POLA0_A::PASSIVE_STATE_IS_LOW,
            true => POLA0_A::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        **self == POLA0_A::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        **self == POLA0_A::PASSIVE_STATE_IS_HIG
    }
}
impl core::ops::Deref for POLA0_R {
    type Target = crate::FieldReader<bool, POLA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Controls the dead-time feature for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE0_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED_ = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED_ = 1,
}
impl From<DTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DTE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE0` reader - Controls the dead-time feature for channel 0."]
pub struct DTE0_R(crate::FieldReader<bool, DTE0_A>);
impl DTE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTE0_A {
        match self.bits {
            false => DTE0_A::DEAD_TIME_DISABLED_,
            true => DTE0_A::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        **self == DTE0_A::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        **self == DTE0_A::DEAD_TIME_ENABLED_
    }
}
impl core::ops::Deref for DTE0_R {
    type Target = crate::FieldReader<bool, DTE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable/disable updates of functional registers for channel 0 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP0_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP0_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP0` reader - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
pub struct DISUP0_R(crate::FieldReader<bool, DISUP0_A>);
impl DISUP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISUP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISUP0_A {
        match self.bits {
            false => DISUP0_A::UPDATE,
            true => DISUP0_A::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == DISUP0_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        **self == DISUP0_A::NOUPDATE
    }
}
impl core::ops::Deref for DISUP0_R {
    type Target = crate::FieldReader<bool, DISUP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stops/starts timer channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN1_A {
    #[doc = "0: Stop."]
    STOP_ = 0,
    #[doc = "1: Run."]
    RUN_ = 1,
}
impl From<RUN1_A> for bool {
    #[inline(always)]
    fn from(variant: RUN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN1` reader - Stops/starts timer channel 1."]
pub struct RUN1_R(crate::FieldReader<bool, RUN1_A>);
impl RUN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN1_A {
        match self.bits {
            false => RUN1_A::STOP_,
            true => RUN1_A::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        **self == RUN1_A::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        **self == RUN1_A::RUN_
    }
}
impl core::ops::Deref for RUN1_R {
    type Target = crate::FieldReader<bool, RUN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Edge/center aligned operation for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER1_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED_ = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED_ = 1,
}
impl From<CENTER1_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER1` reader - Edge/center aligned operation for channel 1."]
pub struct CENTER1_R(crate::FieldReader<bool, CENTER1_A>);
impl CENTER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CENTER1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENTER1_A {
        match self.bits {
            false => CENTER1_A::EDGE_ALIGNED_,
            true => CENTER1_A::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        **self == CENTER1_A::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        **self == CENTER1_A::CENTER_ALIGNED_
    }
}
impl core::ops::Deref for CENTER1_R {
    type Target = crate::FieldReader<bool, CENTER1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Selects polarity of the MCOA1 and MCOB1 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA1_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG = 1,
}
impl From<POLA1_A> for bool {
    #[inline(always)]
    fn from(variant: POLA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA1` reader - Selects polarity of the MCOA1 and MCOB1 pins."]
pub struct POLA1_R(crate::FieldReader<bool, POLA1_A>);
impl POLA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLA1_A {
        match self.bits {
            false => POLA1_A::PASSIVE_STATE_IS_LOW,
            true => POLA1_A::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        **self == POLA1_A::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        **self == POLA1_A::PASSIVE_STATE_IS_HIG
    }
}
impl core::ops::Deref for POLA1_R {
    type Target = crate::FieldReader<bool, POLA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE1_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED_ = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED_ = 1,
}
impl From<DTE1_A> for bool {
    #[inline(always)]
    fn from(variant: DTE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE1` reader - Controls the dead-time feature for channel 1."]
pub struct DTE1_R(crate::FieldReader<bool, DTE1_A>);
impl DTE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTE1_A {
        match self.bits {
            false => DTE1_A::DEAD_TIME_DISABLED_,
            true => DTE1_A::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        **self == DTE1_A::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        **self == DTE1_A::DEAD_TIME_ENABLED_
    }
}
impl core::ops::Deref for DTE1_R {
    type Target = crate::FieldReader<bool, DTE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable/disable updates of functional registers for channel 1 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP1_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP1_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP1` reader - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
pub struct DISUP1_R(crate::FieldReader<bool, DISUP1_A>);
impl DISUP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISUP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISUP1_A {
        match self.bits {
            false => DISUP1_A::UPDATE,
            true => DISUP1_A::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == DISUP1_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        **self == DISUP1_A::NOUPDATE
    }
}
impl core::ops::Deref for DISUP1_R {
    type Target = crate::FieldReader<bool, DISUP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stops/starts timer channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN2_A {
    #[doc = "0: Stop."]
    STOP_ = 0,
    #[doc = "1: Run."]
    RUN_ = 1,
}
impl From<RUN2_A> for bool {
    #[inline(always)]
    fn from(variant: RUN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN2` reader - Stops/starts timer channel 2."]
pub struct RUN2_R(crate::FieldReader<bool, RUN2_A>);
impl RUN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN2_A {
        match self.bits {
            false => RUN2_A::STOP_,
            true => RUN2_A::RUN_,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        **self == RUN2_A::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        **self == RUN2_A::RUN_
    }
}
impl core::ops::Deref for RUN2_R {
    type Target = crate::FieldReader<bool, RUN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Edge/center aligned operation for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER2_A {
    #[doc = "0: Edge-aligned."]
    EDGE_ALIGNED_ = 0,
    #[doc = "1: Center-aligned."]
    CENTER_ALIGNED_ = 1,
}
impl From<CENTER2_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER2` reader - Edge/center aligned operation for channel 2."]
pub struct CENTER2_R(crate::FieldReader<bool, CENTER2_A>);
impl CENTER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CENTER2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENTER2_A {
        match self.bits {
            false => CENTER2_A::EDGE_ALIGNED_,
            true => CENTER2_A::CENTER_ALIGNED_,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        **self == CENTER2_A::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        **self == CENTER2_A::CENTER_ALIGNED_
    }
}
impl core::ops::Deref for CENTER2_R {
    type Target = crate::FieldReader<bool, CENTER2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Selects polarity of the MCOA2 and MCOB2 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA2_A {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG = 1,
}
impl From<POLA2_A> for bool {
    #[inline(always)]
    fn from(variant: POLA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA2` reader - Selects polarity of the MCOA2 and MCOB2 pins."]
pub struct POLA2_R(crate::FieldReader<bool, POLA2_A>);
impl POLA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLA2_A {
        match self.bits {
            false => POLA2_A::PASSIVE_STATE_IS_LOW,
            true => POLA2_A::PASSIVE_STATE_IS_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        **self == POLA2_A::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        **self == POLA2_A::PASSIVE_STATE_IS_HIG
    }
}
impl core::ops::Deref for POLA2_R {
    type Target = crate::FieldReader<bool, POLA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE2_A {
    #[doc = "0: Dead-time disabled."]
    DEAD_TIME_DISABLED_ = 0,
    #[doc = "1: Dead-time enabled."]
    DEAD_TIME_ENABLED_ = 1,
}
impl From<DTE2_A> for bool {
    #[inline(always)]
    fn from(variant: DTE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE2` reader - Controls the dead-time feature for channel 1."]
pub struct DTE2_R(crate::FieldReader<bool, DTE2_A>);
impl DTE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTE2_A {
        match self.bits {
            false => DTE2_A::DEAD_TIME_DISABLED_,
            true => DTE2_A::DEAD_TIME_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        **self == DTE2_A::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        **self == DTE2_A::DEAD_TIME_ENABLED_
    }
}
impl core::ops::Deref for DTE2_R {
    type Target = crate::FieldReader<bool, DTE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable/disable updates of functional registers for channel 2 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP2_A {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    NOUPDATE = 1,
}
impl From<DISUP2_A> for bool {
    #[inline(always)]
    fn from(variant: DISUP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP2` reader - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
pub struct DISUP2_R(crate::FieldReader<bool, DISUP2_A>);
impl DISUP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISUP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISUP2_A {
        match self.bits {
            false => DISUP2_A::UPDATE,
            true => DISUP2_A::NOUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == DISUP2_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        **self == DISUP2_A::NOUPDATE
    }
}
impl core::ops::Deref for DISUP2_R {
    type Target = crate::FieldReader<bool, DISUP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVBDC_A {
    #[doc = "0: The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    OPPOSITE = 0,
    #[doc = "1: The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    SAME = 1,
}
impl From<INVBDC_A> for bool {
    #[inline(always)]
    fn from(variant: INVBDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVBDC` reader - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
pub struct INVBDC_R(crate::FieldReader<bool, INVBDC_A>);
impl INVBDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVBDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVBDC_A {
        match self.bits {
            false => INVBDC_A::OPPOSITE,
            true => INVBDC_A::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `OPPOSITE`"]
    #[inline(always)]
    pub fn is_opposite(&self) -> bool {
        **self == INVBDC_A::OPPOSITE
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        **self == INVBDC_A::SAME
    }
}
impl core::ops::Deref for INVBDC_R {
    type Target = crate::FieldReader<bool, INVBDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "3-phase AC mode select (see Section 24.8.7).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMODE_A {
    #[doc = "0: 3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    _3_PHASE_AC_MODE_OFF = 0,
    #[doc = "1: 3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    _3_PHASE_AC_MODE_ON_ = 1,
}
impl From<ACMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMODE` reader - 3-phase AC mode select (see Section 24.8.7)."]
pub struct ACMODE_R(crate::FieldReader<bool, ACMODE_A>);
impl ACMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMODE_A {
        match self.bits {
            false => ACMODE_A::_3_PHASE_AC_MODE_OFF,
            true => ACMODE_A::_3_PHASE_AC_MODE_ON_,
        }
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_OFF`"]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_off(&self) -> bool {
        **self == ACMODE_A::_3_PHASE_AC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_ON_`"]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_on_(&self) -> bool {
        **self == ACMODE_A::_3_PHASE_AC_MODE_ON_
    }
}
impl core::ops::Deref for ACMODE_R {
    type Target = crate::FieldReader<bool, ACMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "3-phase DC mode select (see Section 24.8.6).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMODE_A {
    #[doc = "0: 3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    _3_PHASE_DC_MODE_OFF = 0,
    #[doc = "1: 3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    _3_PHASE_DC_MODE_ON_ = 1,
}
impl From<DCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMODE` reader - 3-phase DC mode select (see Section 24.8.6)."]
pub struct DCMODE_R(crate::FieldReader<bool, DCMODE_A>);
impl DCMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMODE_A {
        match self.bits {
            false => DCMODE_A::_3_PHASE_DC_MODE_OFF,
            true => DCMODE_A::_3_PHASE_DC_MODE_ON_,
        }
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_OFF`"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_off(&self) -> bool {
        **self == DCMODE_A::_3_PHASE_DC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_ON_`"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_on_(&self) -> bool {
        **self == DCMODE_A::_3_PHASE_DC_MODE_ON_
    }
}
impl core::ops::Deref for DCMODE_R {
    type Target = crate::FieldReader<bool, DCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline(always)]
    pub fn run0(&self) -> RUN0_R {
        RUN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline(always)]
    pub fn center0(&self) -> CENTER0_R {
        CENTER0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline(always)]
    pub fn pola0(&self) -> POLA0_R {
        POLA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline(always)]
    pub fn dte0(&self) -> DTE0_R {
        DTE0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup0(&self) -> DISUP0_R {
        DISUP0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline(always)]
    pub fn run1(&self) -> RUN1_R {
        RUN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline(always)]
    pub fn center1(&self) -> CENTER1_R {
        CENTER1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline(always)]
    pub fn pola1(&self) -> POLA1_R {
        POLA1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte1(&self) -> DTE1_R {
        DTE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup1(&self) -> DISUP1_R {
        DISUP1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline(always)]
    pub fn run2(&self) -> RUN2_R {
        RUN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline(always)]
    pub fn center2(&self) -> CENTER2_R {
        CENTER2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline(always)]
    pub fn pola2(&self) -> POLA2_R {
        POLA2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte2(&self) -> DTE2_R {
        DTE2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup2(&self) -> DISUP2_R {
        DISUP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline(always)]
    pub fn invbdc(&self) -> INVBDC_R {
        INVBDC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline(always)]
    pub fn acmode(&self) -> ACMODE_R {
        ACMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline(always)]
    pub fn dcmode(&self) -> DCMODE_R {
        DCMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "PWM Control read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](index.html) module"]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [con::R](R) reader structure"]
impl crate::Readable for CON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
