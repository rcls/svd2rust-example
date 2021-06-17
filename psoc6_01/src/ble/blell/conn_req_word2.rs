#[doc = "Register `CONN_REQ_WORD2` reader"]
pub struct R(crate::R<CONN_REQ_WORD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD2` writer"]
pub struct W(crate::W<CONN_REQ_WORD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD2_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WINDOW_SIZE_VAL` reader - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub struct TX_WINDOW_SIZE_VAL_R(crate::FieldReader<u8, u8>);
impl TX_WINDOW_SIZE_VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_WINDOW_SIZE_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WINDOW_SIZE_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WINDOW_SIZE_VAL` writer - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub struct TX_WINDOW_SIZE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WINDOW_SIZE_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CRC_INIT_LOWER` reader - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
pub struct CRC_INIT_LOWER_R(crate::FieldReader<u8, u8>);
impl CRC_INIT_LOWER_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_INIT_LOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_INIT_LOWER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_INIT_LOWER` writer - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
pub struct CRC_INIT_LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&self) -> TX_WINDOW_SIZE_VAL_R {
        TX_WINDOW_SIZE_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&self) -> CRC_INIT_LOWER_R {
        CRC_INIT_LOWER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&mut self) -> TX_WINDOW_SIZE_VAL_W {
        TX_WINDOW_SIZE_VAL_W { w: self }
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&mut self) -> CRC_INIT_LOWER_W {
        CRC_INIT_LOWER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word2](index.html) module"]
pub struct CONN_REQ_WORD2_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word2::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word2::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD2 to value 0"]
impl crate::Resettable for CONN_REQ_WORD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
