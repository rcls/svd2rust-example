#[doc = "Register `HASHFILTERL` reader"]
pub struct R(crate::R<HASHFILTERL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHFILTERL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHFILTERL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHFILTERL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHFILTERL` writer"]
pub struct W(crate::W<HASHFILTERL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHFILTERL_SPEC>;
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
impl From<crate::W<HASHFILTERL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHFILTERL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFL` reader - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub struct HFL_R(crate::FieldReader<u32, u32>);
impl HFL_R {
    pub(crate) fn new(bits: u32) -> Self {
        HFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFL` writer - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub struct HFL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&self) -> HFL_R {
        HFL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&mut self) -> HFL_W {
        HFL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash filter table LSBs register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashfilterl](index.html) module"]
pub struct HASHFILTERL_SPEC;
impl crate::RegisterSpec for HASHFILTERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashfilterl::R](R) reader structure"]
impl crate::Readable for HASHFILTERL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashfilterl::W](W) writer structure"]
impl crate::Writable for HASHFILTERL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHFILTERL to value 0"]
impl crate::Resettable for HASHFILTERL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
