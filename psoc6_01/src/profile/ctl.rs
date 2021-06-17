#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN_MODE` reader - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub struct WIN_MODE_R(crate::FieldReader<bool, bool>);
impl WIN_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIN_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIN_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIN_MODE` writer - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub struct WIN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_MODE_W<'a> {
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
#[doc = "Field `ENABLED` reader - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&self) -> WIN_MODE_R {
        WIN_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&mut self) -> WIN_MODE_W {
        WIN_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Profile control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
