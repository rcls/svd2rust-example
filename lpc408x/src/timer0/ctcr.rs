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
#[doc = "Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTMODE_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE = 3,
}
impl From<CTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTMODE` reader - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub struct CTMODE_R(crate::FieldReader<u8, CTMODE_A>);
impl CTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTMODE_A {
        match self.bits {
            0 => CTMODE_A::TIMER_MODE_EVERY_RI,
            1 => CTMODE_A::RISING,
            2 => CTMODE_A::FALLING,
            3 => CTMODE_A::DUALEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline(always)]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        **self == CTMODE_A::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == CTMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == CTMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `DUALEDGE`"]
    #[inline(always)]
    pub fn is_dualedge(&self) -> bool {
        **self == CTMODE_A::DUALEDGE
    }
}
impl core::ops::Deref for CTMODE_R {
    type Target = crate::FieldReader<u8, CTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMODE` writer - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub struct CTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTMODE_A::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTMODE_A::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CTMODE_A::FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dualedge(self) -> &'a mut W {
        self.variant(CTMODE_A::DUALEDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CINSEL_A {
    #[doc = "0: CAPn.0 for TIMERn"]
    CAPN_0_FOR_TIMERN = 0,
    #[doc = "1: CAPn.1 for TIMERn"]
    CAPN_1_FOR_TIMERN = 1,
}
impl From<CINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub struct CINSEL_R(crate::FieldReader<u8, CINSEL_A>);
impl CINSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSEL_A {
        match self.bits {
            0 => CINSEL_A::CAPN_0_FOR_TIMERN,
            1 => CINSEL_A::CAPN_1_FOR_TIMERN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPN_0_FOR_TIMERN`"]
    #[inline(always)]
    pub fn is_capn_0_for_timern(&self) -> bool {
        **self == CINSEL_A::CAPN_0_FOR_TIMERN
    }
    #[doc = "Checks if the value of the field is `CAPN_1_FOR_TIMERN`"]
    #[inline(always)]
    pub fn is_capn_1_for_timern(&self) -> bool {
        **self == CINSEL_A::CAPN_1_FOR_TIMERN
    }
}
impl core::ops::Deref for CINSEL_R {
    type Target = crate::FieldReader<u8, CINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub struct CINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn capn_0_for_timern(self) -> &'a mut W {
        self.variant(CINSEL_A::CAPN_0_FOR_TIMERN)
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn capn_1_for_timern(self) -> &'a mut W {
        self.variant(CINSEL_A::CAPN_1_FOR_TIMERN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&mut self) -> CTMODE_W {
        CTMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&mut self) -> CINSEL_W {
        CINSEL_W { w: self }
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
