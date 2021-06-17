#[doc = "Register `CTCR` reader"]
pub struct R(crate::R<CTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTCR` writer"]
pub struct W(crate::W<CTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTCR_SPEC>;
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
impl From<crate::W<CTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/ Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_A {
    #[doc = "0: Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TIMER_MODE_THE_TC_I = 0,
    #[doc = "1: Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RISING_EDGE_COUNTER_ = 1,
    #[doc = "2: Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FALLING_EDGE_COUNTER = 2,
    #[doc = "3: Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DUAL_EDGE_COUNTER_MO = 3,
}
impl From<MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MOD` reader - Counter/ Timer Mode"]
pub struct MOD_R(crate::FieldReader<u8, MOD_A>);
impl MOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        MOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_A {
        match self.bits {
            0 => MOD_A::TIMER_MODE_THE_TC_I,
            1 => MOD_A::RISING_EDGE_COUNTER_,
            2 => MOD_A::FALLING_EDGE_COUNTER,
            3 => MOD_A::DUAL_EDGE_COUNTER_MO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_THE_TC_I`"]
    #[inline(always)]
    pub fn is_timer_mode_the_tc_i(&self) -> bool {
        **self == MOD_A::TIMER_MODE_THE_TC_I
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_COUNTER_`"]
    #[inline(always)]
    pub fn is_rising_edge_counter_(&self) -> bool {
        **self == MOD_A::RISING_EDGE_COUNTER_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_falling_edge_counter(&self) -> bool {
        **self == MOD_A::FALLING_EDGE_COUNTER
    }
    #[doc = "Checks if the value of the field is `DUAL_EDGE_COUNTER_MO`"]
    #[inline(always)]
    pub fn is_dual_edge_counter_mo(&self) -> bool {
        **self == MOD_A::DUAL_EDGE_COUNTER_MO
    }
}
impl core::ops::Deref for MOD_R {
    type Target = crate::FieldReader<u8, MOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD` writer - Counter/ Timer Mode"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer_mode_the_tc_i(self) -> &'a mut W {
        self.variant(MOD_A::TIMER_MODE_THE_TC_I)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising_edge_counter_(self) -> &'a mut W {
        self.variant(MOD_A::RISING_EDGE_COUNTER_)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling_edge_counter(self) -> &'a mut W {
        self.variant(MOD_A::FALLING_EDGE_COUNTER)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dual_edge_counter_mo(self) -> &'a mut W {
        self.variant(MOD_A::DUAL_EDGE_COUNTER_MO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    FOR_PWM0_00_EQ_PWM0_ = 0,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CIS` reader - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub struct CIS_R(crate::FieldReader<u8, CIS_A>);
impl CIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIS_A> {
        match self.bits {
            0 => Some(CIS_A::FOR_PWM0_00_EQ_PWM0_),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FOR_PWM0_00_EQ_PWM0_`"]
    #[inline(always)]
    pub fn is_for_pwm0_00_eq_pwm0_(&self) -> bool {
        **self == CIS_A::FOR_PWM0_00_EQ_PWM0_
    }
}
impl core::ops::Deref for CIS_R {
    type Target = crate::FieldReader<u8, CIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIS` writer - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn for_pwm0_00_eq_pwm0_(self) -> &'a mut W {
        self.variant(CIS_A::FOR_PWM0_00_EQ_PWM0_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctcr](index.html) module"]
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctcr::R](R) reader structure"]
impl crate::Readable for CTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctcr::W](W) writer structure"]
impl crate::Writable for CTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
