#[doc = "Register `INXCMP1` reader"]
pub struct R(crate::R<INXCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INXCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INXCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INXCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INXCMP1` writer"]
pub struct W(crate::W<INXCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INXCMP1_SPEC>;
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
impl From<crate::W<INXCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INXCMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICMP1` reader - Index compare value 1."]
pub struct ICMP1_R(crate::FieldReader<u32, u32>);
impl ICMP1_R {
    pub(crate) fn new(bits: u32) -> Self {
        ICMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICMP1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICMP1` writer - Index compare value 1."]
pub struct ICMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ICMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&self) -> ICMP1_R {
        ICMP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&mut self) -> ICMP1_W {
        ICMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inxcmp1](index.html) module"]
pub struct INXCMP1_SPEC;
impl crate::RegisterSpec for INXCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inxcmp1::R](R) reader structure"]
impl crate::Readable for INXCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inxcmp1::W](W) writer structure"]
impl crate::Writable for INXCMP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INXCMP1 to value 0xffff_ffff"]
impl crate::Resettable for INXCMP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
