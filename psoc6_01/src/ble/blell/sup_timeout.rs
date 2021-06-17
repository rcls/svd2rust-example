#[doc = "Register `SUP_TIMEOUT` reader"]
pub struct R(crate::R<SUP_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUP_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUP_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUP_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUP_TIMEOUT` writer"]
pub struct W(crate::W<SUP_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUP_TIMEOUT_SPEC>;
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
impl From<crate::W<SUP_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUP_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUPERVISION_TIMEOUT` reader - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
pub struct SUPERVISION_TIMEOUT_R(crate::FieldReader<u16, u16>);
impl SUPERVISION_TIMEOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        SUPERVISION_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPERVISION_TIMEOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPERVISION_TIMEOUT` writer - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
pub struct SUPERVISION_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPERVISION_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout(&self) -> SUPERVISION_TIMEOUT_R {
        SUPERVISION_TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout(&mut self) -> SUPERVISION_TIMEOUT_W {
        SUPERVISION_TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supervision timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sup_timeout](index.html) module"]
pub struct SUP_TIMEOUT_SPEC;
impl crate::RegisterSpec for SUP_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sup_timeout::R](R) reader structure"]
impl crate::Readable for SUP_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sup_timeout::W](W) writer structure"]
impl crate::Writable for SUP_TIMEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUP_TIMEOUT to value 0"]
impl crate::Resettable for SUP_TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
