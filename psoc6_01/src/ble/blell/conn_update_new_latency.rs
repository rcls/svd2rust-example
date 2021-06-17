#[doc = "Register `CONN_UPDATE_NEW_LATENCY` reader"]
pub struct R(crate::R<CONN_UPDATE_NEW_LATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_UPDATE_NEW_LATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_UPDATE_NEW_LATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_UPDATE_NEW_LATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_UPDATE_NEW_LATENCY` writer"]
pub struct W(crate::W<CONN_UPDATE_NEW_LATENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_UPDATE_NEW_LATENCY_SPEC>;
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
impl From<crate::W<CONN_UPDATE_NEW_LATENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_UPDATE_NEW_LATENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_UPDT_SLV_LATENCY` reader - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
pub struct CONN_UPDT_SLV_LATENCY_R(crate::FieldReader<u16, u16>);
impl CONN_UPDT_SLV_LATENCY_R {
    pub(crate) fn new(bits: u16) -> Self {
        CONN_UPDT_SLV_LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONN_UPDT_SLV_LATENCY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONN_UPDT_SLV_LATENCY` writer - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
pub struct CONN_UPDT_SLV_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_UPDT_SLV_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_slv_latency(&self) -> CONN_UPDT_SLV_LATENCY_R {
        CONN_UPDT_SLV_LATENCY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_slv_latency(&mut self) -> CONN_UPDT_SLV_LATENCY_W {
        CONN_UPDT_SLV_LATENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection update new latency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_latency](index.html) module"]
pub struct CONN_UPDATE_NEW_LATENCY_SPEC;
impl crate::RegisterSpec for CONN_UPDATE_NEW_LATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_update_new_latency::R](R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_LATENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_update_new_latency::W](W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_LATENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_UPDATE_NEW_LATENCY to value 0"]
impl crate::Resettable for CONN_UPDATE_NEW_LATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
