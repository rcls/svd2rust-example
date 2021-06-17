#[doc = "Register `CONN_REQ_WORD11` reader"]
pub struct R(crate::R<CONN_REQ_WORD11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD11` writer"]
pub struct W(crate::W<CONN_REQ_WORD11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD11_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOP_INCREMENT_2` reader - This field is used for the data channel selection process."]
pub struct HOP_INCREMENT_2_R(crate::FieldReader<u8, u8>);
impl HOP_INCREMENT_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOP_INCREMENT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOP_INCREMENT_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOP_INCREMENT_2` writer - This field is used for the data channel selection process."]
pub struct HOP_INCREMENT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOP_INCREMENT_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `SCA_2` reader - This field defines the sleep clock accuracies given in ppm."]
pub struct SCA_2_R(crate::FieldReader<u8, u8>);
impl SCA_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCA_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCA_2` writer - This field defines the sleep clock accuracies given in ppm."]
pub struct SCA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCA_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This field is used for the data channel selection process."]
    #[inline(always)]
    pub fn hop_increment_2(&self) -> HOP_INCREMENT_2_R {
        HOP_INCREMENT_2_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field defines the sleep clock accuracies given in ppm."]
    #[inline(always)]
    pub fn sca_2(&self) -> SCA_2_R {
        SCA_2_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field is used for the data channel selection process."]
    #[inline(always)]
    pub fn hop_increment_2(&mut self) -> HOP_INCREMENT_2_W {
        HOP_INCREMENT_2_W { w: self }
    }
    #[doc = "Bits 5:7 - This field defines the sleep clock accuracies given in ppm."]
    #[inline(always)]
    pub fn sca_2(&mut self) -> SCA_2_W {
        SCA_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word11](index.html) module"]
pub struct CONN_REQ_WORD11_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word11::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word11::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD11 to value 0"]
impl crate::Resettable for CONN_REQ_WORD11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
