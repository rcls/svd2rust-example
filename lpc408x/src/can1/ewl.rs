#[doc = "Register `EWL` reader"]
pub struct R(crate::R<EWL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EWL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EWL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EWL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EWL` writer"]
pub struct W(crate::W<EWL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EWL_SPEC>;
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
impl From<crate::W<EWL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EWL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWL` reader - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub struct EWL_R(crate::FieldReader<u8, u8>);
impl EWL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWL` writer - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub struct EWL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W {
        EWL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ewl](index.html) module"]
pub struct EWL_SPEC;
impl crate::RegisterSpec for EWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ewl::R](R) reader structure"]
impl crate::Readable for EWL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ewl::W](W) writer structure"]
impl crate::Writable for EWL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EWL to value 0x60"]
impl crate::Resettable for EWL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
