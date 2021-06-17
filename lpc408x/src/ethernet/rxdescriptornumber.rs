#[doc = "Register `RXDESCRIPTORNUMBER` reader"]
pub struct R(crate::R<RXDESCRIPTORNUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDESCRIPTORNUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDESCRIPTORNUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDESCRIPTORNUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDESCRIPTORNUMBER` writer"]
pub struct W(crate::W<RXDESCRIPTORNUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDESCRIPTORNUMBER_SPEC>;
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
impl From<crate::W<RXDESCRIPTORNUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDESCRIPTORNUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDESCRIPTORN` reader - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub struct RXDESCRIPTORN_R(crate::FieldReader<u16, u16>);
impl RXDESCRIPTORN_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXDESCRIPTORN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDESCRIPTORN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDESCRIPTORN` writer - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub struct RXDESCRIPTORN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDESCRIPTORN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&self) -> RXDESCRIPTORN_R {
        RXDESCRIPTORN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&mut self) -> RXDESCRIPTORN_W {
        RXDESCRIPTORN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive number of descriptors register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdescriptornumber](index.html) module"]
pub struct RXDESCRIPTORNUMBER_SPEC;
impl crate::RegisterSpec for RXDESCRIPTORNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdescriptornumber::R](R) reader structure"]
impl crate::Readable for RXDESCRIPTORNUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdescriptornumber::W](W) writer structure"]
impl crate::Writable for RXDESCRIPTORNUMBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for RXDESCRIPTORNUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
