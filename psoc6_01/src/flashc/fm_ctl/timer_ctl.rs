#[doc = "Register `TIMER_CTL` reader"]
pub struct R(crate::R<TIMER_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_CTL` writer"]
pub struct W(crate::W<TIMER_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CTL_SPEC>;
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
impl From<crate::W<TIMER_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub struct PERIOD_R(crate::FieldReader<u16, u16>);
impl PERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SCALE` reader - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
pub struct SCALE_R(crate::FieldReader<bool, bool>);
impl SCALE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PUMP_CLOCK_SEL` reader - Pump clock select: '0': internal clock. '1': external clock."]
pub struct PUMP_CLOCK_SEL_R(crate::FieldReader<bool, bool>);
impl PUMP_CLOCK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUMP_CLOCK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMP_CLOCK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMP_CLOCK_SEL` writer - Pump clock select: '0': internal clock. '1': external clock."]
pub struct PUMP_CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_CLOCK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PRE_PROG` reader - '1' during pre-program operation"]
pub struct PRE_PROG_R(crate::FieldReader<bool, bool>);
impl PRE_PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRE_PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_PROG` writer - '1' during pre-program operation"]
pub struct PRE_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PROG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PRE_PROG_CSL` reader - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
pub struct PRE_PROG_CSL_R(crate::FieldReader<bool, bool>);
impl PRE_PROG_CSL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRE_PROG_CSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_PROG_CSL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_PROG_CSL` writer - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
pub struct PRE_PROG_CSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PROG_CSL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PUMP_EN` reader - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub struct PUMP_EN_R(crate::FieldReader<bool, bool>);
impl PUMP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMP_EN` writer - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub struct PUMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ACLK_EN` reader - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub struct ACLK_EN_R(crate::FieldReader<bool, bool>);
impl ACLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLK_EN` writer - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub struct ACLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `TIMER_EN` reader - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub struct TIMER_EN_R(crate::FieldReader<bool, bool>);
impl TIMER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_EN` writer - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub struct TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_EN_W<'a> {
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
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&self) -> PUMP_CLOCK_SEL_R {
        PUMP_CLOCK_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&self) -> PRE_PROG_R {
        PRE_PROG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&self) -> PRE_PROG_CSL_R {
        PRE_PROG_CSL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&self) -> ACLK_EN_R {
        ACLK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&mut self) -> PUMP_CLOCK_SEL_W {
        PUMP_CLOCK_SEL_W { w: self }
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&mut self) -> PRE_PROG_W {
        PRE_PROG_W { w: self }
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&mut self) -> PRE_PROG_CSL_W {
        PRE_PROG_CSL_W { w: self }
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&mut self) -> PUMP_EN_W {
        PUMP_EN_W { w: self }
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&mut self) -> ACLK_EN_W {
        ACLK_EN_W { w: self }
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W {
        TIMER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctl](index.html) module"]
pub struct TIMER_CTL_SPEC;
impl crate::RegisterSpec for TIMER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_ctl::R](R) reader structure"]
impl crate::Readable for TIMER_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_ctl::W](W) writer structure"]
impl crate::Writable for TIMER_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_CTL to value 0x0400_0000"]
impl crate::Resettable for TIMER_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
