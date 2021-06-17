#[doc = "Register `CRYPTO_OUTPUT0` reader"]
pub struct R(crate::R<CRYPTO_OUTPUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_OUTPUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_OUTPUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_OUTPUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_OUTPUT0` writer"]
pub struct W(crate::W<CRYPTO_OUTPUT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_OUTPUT0_SPEC>;
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
impl From<crate::W<CRYPTO_OUTPUT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_OUTPUT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT` reader - Four Bytes of the ciphertext CT\\[31:0\\]
= CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
pub struct OUTPUT_R(crate::FieldReader<u32, u32>);
impl OUTPUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        OUTPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUT` writer - Four Bytes of the ciphertext CT\\[31:0\\]
= CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
pub struct OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[31:0\\]
= CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[31:0\\]
= CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&mut self) -> OUTPUT_W {
        OUTPUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography output 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_output0](index.html) module"]
pub struct CRYPTO_OUTPUT0_SPEC;
impl crate::RegisterSpec for CRYPTO_OUTPUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_output0::R](R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_output0::W](W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_OUTPUT0 to value 0"]
impl crate::Resettable for CRYPTO_OUTPUT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
