#[doc = "Register `RXRATE` reader"]
pub struct R(crate::R<RXRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXRATE` writer"]
pub struct W(crate::W<RXRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXRATE_SPEC>;
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
impl From<crate::W<RXRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_DIVIDER` reader - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub struct Y_DIVIDER_R(crate::FieldReader<u8, u8>);
impl Y_DIVIDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        Y_DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_DIVIDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y_DIVIDER` writer - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
pub struct Y_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `X_DIVIDER` reader - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub struct X_DIVIDER_R(crate::FieldReader<u8, u8>);
impl X_DIVIDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        X_DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X_DIVIDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X_DIVIDER` writer - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
pub struct X_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> X_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&self) -> Y_DIVIDER_R {
        Y_DIVIDER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&self) -> X_DIVIDER_R {
        X_DIVIDER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&mut self) -> Y_DIVIDER_W {
        Y_DIVIDER_W { w: self }
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&mut self) -> X_DIVIDER_W {
        X_DIVIDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxrate](index.html) module"]
pub struct RXRATE_SPEC;
impl crate::RegisterSpec for RXRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxrate::R](R) reader structure"]
impl crate::Readable for RXRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxrate::W](W) writer structure"]
impl crate::Writable for RXRATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXRATE to value 0"]
impl crate::Resettable for RXRATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
