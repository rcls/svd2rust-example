#[doc = "Register `LLI%s` reader"]
pub struct R(crate::R<LLI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLI%s` writer"]
pub struct W(crate::W<LLI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLI_SPEC>;
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
impl From<crate::W<LLI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLI` reader - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub struct LLI_R(crate::FieldReader<u32, u32>);
impl LLI_R {
    pub(crate) fn new(bits: u32) -> Self {
        LLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLI` writer - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub struct LLI_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    pub fn lli(&self) -> LLI_R {
        LLI_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    pub fn lli(&mut self) -> LLI_W {
        LLI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 0 Linked List Item Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lli](index.html) module"]
pub struct LLI_SPEC;
impl crate::RegisterSpec for LLI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lli::R](R) reader structure"]
impl crate::Readable for LLI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lli::W](W) writer structure"]
impl crate::Writable for LLI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LLI%s to value 0"]
impl crate::Resettable for LLI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
