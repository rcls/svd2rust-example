#[doc = "Register `CMD_RELOAD` reader"]
pub struct R(crate::R<CMD_RELOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RELOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RELOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RELOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_RELOAD` writer"]
pub struct W(crate::W<CMD_RELOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_RELOAD_SPEC>;
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
impl From<crate::W<CMD_RELOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_RELOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_RELOAD` reader - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_RELOAD_R(crate::FieldReader<u32, u32>);
impl COUNTER_RELOAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNTER_RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_RELOAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_RELOAD` writer - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub struct COUNTER_RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> COUNTER_RELOAD_R {
        COUNTER_RELOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&mut self) -> COUNTER_RELOAD_W {
        COUNTER_RELOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM reload command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_reload](index.html) module"]
pub struct CMD_RELOAD_SPEC;
impl crate::RegisterSpec for CMD_RELOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_reload::R](R) reader structure"]
impl crate::Readable for CMD_RELOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_reload::W](W) writer structure"]
impl crate::Writable for CMD_RELOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_RELOAD to value 0"]
impl crate::Resettable for CMD_RELOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
