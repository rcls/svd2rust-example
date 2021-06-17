#[doc = "Register `TXDESCRIPTORNUMBER` reader"]
pub struct R(crate::R<TXDESCRIPTORNUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDESCRIPTORNUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDESCRIPTORNUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDESCRIPTORNUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDESCRIPTORNUMBER` writer"]
pub struct W(crate::W<TXDESCRIPTORNUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDESCRIPTORNUMBER_SPEC>;
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
impl From<crate::W<TXDESCRIPTORNUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDESCRIPTORNUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDN` reader - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub struct TXDN_R(crate::FieldReader<u16, u16>);
impl TXDN_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDN` writer - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub struct TXDN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&self) -> TXDN_R {
        TXDN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&mut self) -> TXDN_W {
        TXDN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit number of descriptors register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdescriptornumber](index.html) module"]
pub struct TXDESCRIPTORNUMBER_SPEC;
impl crate::RegisterSpec for TXDESCRIPTORNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdescriptornumber::R](R) reader structure"]
impl crate::Readable for TXDESCRIPTORNUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdescriptornumber::W](W) writer structure"]
impl crate::Writable for TXDESCRIPTORNUMBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for TXDESCRIPTORNUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
