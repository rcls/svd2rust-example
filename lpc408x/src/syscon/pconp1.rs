#[doc = "Register `PCONP1` reader"]
pub struct R(crate::R<PCONP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONP1` writer"]
pub struct W(crate::W<PCONP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONP1_SPEC>;
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
impl From<crate::W<PCONP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCCMP` reader - comparator 0/1 power/clock control bit."]
pub struct PCCMP_R(crate::FieldReader<bool, bool>);
impl PCCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCCMP` writer - comparator 0/1 power/clock control bit."]
pub struct PCCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - comparator 0/1 power/clock control bit."]
    #[inline(always)]
    pub fn pccmp(&self) -> PCCMP_R {
        PCCMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - comparator 0/1 power/clock control bit."]
    #[inline(always)]
    pub fn pccmp(&mut self) -> PCCMP_W {
        PCCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control for Peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconp1](index.html) module"]
pub struct PCONP1_SPEC;
impl crate::RegisterSpec for PCONP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pconp1::R](R) reader structure"]
impl crate::Readable for PCONP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pconp1::W](W) writer structure"]
impl crate::Writable for PCONP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONP1 to value 0"]
impl crate::Resettable for PCONP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
