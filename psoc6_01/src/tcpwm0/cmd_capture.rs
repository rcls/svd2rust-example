#[doc = "Register `CMD_CAPTURE` reader"]
pub struct R(crate::R<CMD_CAPTURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_CAPTURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_CAPTURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_CAPTURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_CAPTURE` writer"]
pub struct W(crate::W<CMD_CAPTURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_CAPTURE_SPEC>;
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
impl From<crate::W<CMD_CAPTURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_CAPTURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_CAPTURE` reader - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub struct COUNTER_CAPTURE_R(crate::FieldReader<u32, u32>);
impl COUNTER_CAPTURE_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNTER_CAPTURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_CAPTURE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_CAPTURE` writer - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub struct COUNTER_CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_CAPTURE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> COUNTER_CAPTURE_R {
        COUNTER_CAPTURE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&mut self) -> COUNTER_CAPTURE_W {
        COUNTER_CAPTURE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM capture command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_capture](index.html) module"]
pub struct CMD_CAPTURE_SPEC;
impl crate::RegisterSpec for CMD_CAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_capture::R](R) reader structure"]
impl crate::Readable for CMD_CAPTURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_capture::W](W) writer structure"]
impl crate::Writable for CMD_CAPTURE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_CAPTURE to value 0"]
impl crate::Resettable for CMD_CAPTURE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
