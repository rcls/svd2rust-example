#[doc = "Register `PBOOST` reader"]
pub struct R(crate::R<PBOOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBOOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBOOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBOOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBOOST` writer"]
pub struct W(crate::W<PBOOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBOOST_SPEC>;
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
impl From<crate::W<PBOOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBOOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Boost` reader - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
pub struct BOOST_R(crate::FieldReader<u8, u8>);
impl BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Boost` writer - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
pub struct BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W {
        BOOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power boost register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pboost](index.html) module"]
pub struct PBOOST_SPEC;
impl crate::RegisterSpec for PBOOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pboost::R](R) reader structure"]
impl crate::Readable for PBOOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pboost::W](W) writer structure"]
impl crate::Writable for PBOOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBOOST to value 0"]
impl crate::Resettable for PBOOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
