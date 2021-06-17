#[doc = "Register `CONN_PARAM_NEXT_SUP_TO` reader"]
pub struct R(crate::R<CONN_PARAM_NEXT_SUP_TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_PARAM_NEXT_SUP_TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_PARAM_NEXT_SUP_TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_PARAM_NEXT_SUP_TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_PARAM_NEXT_SUP_TO` writer"]
pub struct W(crate::W<CONN_PARAM_NEXT_SUP_TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_PARAM_NEXT_SUP_TO_SPEC>;
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
impl From<crate::W<CONN_PARAM_NEXT_SUP_TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_PARAM_NEXT_SUP_TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXT_SUP_TO_LOAD` reader - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
pub struct NEXT_SUP_TO_LOAD_R(crate::FieldReader<u16, u16>);
impl NEXT_SUP_TO_LOAD_R {
    pub(crate) fn new(bits: u16) -> Self {
        NEXT_SUP_TO_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_SUP_TO_LOAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT_SUP_TO_LOAD` writer - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
pub struct NEXT_SUP_TO_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_SUP_TO_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn next_sup_to_load(&self) -> NEXT_SUP_TO_LOAD_R {
        NEXT_SUP_TO_LOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn next_sup_to_load(&mut self) -> NEXT_SUP_TO_LOAD_W {
        NEXT_SUP_TO_LOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to configure the supervision timeout for next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param_next_sup_to](index.html) module"]
pub struct CONN_PARAM_NEXT_SUP_TO_SPEC;
impl crate::RegisterSpec for CONN_PARAM_NEXT_SUP_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_param_next_sup_to::R](R) reader structure"]
impl crate::Readable for CONN_PARAM_NEXT_SUP_TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_param_next_sup_to::W](W) writer structure"]
impl crate::Writable for CONN_PARAM_NEXT_SUP_TO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_PARAM_NEXT_SUP_TO to value 0"]
impl crate::Resettable for CONN_PARAM_NEXT_SUP_TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
