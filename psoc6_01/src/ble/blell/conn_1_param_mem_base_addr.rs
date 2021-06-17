#[doc = "Register `CONN_1_PARAM_MEM_BASE_ADDR` reader"]
pub struct R(crate::R<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_1_PARAM_MEM_BASE_ADDR` writer"]
pub struct W(crate::W<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>;
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
impl From<crate::W<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_1_PARAM` reader - N/A"]
pub struct CONN_1_PARAM_R(crate::FieldReader<u16, u16>);
impl CONN_1_PARAM_R {
    pub(crate) fn new(bits: u16) -> Self {
        CONN_1_PARAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_1_PARAM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_1_PARAM` writer - N/A"]
pub struct CONN_1_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_1_PARAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_1_param(&self) -> CONN_1_PARAM_R {
        CONN_1_PARAM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_1_param(&mut self) -> CONN_1_PARAM_W {
        CONN_1_PARAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection Parameter memory base address for connection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_1_param_mem_base_addr](index.html) module"]
pub struct CONN_1_PARAM_MEM_BASE_ADDR_SPEC;
impl crate::RegisterSpec for CONN_1_PARAM_MEM_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_1_param_mem_base_addr::R](R) reader structure"]
impl crate::Readable for CONN_1_PARAM_MEM_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_1_param_mem_base_addr::W](W) writer structure"]
impl crate::Writable for CONN_1_PARAM_MEM_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_1_PARAM_MEM_BASE_ADDR to value 0"]
impl crate::Resettable for CONN_1_PARAM_MEM_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
