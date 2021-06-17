#[doc = "Register `PLLCON%s` reader"]
pub struct R(crate::R<PLLCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCON%s` writer"]
pub struct W(crate::W<PLLCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCON_SPEC>;
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
impl From<crate::W<PLLCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLE` reader - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
pub struct PLLE_R(crate::FieldReader<bool, bool>);
impl PLLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLE` writer - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
pub struct PLLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&mut self) -> PLLE_W {
        PLLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon](index.html) module"]
pub struct PLLCON_SPEC;
impl crate::RegisterSpec for PLLCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcon::R](R) reader structure"]
impl crate::Readable for PLLCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcon::W](W) writer structure"]
impl crate::Writable for PLLCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCON%s to value 0"]
impl crate::Resettable for PLLCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
