#[doc = "Register `I2C_CLKLO` writer"]
pub struct W(crate::W<I2C_CLKLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CLKLO_SPEC>;
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
impl From<crate::W<I2C_CLKLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CLKLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDLO` writer - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
pub struct CDLO_W<'a> {
    w: &'a mut W,
}
impl<'a> CDLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
    #[inline(always)]
    pub fn cdlo(&mut self) -> CDLO_W {
        CDLO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Clock Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clklo](index.html) module"]
pub struct I2C_CLKLO_SPEC;
impl crate::RegisterSpec for I2C_CLKLO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i2c_clklo::W](W) writer structure"]
impl crate::Writable for I2C_CLKLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CLKLO to value 0xb9"]
impl crate::Resettable for I2C_CLKLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb9
    }
}
