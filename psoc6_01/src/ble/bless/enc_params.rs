#[doc = "Register `ENC_PARAMS` reader"]
pub struct R(crate::R<ENC_PARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_PARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_PARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_PARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_PARAMS` writer"]
pub struct W(crate::W<ENC_PARAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_PARAMS_SPEC>;
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
impl From<crate::W<ENC_PARAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_PARAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_PDU_HEADER` reader - LLID of the packet."]
pub struct DATA_PDU_HEADER_R(crate::FieldReader<u8, u8>);
impl DATA_PDU_HEADER_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_PDU_HEADER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_PDU_HEADER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_PDU_HEADER` writer - LLID of the packet."]
pub struct DATA_PDU_HEADER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PDU_HEADER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PAYLOAD_LENGTH_LSB` reader - Length of the input data."]
pub struct PAYLOAD_LENGTH_LSB_R(crate::FieldReader<u8, u8>);
impl PAYLOAD_LENGTH_LSB_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAYLOAD_LENGTH_LSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAYLOAD_LENGTH_LSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAYLOAD_LENGTH_LSB` writer - Length of the input data."]
pub struct PAYLOAD_LENGTH_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_LSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `DIRECTION` reader - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
pub struct DIRECTION_R(crate::FieldReader<bool, bool>);
impl DIRECTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECTION` writer - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PAYLOAD_LENGTH_LSB_EXT` reader - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
pub struct PAYLOAD_LENGTH_LSB_EXT_R(crate::FieldReader<u8, u8>);
impl PAYLOAD_LENGTH_LSB_EXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAYLOAD_LENGTH_LSB_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAYLOAD_LENGTH_LSB_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAYLOAD_LENGTH_LSB_EXT` writer - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
pub struct PAYLOAD_LENGTH_LSB_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_LSB_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `MEM_LATENCY_HIDE` reader - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
pub struct MEM_LATENCY_HIDE_R(crate::FieldReader<bool, bool>);
impl MEM_LATENCY_HIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_LATENCY_HIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_LATENCY_HIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_LATENCY_HIDE` writer - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
pub struct MEM_LATENCY_HIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_LATENCY_HIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&self) -> DATA_PDU_HEADER_R {
        DATA_PDU_HEADER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&self) -> PAYLOAD_LENGTH_LSB_R {
        PAYLOAD_LENGTH_LSB_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&self) -> PAYLOAD_LENGTH_LSB_EXT_R {
        PAYLOAD_LENGTH_LSB_EXT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&self) -> MEM_LATENCY_HIDE_R {
        MEM_LATENCY_HIDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&mut self) -> DATA_PDU_HEADER_W {
        DATA_PDU_HEADER_W { w: self }
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&mut self) -> PAYLOAD_LENGTH_LSB_W {
        PAYLOAD_LENGTH_LSB_W { w: self }
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&mut self) -> PAYLOAD_LENGTH_LSB_EXT_W {
        PAYLOAD_LENGTH_LSB_EXT_W { w: self }
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&mut self) -> MEM_LATENCY_HIDE_W {
        MEM_LATENCY_HIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Parameter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_params](index.html) module"]
pub struct ENC_PARAMS_SPEC;
impl crate::RegisterSpec for ENC_PARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_params::R](R) reader structure"]
impl crate::Readable for ENC_PARAMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_params::W](W) writer structure"]
impl crate::Writable for ENC_PARAMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_PARAMS to value 0"]
impl crate::Resettable for ENC_PARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
