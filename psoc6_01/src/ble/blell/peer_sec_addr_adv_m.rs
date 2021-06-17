#[doc = "Register `PEER_SEC_ADDR_ADV_M` reader"]
pub struct R(crate::R<PEER_SEC_ADDR_ADV_M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEER_SEC_ADDR_ADV_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEER_SEC_ADDR_ADV_M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEER_SEC_ADDR_ADV_M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEER_SEC_ADDR_ADV_M` writer"]
pub struct W(crate::W<PEER_SEC_ADDR_ADV_M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEER_SEC_ADDR_ADV_M_SPEC>;
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
impl From<crate::W<PEER_SEC_ADDR_ADV_M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEER_SEC_ADDR_ADV_M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEER_SEC_ADDR_M` reader - Middle 16 bit of 48-bit secondary address of the peer device for ADV_DIR."]
pub struct PEER_SEC_ADDR_M_R(crate::FieldReader<u16, u16>);
impl PEER_SEC_ADDR_M_R {
    pub(crate) fn new(bits: u16) -> Self {
        PEER_SEC_ADDR_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEER_SEC_ADDR_M_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEER_SEC_ADDR_M` writer - Middle 16 bit of 48-bit secondary address of the peer device for ADV_DIR."]
pub struct PEER_SEC_ADDR_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_SEC_ADDR_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit secondary address of the peer device for ADV_DIR."]
    #[inline(always)]
    pub fn peer_sec_addr_m(&self) -> PEER_SEC_ADDR_M_R {
        PEER_SEC_ADDR_M_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit secondary address of the peer device for ADV_DIR."]
    #[inline(always)]
    pub fn peer_sec_addr_m(&mut self) -> PEER_SEC_ADDR_M_W {
        PEER_SEC_ADDR_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Middle 16 bits of the secondary address of the peer device for ADV_DIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_sec_addr_adv_m](index.html) module"]
pub struct PEER_SEC_ADDR_ADV_M_SPEC;
impl crate::RegisterSpec for PEER_SEC_ADDR_ADV_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peer_sec_addr_adv_m::R](R) reader structure"]
impl crate::Readable for PEER_SEC_ADDR_ADV_M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peer_sec_addr_adv_m::W](W) writer structure"]
impl crate::Writable for PEER_SEC_ADDR_ADV_M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEER_SEC_ADDR_ADV_M to value 0"]
impl crate::Resettable for PEER_SEC_ADDR_ADV_M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
