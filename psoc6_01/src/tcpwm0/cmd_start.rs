#[doc = "Register `CMD_START` reader"]
pub struct R(crate::R<CMD_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_START` writer"]
pub struct W(crate::W<CMD_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_START_SPEC>;
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
impl From<crate::W<CMD_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_START` reader - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_START_R(crate::FieldReader<u32, u32>);
impl COUNTER_START_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNTER_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_START_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_START` writer - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_START_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&self) -> COUNTER_START_R {
        COUNTER_START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&mut self) -> COUNTER_START_W {
        COUNTER_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM start command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_start](index.html) module"]
pub struct CMD_START_SPEC;
impl crate::RegisterSpec for CMD_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_start::R](R) reader structure"]
impl crate::Readable for CMD_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_start::W](W) writer structure"]
impl crate::Writable for CMD_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_START to value 0"]
impl crate::Resettable for CMD_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
