#[doc = "Register `PLLCFG%s` reader"]
pub struct R(crate::R<PLLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFG%s` writer"]
pub struct W(crate::W<PLLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFG_SPEC>;
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
impl From<crate::W<PLLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
pub struct MSEL_R(crate::FieldReader<u8, u8>);
impl MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEL` writer - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PSEL` reader - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
pub struct PSEL_R(crate::FieldReader<u8, u8>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfg](index.html) module"]
pub struct PLLCFG_SPEC;
impl crate::RegisterSpec for PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfg::R](R) reader structure"]
impl crate::Readable for PLLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfg::W](W) writer structure"]
impl crate::Writable for PLLCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFG%s to value 0"]
impl crate::Resettable for PLLCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
