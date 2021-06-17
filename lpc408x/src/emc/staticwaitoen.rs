#[doc = "Register `STATICWAITOEN%s` reader"]
pub struct R(crate::R<STATICWAITOEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITOEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITOEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITOEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITOEN%s` writer"]
pub struct W(crate::W<STATICWAITOEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITOEN_SPEC>;
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
impl From<crate::W<STATICWAITOEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITOEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITOEN` reader - Wait output enable. Delay from chip select assertion to output enable. 0x0 = No delay (POR reset value). 0x1 - 0xF = n cycle delay. The delay is WAITOEN x tCCLK."]
pub struct WAITOEN_R(crate::FieldReader<u8, u8>);
impl WAITOEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITOEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITOEN` writer - Wait output enable. Delay from chip select assertion to output enable. 0x0 = No delay (POR reset value). 0x1 - 0xF = n cycle delay. The delay is WAITOEN x tCCLK."]
pub struct WAITOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITOEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait output enable. Delay from chip select assertion to output enable. 0x0 = No delay (POR reset value). 0x1 - 0xF = n cycle delay. The delay is WAITOEN x tCCLK."]
    #[inline(always)]
    pub fn waitoen(&self) -> WAITOEN_R {
        WAITOEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait output enable. Delay from chip select assertion to output enable. 0x0 = No delay (POR reset value). 0x1 - 0xF = n cycle delay. The delay is WAITOEN x tCCLK."]
    #[inline(always)]
    pub fn waitoen(&mut self) -> WAITOEN_W {
        WAITOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay from EMC_CS0 or address change, whichever is later, to output enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitoen](index.html) module"]
pub struct STATICWAITOEN_SPEC;
impl crate::RegisterSpec for STATICWAITOEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitoen::R](R) reader structure"]
impl crate::Readable for STATICWAITOEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitoen::W](W) writer structure"]
impl crate::Writable for STATICWAITOEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITOEN%s to value 0"]
impl crate::Resettable for STATICWAITOEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
