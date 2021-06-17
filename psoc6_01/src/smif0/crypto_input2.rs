#[doc = "Register `CRYPTO_INPUT2` reader"]
pub struct R(crate::R<CRYPTO_INPUT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_INPUT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_INPUT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_INPUT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_INPUT2` writer"]
pub struct W(crate::W<CRYPTO_INPUT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_INPUT2_SPEC>;
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
impl From<crate::W<CRYPTO_INPUT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_INPUT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT` reader - Four Bytes of the plaintext PT\\[95:64\\]
= CRYPTO_INPUT2.INPUT\\[31:0\\]."]
pub struct INPUT_R(crate::FieldReader<u32, u32>);
impl INPUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        INPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT` writer - Four Bytes of the plaintext PT\\[95:64\\]
= CRYPTO_INPUT2.INPUT\\[31:0\\]."]
pub struct INPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[95:64\\]
= CRYPTO_INPUT2.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[95:64\\]
= CRYPTO_INPUT2.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W {
        INPUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography input 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input2](index.html) module"]
pub struct CRYPTO_INPUT2_SPEC;
impl crate::RegisterSpec for CRYPTO_INPUT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_input2::R](R) reader structure"]
impl crate::Readable for CRYPTO_INPUT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_input2::W](W) writer structure"]
impl crate::Writable for CRYPTO_INPUT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_INPUT2 to value 0"]
impl crate::Resettable for CRYPTO_INPUT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
