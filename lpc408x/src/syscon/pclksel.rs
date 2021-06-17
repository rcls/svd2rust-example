#[doc = "Register `PCLKSEL` reader"]
pub struct R(crate::R<PCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLKSEL` writer"]
pub struct W(crate::W<PCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLKSEL_SPEC>;
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
impl From<crate::W<PCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLKDIV` reader - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
pub struct PCLKDIV_R(crate::FieldReader<u8, u8>);
impl PCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLKDIV` writer - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
pub struct PCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline(always)]
    pub fn pclkdiv(&self) -> PCLKDIV_R {
        PCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline(always)]
    pub fn pclkdiv(&mut self) -> PCLKDIV_W {
        PCLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksel](index.html) module"]
pub struct PCLKSEL_SPEC;
impl crate::RegisterSpec for PCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclksel::R](R) reader structure"]
impl crate::Readable for PCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclksel::W](W) writer structure"]
impl crate::Writable for PCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCLKSEL to value 0x10"]
impl crate::Resettable for PCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
