#[doc = "Register `CTRL_CLR` reader"]
pub struct R(crate::R<CTRL_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_CLR` writer"]
pub struct W(crate::W<CTRL_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_CLR_SPEC>;
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
impl From<crate::W<CTRL_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_ENABLED` reader - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub struct COUNTER_ENABLED_R(crate::FieldReader<u32, u32>);
impl COUNTER_ENABLED_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNTER_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_ENABLED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTER_ENABLED` writer - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub struct COUNTER_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_ENABLED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> COUNTER_ENABLED_R {
        COUNTER_ENABLED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&mut self) -> COUNTER_ENABLED_W {
        COUNTER_ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM control clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](index.html) module"]
pub struct CTRL_CLR_SPEC;
impl crate::RegisterSpec for CTRL_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_clr::R](R) reader structure"]
impl crate::Readable for CTRL_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](W) writer structure"]
impl crate::Writable for CTRL_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_CLR to value 0"]
impl crate::Resettable for CTRL_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
