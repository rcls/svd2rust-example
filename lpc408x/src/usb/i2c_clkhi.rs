#[doc = "Register `I2C_CLKHI` reader"]
pub struct R(crate::R<I2C_CLKHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CLKHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CLKHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CLKHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CLKHI` writer"]
pub struct W(crate::W<I2C_CLKHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CLKHI_SPEC>;
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
impl From<crate::W<I2C_CLKHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CLKHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDHI` reader - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub struct CDHI_R(crate::FieldReader<u8, u8>);
impl CDHI_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDHI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDHI` writer - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub struct CDHI_W<'a> {
    w: &'a mut W,
}
impl<'a> CDHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&self) -> CDHI_R {
        CDHI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&mut self) -> CDHI_W {
        CDHI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Clock High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clkhi](index.html) module"]
pub struct I2C_CLKHI_SPEC;
impl crate::RegisterSpec for I2C_CLKHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_clkhi::R](R) reader structure"]
impl crate::Readable for I2C_CLKHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_clkhi::W](W) writer structure"]
impl crate::Writable for I2C_CLKHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CLKHI to value 0xb9"]
impl crate::Resettable for I2C_CLKHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb9
    }
}
