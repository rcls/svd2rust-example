#[doc = "Register `RSLV_LIST_PEER_RPA_BASE_ADDR` reader"]
pub struct R(crate::R<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSLV_LIST_PEER_RPA_BASE_ADDR` writer"]
pub struct W(crate::W<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>;
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
impl From<crate::W<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSLV_LIST_PEER_RPA_BASE_ADDR` reader - Device address values written to the list are written as 16-bit wide address."]
pub struct RSLV_LIST_PEER_RPA_BASE_ADDR_R(crate::FieldReader<u16, u16>);
impl RSLV_LIST_PEER_RPA_BASE_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        RSLV_LIST_PEER_RPA_BASE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSLV_LIST_PEER_RPA_BASE_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSLV_LIST_PEER_RPA_BASE_ADDR` writer - Device address values written to the list are written as 16-bit wide address."]
pub struct RSLV_LIST_PEER_RPA_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSLV_LIST_PEER_RPA_BASE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device address values written to the list are written as 16-bit wide address."]
    #[inline(always)]
    pub fn rslv_list_peer_rpa_base_addr(&self) -> RSLV_LIST_PEER_RPA_BASE_ADDR_R {
        RSLV_LIST_PEER_RPA_BASE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address values written to the list are written as 16-bit wide address."]
    #[inline(always)]
    pub fn rslv_list_peer_rpa_base_addr(&mut self) -> RSLV_LIST_PEER_RPA_BASE_ADDR_W {
        RSLV_LIST_PEER_RPA_BASE_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resolving list base address for storing resolved Peer RPA address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_peer_rpa_base_addr](index.html) module"]
pub struct RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC;
impl crate::RegisterSpec for RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rslv_list_peer_rpa_base_addr::R](R) reader structure"]
impl crate::Readable for RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rslv_list_peer_rpa_base_addr::W](W) writer structure"]
impl crate::Writable for RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSLV_LIST_PEER_RPA_BASE_ADDR to value 0"]
impl crate::Resettable for RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
