#[doc = "Register `HASHFILTERH` reader"]
pub struct R(crate::R<HASHFILTERH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHFILTERH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHFILTERH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHFILTERH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHFILTERH` writer"]
pub struct W(crate::W<HASHFILTERH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHFILTERH_SPEC>;
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
impl From<crate::W<HASHFILTERH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHFILTERH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFH` reader - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub struct HFH_R(crate::FieldReader<u32, u32>);
impl HFH_R {
    pub(crate) fn new(bits: u32) -> Self {
        HFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFH` writer - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub struct HFH_W<'a> {
    w: &'a mut W,
}
impl<'a> HFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfh(&self) -> HFH_R {
        HFH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfh(&mut self) -> HFH_W {
        HFH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash filter table MSBs register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashfilterh](index.html) module"]
pub struct HASHFILTERH_SPEC;
impl crate::RegisterSpec for HASHFILTERH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashfilterh::R](R) reader structure"]
impl crate::Readable for HASHFILTERH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashfilterh::W](W) writer structure"]
impl crate::Writable for HASHFILTERH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHFILTERH to value 0"]
impl crate::Resettable for HASHFILTERH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
