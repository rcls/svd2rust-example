#[doc = "Register `FCANIC0` reader"]
pub struct R(crate::R<FCANIC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCANIC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCANIC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCANIC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCANIC0` writer"]
pub struct W(crate::W<FCANIC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCANIC0_SPEC>;
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
impl From<crate::W<FCANIC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCANIC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTPND` reader - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
pub struct INTPND_R(crate::FieldReader<u32, u32>);
impl INTPND_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTPND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTPND` writer - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
pub struct INTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    pub fn intpnd(&mut self) -> INTPND_W {
        INTPND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FullCAN interrupt and capture register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcanic0](index.html) module"]
pub struct FCANIC0_SPEC;
impl crate::RegisterSpec for FCANIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcanic0::R](R) reader structure"]
impl crate::Readable for FCANIC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcanic0::W](W) writer structure"]
impl crate::Writable for FCANIC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCANIC0 to value 0"]
impl crate::Resettable for FCANIC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
