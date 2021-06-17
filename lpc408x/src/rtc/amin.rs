#[doc = "Register `AMIN` reader"]
pub struct R(crate::R<AMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMIN` writer"]
pub struct W(crate::W<AMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMIN_SPEC>;
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
impl From<crate::W<AMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub struct MINUTES_R(crate::FieldReader<u8, u8>);
impl MINUTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINUTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINUTES` writer - Minutes value in the range of 0 to 59"]
pub struct MINUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm value for Minutes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amin](index.html) module"]
pub struct AMIN_SPEC;
impl crate::RegisterSpec for AMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amin::R](R) reader structure"]
impl crate::Readable for AMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amin::W](W) writer structure"]
impl crate::Writable for AMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMIN to value 0"]
impl crate::Resettable for AMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
