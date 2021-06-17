#[doc = "Register `CRYPTO_KEY0` writer"]
pub struct W(crate::W<CRYPTO_KEY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_KEY0_SPEC>;
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
impl From<crate::W<CRYPTO_KEY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_KEY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Four Bytes of the key KEY\\[31:0\\]
= CRYPTO_KEY0.KEY\\[31:0\\]."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[31:0\\]
= CRYPTO_KEY0.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography key 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_key0](index.html) module"]
pub struct CRYPTO_KEY0_SPEC;
impl crate::RegisterSpec for CRYPTO_KEY0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [crypto_key0::W](W) writer structure"]
impl crate::Writable for CRYPTO_KEY0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_KEY0 to value 0"]
impl crate::Resettable for CRYPTO_KEY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
