#[doc = "Register `CONN_REQ_WORD3` reader"]
pub struct R(crate::R<CONN_REQ_WORD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD3` writer"]
pub struct W(crate::W<CONN_REQ_WORD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD3_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_INIT_UPPER` reader - This field defines the upper byte \\[23:8\\]
of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
pub struct CRC_INIT_UPPER_R(crate::FieldReader<u16, u16>);
impl CRC_INIT_UPPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRC_INIT_UPPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_INIT_UPPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_INIT_UPPER` writer - This field defines the upper byte \\[23:8\\]
of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
pub struct CRC_INIT_UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the upper byte \\[23:8\\]
of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn crc_init_upper(&self) -> CRC_INIT_UPPER_R {
        CRC_INIT_UPPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper byte \\[23:8\\]
of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn crc_init_upper(&mut self) -> CRC_INIT_UPPER_W {
        CRC_INIT_UPPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word3](index.html) module"]
pub struct CONN_REQ_WORD3_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word3::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word3::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD3 to value 0"]
impl crate::Resettable for CONN_REQ_WORD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
