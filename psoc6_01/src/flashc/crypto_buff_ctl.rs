#[doc = "Register `CRYPTO_BUFF_CTL` reader"]
pub struct R(crate::R<CRYPTO_BUFF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_BUFF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_BUFF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_BUFF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_BUFF_CTL` writer"]
pub struct W(crate::W<CRYPTO_BUFF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_BUFF_CTL_SPEC>;
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
impl From<crate::W<CRYPTO_BUFF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_BUFF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
pub struct PREF_EN_R(crate::FieldReader<bool, bool>);
impl PREF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - Cache enable: 0: Disabled. 1: Enabled."]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - Cache enable: 0: Disabled. 1: Enabled."]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the buffer to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_buff_ctl](index.html) module"]
pub struct CRYPTO_BUFF_CTL_SPEC;
impl crate::RegisterSpec for CRYPTO_BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_buff_ctl::R](R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_buff_ctl::W](W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_BUFF_CTL to value 0xc000_0000"]
impl crate::Resettable for CRYPTO_BUFF_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
