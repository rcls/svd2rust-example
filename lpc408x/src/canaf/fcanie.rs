#[doc = "Register `FCANIE` reader"]
pub struct R(crate::R<FCANIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCANIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCANIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCANIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCANIE` writer"]
pub struct W(crate::W<FCANIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCANIE_SPEC>;
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
impl From<crate::W<FCANIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCANIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCANIE` reader - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub struct FCANIE_R(crate::FieldReader<bool, bool>);
impl FCANIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCANIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCANIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCANIE` writer - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub struct FCANIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCANIE_W<'a> {
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
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    pub fn fcanie(&self) -> FCANIE_R {
        FCANIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    pub fn fcanie(&mut self) -> FCANIE_W {
        FCANIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FullCAN interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcanie](index.html) module"]
pub struct FCANIE_SPEC;
impl crate::RegisterSpec for FCANIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcanie::R](R) reader structure"]
impl crate::Readable for FCANIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcanie::W](W) writer structure"]
impl crate::Writable for FCANIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCANIE to value 0"]
impl crate::Resettable for FCANIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
