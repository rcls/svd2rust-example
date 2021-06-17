#[doc = "Register `ENC_CONFIG` reader"]
pub struct R(crate::R<ENC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_CONFIG` writer"]
pub struct W(crate::W<ENC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_CONFIG_SPEC>;
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
impl From<crate::W<ENC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_PROC` reader - 1 Start the AES processing"]
pub struct START_PROC_R(crate::FieldReader<bool, bool>);
impl START_PROC_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_PROC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_PROC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_PROC` writer - 1 Start the AES processing"]
pub struct START_PROC_W<'a> {
    w: &'a mut W,
}
impl<'a> START_PROC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ECB_CCM` reader - 0 - CCM 1 - ECB"]
pub struct ECB_CCM_R(crate::FieldReader<bool, bool>);
impl ECB_CCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECB_CCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECB_CCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECB_CCM` writer - 0 - CCM 1 - ECB"]
pub struct ECB_CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ECB_CCM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DEC_ENC` reader - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
pub struct DEC_ENC_R(crate::FieldReader<bool, bool>);
impl DEC_ENC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEC_ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEC_ENC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEC_ENC` writer - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
pub struct DEC_ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_ENC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PAYLOAD_LENGTH_MSB` reader - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
pub struct PAYLOAD_LENGTH_MSB_R(crate::FieldReader<u8, u8>);
impl PAYLOAD_LENGTH_MSB_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAYLOAD_LENGTH_MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAYLOAD_LENGTH_MSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAYLOAD_LENGTH_MSB` writer - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
pub struct PAYLOAD_LENGTH_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `B0_FLAGS` reader - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
pub struct B0_FLAGS_R(crate::FieldReader<u8, u8>);
impl B0_FLAGS_R {
    pub(crate) fn new(bits: u8) -> Self {
        B0_FLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_FLAGS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_FLAGS` writer - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
pub struct B0_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `AES_B0_DATA_OVERRIDE` reader - Configuration to use B0 DATA provided by FW for CCM computation"]
pub struct AES_B0_DATA_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl AES_B0_DATA_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES_B0_DATA_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_B0_DATA_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_B0_DATA_OVERRIDE` writer - Configuration to use B0 DATA provided by FW for CCM computation"]
pub struct AES_B0_DATA_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_B0_DATA_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&self) -> START_PROC_R {
        START_PROC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&self) -> ECB_CCM_R {
        ECB_CCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&self) -> DEC_ENC_R {
        DEC_ENC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
    #[inline(always)]
    pub fn payload_length_msb(&self) -> PAYLOAD_LENGTH_MSB_R {
        PAYLOAD_LENGTH_MSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
    #[inline(always)]
    pub fn b0_flags(&self) -> B0_FLAGS_R {
        B0_FLAGS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Configuration to use B0 DATA provided by FW for CCM computation"]
    #[inline(always)]
    pub fn aes_b0_data_override(&self) -> AES_B0_DATA_OVERRIDE_R {
        AES_B0_DATA_OVERRIDE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&mut self) -> START_PROC_W {
        START_PROC_W { w: self }
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&mut self) -> ECB_CCM_W {
        ECB_CCM_W { w: self }
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&mut self) -> DEC_ENC_W {
        DEC_ENC_W { w: self }
    }
    #[doc = "Bits 8:15 - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
    #[inline(always)]
    pub fn payload_length_msb(&mut self) -> PAYLOAD_LENGTH_MSB_W {
        PAYLOAD_LENGTH_MSB_W { w: self }
    }
    #[doc = "Bits 16:23 - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
    #[inline(always)]
    pub fn b0_flags(&mut self) -> B0_FLAGS_W {
        B0_FLAGS_W { w: self }
    }
    #[doc = "Bit 24 - Configuration to use B0 DATA provided by FW for CCM computation"]
    #[inline(always)]
    pub fn aes_b0_data_override(&mut self) -> AES_B0_DATA_OVERRIDE_W {
        AES_B0_DATA_OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_config](index.html) module"]
pub struct ENC_CONFIG_SPEC;
impl crate::RegisterSpec for ENC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_config::R](R) reader structure"]
impl crate::Readable for ENC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_config::W](W) writer structure"]
impl crate::Writable for ENC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_CONFIG to value 0"]
impl crate::Resettable for ENC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
