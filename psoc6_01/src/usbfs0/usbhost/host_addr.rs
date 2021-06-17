#[doc = "Register `HOST_ADDR` reader"]
pub struct R(crate::R<HOST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_ADDR` writer"]
pub struct W(crate::W<HOST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_ADDR_SPEC>;
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
impl From<crate::W<HOST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct ADDRESS_R(crate::FieldReader<u8, u8>);
impl ADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits are used to specify a token address. Note: - This bit is reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_addr](index.html) module"]
pub struct HOST_ADDR_SPEC;
impl crate::RegisterSpec for HOST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_addr::R](R) reader structure"]
impl crate::Readable for HOST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_addr::W](W) writer structure"]
impl crate::Writable for HOST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_ADDR to value 0"]
impl crate::Resettable for HOST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
